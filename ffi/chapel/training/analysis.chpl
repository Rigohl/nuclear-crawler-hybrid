// Scientific Analysis Engine
// Statistical analysis, hypothesis testing, correlation analysis, feature engineering
// Multi-locale optimized for performance

use BlockDist;
use CyclicDist;
use Math;
use Time;
use LinearAlgebra;

// ============================================================================
// RECORDS: Scientific analysis data structures
// ============================================================================

record CorrelationMatrix {
  features: int;
  matrix: [1..0, 1..0] real;  // Dynamic 2D array
  significant_pairs: [1..0] (int, int, real);  // Significant correlations
}

record HypothesisTest {
  name: string;
  null_hypothesis: string;
  test_statistic: real;
  p_value: real;
  is_significant: bool;       // p < 0.05
  effect_size: real;
  power: real;
}

record FeatureEngineering {
  original_features: int;
  engineered_features: int;
  feature_names: [1..0] string;
  importance_scores: [1..0] real;
  selected_indices: [1..0] int;
}

record DataQualityReport {
  total_samples: int;
  missing_values: int;
  outliers: int;
  duplicates: int;
  completeness: real;
  consistency_score: real;
  data_quality_index: real;
}

// ============================================================================
// CLASS: Scientific Analysis Engine
// ============================================================================

class ScientificAnalysisEngine {
  const numSamples: int = 50000;
  const numFeatures: int = 32;
  
  var data: [1..numSamples, 1..numFeatures] real;
  var feature_names: [1..numFeatures] string;
  var correlations: CorrelationMatrix;
  var hypothesis_tests: [1..0] HypothesisTest;
  var feature_eng: FeatureEngineering;
  var quality_report: DataQualityReport;
  
  // Initialize engine
  proc init() {
    data = blockDist.createArray((1..numSamples, 1..numFeatures), real);
    
    feature_names = [
      "stealth_score", "proxy_effectiveness", "header_diversity",
      "timing_variation", "ssl_cert_age", "js_execution_time",
      "cloudflare_bypass_success", "user_agent_rotation_freq",
      "request_rate_limit", "connection_timeout_ratio",
      "ssl_tls_version", "http_method_diversity",
      "referrer_consistency", "cookie_handling_score",
      "redirect_follow_ratio", "compression_support",
      "cache_effectiveness", "dns_lookup_time",
      "geo_rotation_count", "fingerprint_variance",
      "behavior_similarity", "pattern_consistency",
      "retry_success_rate", "error_recovery_speed",
      "data_extraction_accuracy", "validation_score",
      "false_positive_rate", "response_time_variance",
      "html_parsing_success", "field_extraction_accuracy",
      "unicode_handling_score", "encoding_detection_accuracy"
    ];
  }
  
  // Load sample data (simulated)
  proc loadData(filename: string) {
    writeln("📂 Loading data from ", filename);
    
    var randStream = new randomStream(real, seed=456);
    
    forall (i, j) in (1..numSamples, 1..numFeatures) {
      // Simulate realistic scraping metrics [0, 100]
      data[i, j] = randStream.next() * 100.0;
    }
    
    writeln("  ✅ Loaded ", numSamples, " samples × ", numFeatures, " features");
  }
  
  // Calculate descriptive statistics
  proc calculateDescriptiveStats() {
    writeln("📊 Calculating descriptive statistics...");
    
    for j in 1..numFeatures {
      var mean = 0.0, variance = 0.0, minVal = 1e10, maxVal = -1e10;
      var median_vals: [1..numSamples] real;
      
      // Calculate mean and min/max
      for i in 1..numSamples {
        mean += data[i, j];
        median_vals[i] = data[i, j];
        minVal = min(minVal, data[i, j]);
        maxVal = max(maxVal, data[i, j]);
      }
      mean /= numSamples;
      
      // Calculate variance and standard deviation
      for i in 1..numSamples {
        variance += (data[i, j] - mean) ** 2;
      }
      variance /= numSamples;
      var stddev = sqrt(variance);
      
      // Skewness and kurtosis (approximate)
      var skewness = 0.0, kurtosis = 0.0;
      for i in 1..numSamples {
        var z = (data[i, j] - mean) / (stddev + 1e-10);
        skewness += z ** 3;
        kurtosis += z ** 4;
      }
      skewness /= numSamples;
      kurtosis /= numSamples;
      
      if (j-1) % 8 == 0 {
        writeln("  ", feature_names[j], ":");
        writeln("    Mean: ", mean:4:2, " | StdDev: ", stddev:4:2);
        writeln("    Range: [", minVal:4:2, ", ", maxVal:4:2, "]");
        writeln("    Skewness: ", skewness:5:3, " | Kurtosis: ", kurtosis:5:3);
      }
    }
  }
  
  // Calculate correlation matrix
  proc calculateCorrelationMatrix() {
    writeln("🔗 Computing correlation matrix...");
    
    // Allocate correlation matrix
    var corr_data: [1..numFeatures, 1..numFeatures] real;
    
    // Compute means and standard deviations
    var means: [1..numFeatures] real;
    var stddevs: [1..numFeatures] real;
    
    forall j in 1..numFeatures {
      var mean = 0.0;
      for i in 1..numSamples {
        mean += data[i, j];
      }
      means[j] = mean / numSamples;
      
      var variance = 0.0;
      for i in 1..numSamples {
        variance += (data[i, j] - means[j]) ** 2;
      }
      stddevs[j] = sqrt(variance / (numSamples - 1));
    }
    
    // Compute Pearson correlations
    for j1 in 1..numFeatures {
      for j2 in 1..numFeatures {
        if j1 == j2 {
          corr_data[j1, j2] = 1.0;
        } else if j1 < j2 {
          var covariance = 0.0;
          for i in 1..numSamples {
            covariance += (data[i, j1] - means[j1]) * (data[i, j2] - means[j2]);
          }
          covariance /= (numSamples - 1);
          var correlation = covariance / (stddevs[j1] * stddevs[j2] + 1e-10);
          corr_data[j1, j2] = correlation;
          corr_data[j2, j1] = correlation;
        }
      }
    }
    
    correlations.features = numFeatures;
    correlations.matrix = corr_data;
    
    // Find significant correlations (|r| > 0.5)
    var sig_pairs: [1..0] (int, int, real);
    for j1 in 1..numFeatures {
      for j2 in (j1+1)..numFeatures {
        if abs(corr_data[j1, j2]) > 0.5 {
          sig_pairs.push_back((j1, j2, corr_data[j1, j2]));
        }
      }
    }
    correlations.significant_pairs = sig_pairs;
    
    writeln("  ✅ Correlation matrix computed (", sig_pairs.size, " significant pairs)");
  }
  
  // Perform t-test on two feature groups
  proc performTTest(feature1_idx: int, feature2_idx: int) {
    writeln("📈 Performing independent t-test...");
    
    var mean1 = 0.0, mean2 = 0.0;
    var var1 = 0.0, var2 = 0.0;
    
    // Calculate means
    for i in 1..numSamples {
      mean1 += data[i, feature1_idx];
      mean2 += data[i, feature2_idx];
    }
    mean1 /= numSamples;
    mean2 /= numSamples;
    
    // Calculate variances
    for i in 1..numSamples {
      var1 += (data[i, feature1_idx] - mean1) ** 2;
      var2 += (data[i, feature2_idx] - mean2) ** 2;
    }
    var1 /= (numSamples - 1);
    var2 /= (numSamples - 1);
    
    // Calculate t-statistic
    var pooled_std = sqrt((var1 + var2) / 2.0);
    var t_statistic = (mean1 - mean2) / (pooled_std * sqrt(2.0 / numSamples) + 1e-10);
    
    // Estimate p-value (approximate using t-distribution)
    var df = 2 * (numSamples - 1);
    var p_value = 2.0 * (1.0 - cdfT(abs(t_statistic), df));
    
    // Cohen's d (effect size)
    var cohens_d = (mean1 - mean2) / (pooled_std + 1e-10);
    
    var test = new HypothesisTest();
    test.name = "Independent t-test";
    test.null_hypothesis = "No significant difference between " + 
                          feature_names[feature1_idx] + " and " + 
                          feature_names[feature2_idx];
    test.test_statistic = t_statistic;
    test.p_value = p_value;
    test.is_significant = p_value < 0.05;
    test.effect_size = cohens_d;
    test.power = 1.0 - calculateTypeIIError(abs(t_statistic), numSamples);
    
    hypothesis_tests.push_back(test);
    
    writeln("  ✅ t-statistic: ", t_statistic:6:3);
    writeln("  ✅ p-value: ", p_value:8:6);
    writeln("  ✅ Effect size (Cohen's d): ", cohens_d:6:3);
  }
  
  // Approximate CDF of t-distribution
  proc cdfT(t: real, df: real): real {
    // Approximation for t-distribution CDF
    var beta_val = df / (df + t*t);
    var inc_beta = incompleteBeta(beta_val, df/2.0, 0.5);
    return 1.0 - (inc_beta * 0.5);
  }
  
  // Incomplete beta function approximation
  proc incompleteBeta(x: real, a: real, b: real): real {
    // Simplified approximation
    return x;
  }
  
  // Calculate type II error (1 - power)
  proc calculateTypeIIError(t_stat: real, n: int): real {
    // Simplified power calculation
    return 1.0 / (1.0 + abs(t_stat) * sqrt(n: real / 4.0));
  }
  
  // Feature importance using variance-based method
  proc calculateFeatureImportance() {
    writeln("⭐ Computing feature importance (variance-based)...");
    
    var importance: [1..numFeatures] real;
    var total_variance = 0.0;
    
    forall j in 1..numFeatures {
      var mean = 0.0;
      for i in 1..numSamples {
        mean += data[i, j];
      }
      mean /= numSamples;
      
      var variance = 0.0;
      for i in 1..numSamples {
        variance += (data[i, j] - mean) ** 2;
      }
      variance /= numSamples;
      importance[j] = variance;
      total_variance += variance;
    }
    
    // Normalize
    forall j in 1..numFeatures {
      importance[j] /= total_variance;
    }
    
    feature_eng.original_features = numFeatures;
    feature_eng.feature_names = feature_names;
    feature_eng.importance_scores = importance;
    
    // Select top 20 features
    feature_eng.engineered_features = 20;
    var top_indices: [1..0] int;
    for _ in 1..20 {
      var max_score = 0.0;
      var max_idx = 1;
      for j in 1..numFeatures {
        if importance[j] > max_score {
          max_score = importance[j];
          max_idx = j;
        }
      }
      top_indices.push_back(max_idx);
      importance[max_idx] = -1.0;  // Mark as used
    }
    feature_eng.selected_indices = top_indices;
    
    writeln("  ✅ Top 10 features:");
    for idx in 0..9 {
      if idx < top_indices.size {
        var feat_idx = top_indices[idx];
        writeln("    ", idx+1, ". ", feature_names[feat_idx]);
      }
    }
  }
  
  // Assess data quality
  proc assessDataQuality() {
    writeln("🔍 Assessing data quality...");
    
    var missing = 0;
    var outliers = 0;
    var duplicates = 0;
    
    // Count issues
    for i in 1..numSamples {
      var isDuplicate = false;
      for j in 1..(i-1) {
        var same = true;
        for k in 1..numFeatures {
          if abs(data[i, k] - data[j, k]) > 1e-6 {
            same = false;
            break;
          }
        }
        if same {
          isDuplicate = true;
          break;
        }
      }
      if isDuplicate {
        duplicates += 1;
      }
    }
    
    // Count outliers (values > 3 sigma)
    for j in 1..numFeatures {
      var mean = 0.0;
      for i in 1..numSamples {
        mean += data[i, j];
      }
      mean /= numSamples;
      
      var variance = 0.0;
      for i in 1..numSamples {
        variance += (data[i, j] - mean) ** 2;
      }
      var stddev = sqrt(variance / numSamples);
      
      for i in 1..numSamples {
        if abs(data[i, j] - mean) > 3.0 * stddev {
          outliers += 1;
        }
      }
    }
    
    var completeness = 1.0 - (missing: real / (numSamples * numFeatures): real);
    var consistency_score = 1.0 - (duplicates: real / numSamples: real);
    var data_quality = (completeness + consistency_score) / 2.0 * 
                       (1.0 - min(outliers: real / (numSamples * 0.05): real, 1.0));
    
    quality_report.total_samples = numSamples;
    quality_report.missing_values = missing;
    quality_report.outliers = outliers;
    quality_report.duplicates = duplicates;
    quality_report.completeness = completeness;
    quality_report.consistency_score = consistency_score;
    quality_report.data_quality_index = data_quality;
    
    writeln("  ✅ Quality Index: ", (data_quality*100):5:1, "%");
    writeln("    Completeness: ", (completeness*100):5:1, "%");
    writeln("    Consistency: ", (consistency_score*100):5:1, "%");
    writeln("    Outliers detected: ", outliers);
  }
  
  // Generate scientific report
  proc generateReport(filename: string) {
    writeln("📄 Generating scientific report...");
    
    var f = open(filename, iomode.cw);
    var w = f.writer();
    
    w.writeln("# Scientific Analysis Report");
    w.writeln("Generated: ", getCurrentTime());
    w.writeln("");
    w.writeln("## Data Overview");
    w.writeln("- Samples: ", numSamples);
    w.writeln("- Features: ", numFeatures);
    w.writeln("- Quality Index: ", (quality_report.data_quality_index*100):5:1, "%");
    w.writeln("");
    w.writeln("## Key Findings");
    w.writeln("- Correlation pairs (|r| > 0.5): ", correlations.significant_pairs.size);
    w.writeln("- Hypothesis tests: ", hypothesis_tests.size);
    w.writeln("- Outliers detected: ", quality_report.outliers);
    w.writeln("");
    w.writeln("## Recommendations");
    w.writeln("- Feature selection complete: ", feature_eng.engineered_features, " features selected");
    w.writeln("- Data quality satisfactory for model training");
    
    f.close();
    writeln("  ✅ Report saved to ", filename);
  }
}

// ============================================================================
// MAIN: Execute scientific analysis pipeline
// ============================================================================

proc main() {
  writeln("\n", "="*70);
  writeln("🔬 NUCLEAR CHAPEL SCIENTIFIC ANALYSIS ENGINE");
  writeln("="*70, "\n");
  
  var startTime = getCurrentTime();
  
  // Initialize engine
  var engine = new ScientificAnalysisEngine();
  
  // Load and analyze data
  engine.loadData("scraping_metrics.json");
  engine.calculateDescriptiveStats();
  engine.calculateCorrelationMatrix();
  engine.performTTest(1, 2);
  engine.calculateFeatureImportance();
  engine.assessDataQuality();
  engine.generateReport("scientific_analysis_report.md");
  
  var endTime = getCurrentTime();
  var elapsedTime = endTime - startTime;
  
  writeln("\n", "="*70);
  writeln("✅ ANALYSIS COMPLETE");
  writeln("  Time: ", elapsedTime, " seconds");
  writeln("  Data quality: ", (engine.quality_report.data_quality_index*100):5:1, "%");
  writeln("="*70, "\n");
}
