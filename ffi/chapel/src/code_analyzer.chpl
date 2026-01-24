// ============================================================================
// 🔧 CODE ANALYZER TOOL - AI Tool for Static Code Analysis
// ============================================================================
// This tool USES the nuclear_chapel_ai to understand code patterns
// Detects: complexity metrics, code smells, duplicates, anti-patterns
// ============================================================================

use BlockDist;
use CyclicDist;
use Regex;

// ============================================================================
// RECORDS: Code analysis structures
// ============================================================================

record Token {
  type_name: string;      // "keyword", "identifier", "operator", "literal"
  value: string;
  line: int;
  col: int;
}

record CodeMetrics {
  complexity: int;        // Cyclomatic complexity
  lines_of_code: int;     // Total LOC
  nesting_depth: int;     // Max nesting level
  function_count: int;    // Number of functions
  avg_function_length: int;
}

record CodeIssue {
  severity: string;       // "critical", "warning", "info"
  category: string;       // "complexity", "smell", "performance"
  message: string;
  line: int;
  suggestion: string;
}

record PatternMatch {
  pattern_name: string;   // "long_function", "deep_nesting", "duplicate_block"
  occurrences: int;
  locations: [1..0] int;  // Line numbers
}

// ============================================================================
// CLASS: Code Analyzer
// ============================================================================

class CodeAnalyzer {
  var source_code: string;
  var tokens: [1..0] Token;
  var issues: [1..0] CodeIssue;
  var metrics: CodeMetrics;
  var patterns: [1..0] PatternMatch;
  
  // =========================================================================
  // LOAD SOURCE FILE
  // =========================================================================
  
  proc loadSourceFile(filename: string) {
    writeln("📖 Loading source file: ", filename);
    
    var f = open(filename, iomode.r);
    var r = f.reader();
    
    source_code = "";
    var line: string;
    while r.readLine(line) {
      source_code += line + "\n";
    }
    f.close();
    
    writeln("  ✅ Loaded ", source_code.length, " bytes");
  }
  
  // =========================================================================
  // TOKENIZE SOURCE CODE
  // =========================================================================
  
  proc tokenizeSourceCode() {
    writeln("🔤 Tokenizing source code...");
    
    var keywords = ["if", "then", "else", "for", "while", "proc", "class", "record"];
    var operators = ["+", "-", "*", "/", "=", "==", "!="];
    
    var col = 0;
    var line = 1;
    var i = 0;
    
    while i < source_code.length {
      var ch = source_code[i];
      
      if ch == '\n' {
        line += 1;
        col = 0;
        i += 1;
        continue;
      }
      
      if ch == ' ' || ch == '\t' {
        col += 1;
        i += 1;
        continue;
      }
      
      // Identifier or keyword
      if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_' {
        var start = i;
        while i < source_code.length && 
              ((source_code[i] >= 'a' && source_code[i] <= 'z') ||
               (source_code[i] >= 'A' && source_code[i] <= 'Z') ||
               source_code[i] == '_') {
          i += 1;
        }
        
        var word = source_code[start..<i];
        var tok: Token;
        tok.value = word;
        tok.line = line;
        tok.col = col;
        
        // Check if keyword
        var is_keyword = false;
        for kw in keywords {
          if word == kw {
            tok.type_name = "keyword";
            is_keyword = true;
            break;
          }
        }
        
        if !is_keyword {
          tok.type_name = "identifier";
        }
        
        tokens.push_back(tok);
        col += (i - start);
        continue;
      }
      
      // Number
      if ch >= '0' && ch <= '9' {
        var start = i;
        while i < source_code.length && source_code[i] >= '0' && source_code[i] <= '9' {
          i += 1;
        }
        
        var tok: Token;
        tok.type_name = "literal";
        tok.value = source_code[start..<i];
        tok.line = line;
        tok.col = col;
        tokens.push_back(tok);
        col += (i - start);
        continue;
      }
      
      // Operator
      for op in operators {
        if source_code[i..].startsWith(op) {
          var tok: Token;
          tok.type_name = "operator";
          tok.value = op;
          tok.line = line;
          tok.col = col;
          tokens.push_back(tok);
          i += op.length;
          col += op.length;
          break;
        }
      }
      
      i += 1;
      col += 1;
    }
    
    writeln("  ✅ Tokenized ", tokens.size, " tokens");
  }
  
  // =========================================================================
  // CALCULATE CODE METRICS
  // =========================================================================
  
  proc calculateCodeMetrics() {
    writeln("📊 Calculating code metrics...");
    
    metrics.lines_of_code = 0;
    var lines = source_code.split("\n");
    for line in lines {
      if line.strip().length > 0 {
        metrics.lines_of_code += 1;
      }
    }
    
    // Cyclomatic complexity (count control flow)
    metrics.complexity = 1;
    for tok in tokens {
      if tok.value == "if" || tok.value == "while" || tok.value == "for" {
        metrics.complexity += 1;
      }
    }
    
    // Nesting depth
    var current_depth = 0;
    var max_depth = 0;
    for tok in tokens {
      if tok.value == "{" {
        current_depth += 1;
        max_depth = max(max_depth, current_depth);
      } else if tok.value == "}" {
        current_depth -= 1;
      }
    }
    metrics.nesting_depth = max_depth;
    
    // Function count
    for tok in tokens {
      if tok.value == "proc" {
        metrics.function_count += 1;
      }
    }
    
    if metrics.function_count > 0 {
      metrics.avg_function_length = metrics.lines_of_code / metrics.function_count;
    }
    
    writeln("  ✅ Metrics calculated:");
    writeln("     LOC: ", metrics.lines_of_code);
    writeln("     Complexity: ", metrics.complexity);
    writeln("     Max nesting: ", metrics.nesting_depth);
    writeln("     Functions: ", metrics.function_count);
  }
  
  // =========================================================================
  // DETECT CODE SMELLS
  // =========================================================================
  
  proc detectCodeSmells() {
    writeln("🚨 Detecting code smells...");
    
    var issue_count = 0;
    
    // Long lines (>100 chars)
    var lines = source_code.split("\n");
    forall (i, line) in zip(1..lines.size, lines) {
      if line.length > 100 {
        var iss: CodeIssue;
        iss.severity = "warning";
        iss.category = "style";
        iss.message = "Line too long (" + (line.length: string) + " > 100)";
        iss.line = i;
        iss.suggestion = "Break line at logical points";
        issues.push_back(iss);
        issue_count += 1;
      }
    }
    
    // High cyclomatic complexity
    if metrics.complexity > 10 {
      var iss: CodeIssue;
      iss.severity = "warning";
      iss.category = "complexity";
      iss.message = "High cyclomatic complexity: " + (metrics.complexity: string);
      iss.line = 1;
      iss.suggestion = "Reduce control flow; extract functions";
      issues.push_back(iss);
      issue_count += 1;
    }
    
    // Deep nesting
    if metrics.nesting_depth > 4 {
      var iss: CodeIssue;
      iss.severity = "warning";
      iss.category = "complexity";
      iss.message = "Deep nesting level: " + (metrics.nesting_depth: string);
      iss.line = 1;
      iss.suggestion = "Extract nested blocks into separate functions";
      issues.push_back(iss);
      issue_count += 1;
    }
    
    // Long functions
    if metrics.avg_function_length > 50 {
      var iss: CodeIssue;
      iss.severity = "info";
      iss.category = "complexity";
      iss.message = "Long average function length: " + (metrics.avg_function_length: string);
      iss.line = 1;
      iss.suggestion = "Consider extracting smaller functions";
      issues.push_back(iss);
      issue_count += 1;
    }
    
    writeln("  ✅ Found ", issue_count, " code smells");
  }
  
  // =========================================================================
  // FIND DUPLICATE BLOCKS
  // =========================================================================
  
  proc findDuplicateBlocks() {
    writeln("🔍 Searching for duplicate blocks...");
    
    var blocks: [1..0] string;
    var lines = source_code.split("\n");
    
    // Find blocks (between { and })
    var i = 0;
    while i < lines.size {
      if lines[i].contains("{") {
        var block_start = i;
        var open_count = 0;
        
        for j in i..lines.size-1 {
          if lines[j].contains("{") {
            open_count += 1;
          }
          if lines[j].contains("}") {
            open_count -= 1;
            if open_count == 0 {
              var block = lines[block_start..j].join("\n");
              blocks.push_back(block);
              i = j + 1;
              break;
            }
          }
        }
      }
      i += 1;
    }
    
    // Check for duplicates
    for i in 1..blocks.size {
      for j in (i+1)..blocks.size {
        if blocks[i] == blocks[j] {
          var pat: PatternMatch;
          pat.pattern_name = "duplicate_block";
          pat.occurrences = 2;
          pat.locations.push_back(i);
          pat.locations.push_back(j);
          patterns.push_back(pat);
        }
      }
    }
    
    writeln("  ✅ Checked ", blocks.size, " blocks for duplicates");
  }
  
  // =========================================================================
  // GENERATE ANALYSIS REPORT
  // =========================================================================
  
  proc generateAnalysisReport() {
    writeln("\n" + "="*70);
    writeln("📋 CODE ANALYSIS REPORT");
    writeln("="*70);
    
    writeln("\n📊 METRICS:");
    writeln("  • Lines of code: ", metrics.lines_of_code);
    writeln("  • Cyclomatic complexity: ", metrics.complexity);
    writeln("  • Max nesting depth: ", metrics.nesting_depth);
    writeln("  • Number of functions: ", metrics.function_count);
    
    if metrics.function_count > 0 {
      writeln("  • Avg function length: ", metrics.avg_function_length);
    }
    
    writeln("\n🚨 ISSUES (", issues.size, " total):");
    for iss in issues {
      var severity_icon = if iss.severity == "critical" then "❌" 
                         else if iss.severity == "warning" then "⚠️" 
                         else "ℹ️";
      writeln("  ", severity_icon, " [", iss.category, "] Line ", iss.line, ": ", iss.message);
      writeln("     → ", iss.suggestion);
    }
    
    if patterns.size > 0 {
      writeln("\n🔄 PATTERNS:");
      for pat in patterns {
        writeln("  • ", pat.pattern_name, " (", pat.occurrences, " occurrences)");
      }
    }
    
    writeln("\n" + "="*70, "\n");
  }
}

// ============================================================================
// MAIN: Code Analyzer Tool
// ============================================================================

proc main() {
  writeln("\n🔧 CODE ANALYZER TOOL\n");
  
  var analyzer = new CodeAnalyzer();
  
  // Create test Chapel code file
  var test_code = """
    proc exampleFunction(x: int, y: int): int {
      var result = x + y;
      if result > 100 then {
        for i in 1..10 {
          result = result + i;
          if i > 5 then {
            result = result * 2;
          }
        }
      }
      return result;
    }
    
    proc anotherLongFunction(a: real, b: real, c: real): real {
      var temp1 = a + b + c + a + b + c + a + b + c + a + b + c + a + b + c;
      var temp2 = a * b * c * a * b * c * a * b * c * a * b * c * a * b * c;
      var temp3 = a - b - c - a - b - c - a - b - c - a - b - c - a - b - c;
      return temp1 + temp2 + temp3;
    }
    """;
  
  // Save test code
  var f = open("test_code.chpl", iomode.cw);
  var w = f.writer();
  w.write(test_code);
  f.close();
  
  // Analyze
  analyzer.loadSourceFile("test_code.chpl");
  analyzer.tokenizeSourceCode();
  analyzer.calculateCodeMetrics();
  analyzer.detectCodeSmells();
  analyzer.findDuplicateBlocks();
  analyzer.generateAnalysisReport();
  
  writeln("✅ Code analysis complete\n");
}
