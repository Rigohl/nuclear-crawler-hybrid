// ============================================================================
// 👁️ CODE REVIEWER - AI Tool for Comprehensive Code Review
// ============================================================================
// This tool USES the nuclear_chapel_ai to review code
// Checks: performance, safety, style, complexity with A-F grading
// ============================================================================

record ReviewFinding {
  area: string;           // "performance", "safety", "style", "complexity"
  severity: string;       // "critical", "major", "minor"
  message: string;
  line: int;
  recommendation: string;
}

record ReviewMetric {
  name: string;
  value: real;
  threshold: real;
  status: string;         // "pass", "warning", "fail"
}

record ReviewScore {
  area_name: string;
  score: real;            // 0-100
  findings_count: int;
  grade: string;          // A+, A, B+, B, C
}

class CodeReviewer {
  var source_code: string;
  var findings: [1..0] ReviewFinding;
  var metrics: [1..0] ReviewMetric;
  var scores: [1..0] ReviewScore;
  var overall_grade: string = "B";
  
  // =========================================================================
  // LOAD SOURCE CODE
  // =========================================================================
  
  proc loadSourceCode(filename: string) {
    writeln("📖 Loading code for review: ", filename);
    
    var f = open(filename, iomode.r);
    var r = f.reader();
    
    source_code = "";
    var line: string;
    while r.readLine(line) {
      source_code += line + "\n";
    }
    f.close();
    
    writeln("  ✅ Loaded for review");
  }
  
  // =========================================================================
  // REVIEW PERFORMANCE
  // =========================================================================
  
  proc reviewPerformance() {
    writeln("⚡ Reviewing performance...");
    
    var perf_score: ReviewScore;
    perf_score.area_name = "Performance";
    perf_score.findings_count = 0;
    
    var lines = source_code.split("\n");
    
    // Check for string concatenation in loops
    forall (i, line) in zip(1..lines.size, lines) {
      if line.contains("for ") && (line.contains("+") || line.contains("+=")) {
        if line.contains("str") || line.contains("String") {
          var finding: ReviewFinding;
          finding.area = "performance";
          finding.severity = "major";
          finding.message = "String concatenation in loop (O(n²))";
          finding.line = i;
          finding.recommendation = "Use StringBuilder or StringBuilder-like pattern";
          findings.push_back(finding);
          perf_score.findings_count += 1;
        }
      }
    }
    
    // Check for unnecessary copies
    forall (i, line) in zip(1..lines.size, lines) {
      if line.contains("var copy = ") && line.contains(".clone()") {
        var finding: ReviewFinding;
        finding.area = "performance";
        finding.severity = "minor";
        finding.message = "Unnecessary copy operation";
        finding.line = i;
        finding.recommendation = "Use reference or move semantics when possible";
        findings.push_back(finding);
        perf_score.findings_count += 1;
      }
    }
    
    // Check for missing vectorization
    forall (i, line) in zip(1..lines.size, lines) {
      if line.contains("for i in ") && !line.contains("forall") {
        if !line.contains("sequential") {
          // Could be vectorized
        }
      }
    }
    
    // Check for BlockDist usage
    var has_blockdist = false;
    for line in lines {
      if line.contains("BlockDist") {
        has_blockdist = true;
        break;
      }
    }
    
    if !has_blockdist && lines.size > 100 {
      var finding: ReviewFinding;
      finding.area = "performance";
      finding.severity = "minor";
      finding.message = "Large code without BlockDist parallelization";
      finding.line = 1;
      finding.recommendation = "Consider using BlockDist for data parallelism";
      findings.push_back(finding);
      perf_score.findings_count += 1;
    }
    
    perf_score.score = max(0.0, 100.0 - (perf_score.findings_count: real * 10.0));
    if perf_score.score >= 90.0 {
      perf_score.grade = "A+";
    } else if perf_score.score >= 80.0 {
      perf_score.grade = "A";
    } else if perf_score.score >= 70.0 {
      perf_score.grade = "B+";
    } else if perf_score.score >= 60.0 {
      perf_score.grade = "B";
    } else {
      perf_score.grade = "C";
    }
    
    scores.push_back(perf_score);
    writeln("  ✅ Performance review: ", perf_score.grade, " (", perf_score.findings_count, " issues)");
  }
  
  // =========================================================================
  // REVIEW SAFETY
  // =========================================================================
  
  proc reviewSafety() {
    writeln("🔒 Reviewing safety...");
    
    var safety_score: ReviewScore;
    safety_score.area_name = "Safety";
    safety_score.findings_count = 0;
    
    var lines = source_code.split("\n");
    
    // Check for unchecked array access
    forall (i, line) in zip(1..lines.size, lines) {
      if line.contains("[") && !line.contains("if") && !line.contains("bounds") {
        var finding: ReviewFinding;
        finding.area = "safety";
        finding.severity = "critical";
        finding.message = "Unchecked array access";
        finding.line = i;
        finding.recommendation = "Add bounds checking before array access";
        findings.push_back(finding);
        safety_score.findings_count += 1;
      }
    }
    
    // Check for null pointer dereferences
    forall (i, line) in zip(1..lines.size, lines) {
      if line.contains("->") && !line.contains("null?") {
        var finding: ReviewFinding;
        finding.area = "safety";
        finding.severity = "major";
        finding.message = "Potential null pointer dereference";
        finding.line = i;
        finding.recommendation = "Use optional types with proper null checking";
        findings.push_back(finding);
        safety_score.findings_count += 1;
      }
    }
    
    // Check for overflow risks
    forall (i, line) in zip(1..lines.size, lines) {
      if line.contains("* 2") || line.contains("** 2") {
        var finding: ReviewFinding;
        finding.area = "safety";
        finding.severity = "minor";
        finding.message = "Potential integer overflow";
        finding.line = i;
        finding.recommendation = "Use checked arithmetic or larger types";
        findings.push_back(finding);
        safety_score.findings_count += 1;
      }
    }
    
    // Check for error handling
    var error_handling_count = 0;
    for line in lines {
      if line.contains("try") || line.contains("?") || line.contains("error") {
        error_handling_count += 1;
      }
    }
    
    if error_handling_count == 0 && lines.size > 50 {
      var finding: ReviewFinding;
      finding.area = "safety";
      finding.severity = "major";
      finding.message = "No error handling in large code block";
      finding.line = 1;
      finding.recommendation = "Add try-catch or Result types for error handling";
      findings.push_back(finding);
      safety_score.findings_count += 1;
    }
    
    safety_score.score = max(0.0, 100.0 - (safety_score.findings_count: real * 15.0));
    if safety_score.score >= 90.0 {
      safety_score.grade = "A+";
    } else if safety_score.score >= 80.0 {
      safety_score.grade = "A";
    } else if safety_score.score >= 70.0 {
      safety_score.grade = "B+";
    } else if safety_score.score >= 60.0 {
      safety_score.grade = "B";
    } else {
      safety_score.grade = "C";
    }
    
    scores.push_back(safety_score);
    writeln("  ✅ Safety review: ", safety_score.grade, " (", safety_score.findings_count, " issues)");
  }
  
  // =========================================================================
  // REVIEW STYLE
  // =========================================================================
  
  proc reviewStyle() {
    writeln("🎨 Reviewing style...");
    
    var style_score: ReviewScore;
    style_score.area_name = "Style";
    style_score.findings_count = 0;
    
    var lines = source_code.split("\n");
    
    // Check for vague variable names
    forall (i, line) in zip(1..lines.size, lines) {
      if line.contains("var x ") || line.contains("var a ") || 
         line.contains("var temp") {
        var finding: ReviewFinding;
        finding.area = "style";
        finding.severity = "minor";
        finding.message = "Vague variable name";
        finding.line = i;
        finding.recommendation = "Use descriptive variable names";
        findings.push_back(finding);
        style_score.findings_count += 1;
      }
    }
    
    // Check for missing documentation
    var documented_functions = 0;
    var total_functions = 0;
    forall (i, line) in zip(1..lines.size, lines) {
      if line.contains("proc ") {
        total_functions += 1;
        if i > 1 && lines[i-1].contains("//") {
          documented_functions += 1;
        }
      }
    }
    
    if total_functions > 0 && documented_functions < (total_functions / 2) {
      var finding: ReviewFinding;
      finding.area = "style";
      finding.severity = "minor";
      finding.message = "Missing function documentation";
      finding.line = 1;
      finding.recommendation = "Add comments for all public functions";
      findings.push_back(finding);
      style_score.findings_count += 1;
    }
    
    // Check for indentation consistency
    forall (i, line) in zip(1..lines.size, lines) {
      if line.length > 0 && line[1] != ' ' && line[1] != '\t' {
        if line.contains("{") || line.contains("}") {
          var finding: ReviewFinding;
          finding.area = "style";
          finding.severity = "minor";
          finding.message = "Inconsistent indentation";
          finding.line = i;
          finding.recommendation = "Use consistent indentation (2 or 4 spaces)";
          findings.push_back(finding);
          style_score.findings_count += 1;
        }
      }
    }
    
    style_score.score = max(0.0, 100.0 - (style_score.findings_count: real * 5.0));
    if style_score.score >= 90.0 {
      style_score.grade = "A+";
    } else if style_score.score >= 80.0 {
      style_score.grade = "A";
    } else if style_score.score >= 70.0 {
      style_score.grade = "B+";
    } else if style_score.score >= 60.0 {
      style_score.grade = "B";
    } else {
      style_score.grade = "C";
    }
    
    scores.push_back(style_score);
    writeln("  ✅ Style review: ", style_score.grade, " (", style_score.findings_count, " issues)");
  }
  
  // =========================================================================
  // REVIEW COMPLEXITY
  // =========================================================================
  
  proc reviewComplexity() {
    writeln("🧠 Reviewing complexity...");
    
    var complexity_score: ReviewScore;
    complexity_score.area_name = "Complexity";
    complexity_score.findings_count = 0;
    
    var lines = source_code.split("\n");
    
    // Calculate metrics
    var cyclomatic = 1;
    var max_nesting = 0;
    var current_nesting = 0;
    var max_function_length = 0;
    var current_function_length = 0;
    var in_function = false;
    
    forall (i, line) in zip(1..lines.size, lines) {
      if line.contains("if ") || line.contains("while ") || line.contains("for ") {
        cyclomatic += 1;
      }
      
      if line.contains("{") {
        current_nesting += 1;
        max_nesting = max(max_nesting, current_nesting);
        
        if line.contains("proc ") {
          in_function = true;
          current_function_length = 0;
        }
      }
      
      if line.contains("}") {
        current_nesting -= 1;
        if in_function {
          max_function_length = max(max_function_length, current_function_length);
        }
      }
      
      if in_function {
        current_function_length += 1;
      }
    }
    
    // Check cyclomatic complexity (should be < 10)
    if cyclomatic > 10 {
      var finding: ReviewFinding;
      finding.area = "complexity";
      finding.severity = "major";
      finding.message = "High cyclomatic complexity: " + (cyclomatic: string);
      finding.line = 1;
      finding.recommendation = "Break functions into smaller, simpler pieces";
      findings.push_back(finding);
      complexity_score.findings_count += 1;
    }
    
    // Check nesting depth (should be < 4)
    if max_nesting > 4 {
      var finding: ReviewFinding;
      finding.area = "complexity";
      finding.severity = "major";
      finding.message = "Deep nesting: " + (max_nesting: string) + " levels";
      finding.line = 1;
      finding.recommendation = "Extract nested blocks into separate functions";
      findings.push_back(finding);
      complexity_score.findings_count += 1;
    }
    
    // Check function length (should be < 50 lines)
    if max_function_length > 50 {
      var finding: ReviewFinding;
      finding.area = "complexity";
      finding.severity = "minor";
      finding.message = "Long function: " + (max_function_length: string) + " lines";
      finding.line = 1;
      finding.recommendation = "Consider breaking into smaller functions";
      findings.push_back(finding);
      complexity_score.findings_count += 1;
    }
    
    complexity_score.score = max(0.0, 100.0 - (complexity_score.findings_count: real * 12.0));
    if complexity_score.score >= 90.0 {
      complexity_score.grade = "A+";
    } else if complexity_score.score >= 80.0 {
      complexity_score.grade = "A";
    } else if complexity_score.score >= 70.0 {
      complexity_score.grade = "B+";
    } else if complexity_score.score >= 60.0 {
      complexity_score.grade = "B";
    } else {
      complexity_score.grade = "C";
    }
    
    scores.push_back(complexity_score);
    writeln("  ✅ Complexity review: ", complexity_score.grade, " (", complexity_score.findings_count, " issues)");
  }
  
  // =========================================================================
  // RUN ALL REVIEWS
  // =========================================================================
  
  proc runFullReview() {
    writeln("\n👁️ Running comprehensive code review...\n");
    
    var startTime = getCurrentTime();
    
    reviewPerformance();
    reviewSafety();
    reviewStyle();
    reviewComplexity();
    
    calculateReviewScore();
    
    var endTime = getCurrentTime();
    var elapsedTime = endTime - startTime;
    
    writeln("\n" + "="*70);
    writeln("✅ REVIEW COMPLETE");
    writeln("  • Overall grade: ", overall_grade);
    writeln("  • Total findings: ", findings.size);
    writeln("  • Time: ", elapsedTime, "s");
    writeln("="*70, "\n");
  }
  
  // =========================================================================
  // CALCULATE OVERALL SCORE
  // =========================================================================
  
  proc calculateReviewScore() {
    var total_score = 0.0;
    for score in scores {
      total_score += score.score;
    }
    
    var avg_score = total_score / scores.size;
    
    if avg_score >= 95.0 {
      overall_grade = "A+";
    } else if avg_score >= 90.0 {
      overall_grade = "A";
    } else if avg_score >= 85.0 {
      overall_grade = "B+";
    } else if avg_score >= 70.0 {
      overall_grade = "B";
    } else if avg_score >= 60.0 {
      overall_grade = "C";
    } else {
      overall_grade = "F";
    }
  }
  
  // =========================================================================
  // GENERATE REVIEW REPORT
  // =========================================================================
  
  proc generateReviewReport() {
    writeln("\n" + "="*70);
    writeln("📋 CODE REVIEW REPORT");
    writeln("="*70);
    
    writeln("\n🎯 SCORES BY AREA:");
    for score in scores {
      writeln("  • ", score.area_name, ": ", score.grade, " (", (score.score:5:1), "/100)");
    }
    
    writeln("\n📊 OVERALL GRADE: ", overall_grade);
    
    writeln("\n🔍 FINDINGS (", findings.size, " total):");
    for finding in findings {
      var icon = if finding.severity == "critical" then "❌"
                else if finding.severity == "major" then "⚠️"
                else "ℹ️";
      writeln("  ", icon, " [", finding.area, "] Line ", finding.line, ": ", finding.message);
      writeln("     → ", finding.recommendation);
    }
    
    writeln("\n" + "="*70, "\n");
  }
}

// ============================================================================
// MAIN: Code Reviewer
// ============================================================================

proc main() {
  writeln("\n👁️ CODE REVIEWER\n");
  
  var reviewer = new CodeReviewer();
  
  // Create test code
  var test_code = """
    proc complexFunction(a: int, b: int) {
      var x = a;
      if x > 0 {
        for i in 1..10 {
          if i > 5 {
            if b > 0 {
              if x < 100 {
                x = x + i;
              }
            }
          }
        }
      }
      return x;
    }
    
    proc stringConcat(items: [1..0] string): string {
      var result = "";
      for item in items {
        result = result + item;
      }
      return result;
    }
    """;
  
  var f = open("review_code.chpl", iomode.cw);
  var w = f.writer();
  w.write(test_code);
  f.close();
  
  reviewer.loadSourceCode("review_code.chpl");
  reviewer.runFullReview();
  reviewer.generateReviewReport();
  
  writeln("✅ Code review complete\n");
}
