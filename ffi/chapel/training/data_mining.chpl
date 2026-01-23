// Data Mining Engine for Web Scraping Intelligence
// Pure Chapel implementation with multi-locale parallelism
// Specialization: Pattern extraction, clustering, anomaly detection

use BlockDist;
use CyclicDist;
use Math;
use Time;
use Random;
use LinearAlgebra;

// ============================================================================
// RECORDS: Data structures for mining operations
// ============================================================================

record MinedPattern {
  id: int;
  technique: string;          // Scraping technique used
  feature_vector: [1..32] real;
  frequency: int;
  confidence: real;
  timestamp: real;
  cluster_id: int = -1;
}

record ClusterMetrics {
  cluster_id: int;
  size: int;
  centroid: [1..32] real;
  intra_distance: real;
  silhouette_score: real;
}

record AnomalyInstance {
  pattern_id: int;
  technique: string;
  anomaly_score: real;
  deviation_factor: real;
  detected_at: real;
}

record PatternStatistics {
  mean: [1..32] real;
  stddev: [1..32] real;
  min: [1..32] real;
  max: [1..32] real;
  correlation_matrix: [1..32, 1..32] real;
}

// ============================================================================
// CLASS: Data Mining Engine with K-Means Clustering
// ============================================================================

class DataMiningEngine {
  const numPatterns: int = 50000;
  const numFeatures: int = 32;
  const numClusters: int = 12;      // Scraping technique families
  const maxIterations: int = 30;
  const convergenceThreshold: real = 0.001;
  
  var patterns: [1..numPatterns] MinedPattern;
  var centroids: [1..numClusters, 1..numFeatures] real;
  var cluster_assignments: [1..numPatterns] int;
  var cluster_metrics: [1..numClusters] ClusterMetrics;
  var anomalies: [1..0] AnomalyInstance;  // Dynamic array for anomalies
  var stats: PatternStatistics;
  
  // Initialize with distributed storage
  proc init() {
    patterns = blockDist.createArray(1..numPatterns, MinedPattern);
    centroids = blockDist.createArray((1..numClusters, 1..numFeatures), real);
    cluster_assignments = blockDist.createArray(1..numPatterns, int);
  }
  
  // Extract features from scraped data (simulation with random patterns)
  proc extractPatterns(dataFile: string) {
    var randStream = new randomStream(real, 42);
    
    writeln("📊 Extracting patterns from ", dataFile);
    
    forall i in 1..numPatterns {
      var pattern = new MinedPattern();
      pattern.id = i;
      pattern.technique = ["User-Agent", "Proxy-Chain", "Headers", 
                          "Timing", "SSL", "JavaScript", "Cloudflare"][
                          ((i % 7) + 1)];
      
      // Random features [0, 1]
      forall j in 1..numFeatures {
        pattern.feature_vector[j] = randStream.next();
      }
      
      pattern.frequency = ((i % 100) + 1);
      pattern.confidence = 0.5 + (randStream.next() * 0.5);
      pattern.timestamp = getCurrentTime();
      
      patterns[i] = pattern;
    }
    
    writeln("  ✅ Extracted ", numPatterns, " patterns");
  }
  
  // Initialize K-Means centroids (K-means++ initialization)
  proc initializeCentroids() {
    var randStream = new randomStream(int, 123);
    
    writeln("🎯 Initializing K-Means centroids (K-means++)");
    
    // Pick first centroid randomly
    const firstIdx = (randStream.next() % numPatterns) + 1;
    forall j in 1..numFeatures {
      centroids[1, j] = patterns[firstIdx].feature_vector[j];
    }
    
    // K-means++ initialization for remaining centroids
    for k in 2..numClusters {
      var distances: [1..numPatterns] real;
      var maxDist = 0.0;
      var maxIdx = 1;
      
      // Calculate distance to nearest centroid
      forall i in 1..numPatterns {
        var minDist = 1e10;
        for kk in 1..(k-1) {
          var dist = 0.0;
          for j in 1..numFeatures {
            dist += (patterns[i].feature_vector[j] - centroids[kk, j]) ** 2;
          }
          dist = sqrt(dist);
          minDist = min(minDist, dist);
        }
        distances[i] = minDist;
        
        // Track farthest point
        if distances[i] > maxDist {
          maxDist = distances[i];
          maxIdx = i;
        }
      }
      
      // Set new centroid to farthest point
      forall j in 1..numFeatures {
        centroids[k, j] = patterns[maxIdx].feature_vector[j];
      }
    }
    
    writeln("  ✅ Centroids initialized (K-means++)");
  }
  
  // K-Means clustering (main algorithm)
  proc kMeansClustering() {
    writeln("🔄 Running K-Means clustering...");
    
    var prevAssignments: [1..numPatterns] int = 0;
    var converged = false;
    var iteration = 0;
    
    while !converged && iteration < maxIterations {
      iteration += 1;
      
      // Assignment step: Assign each pattern to nearest centroid
      forall i in 1..numPatterns {
        var minDist = 1e10;
        var nearestCluster = 1;
        
        for k in 1..numClusters {
          var dist = 0.0;
          for j in 1..numFeatures {
            dist += (patterns[i].feature_vector[j] - centroids[k, j]) ** 2;
          }
          dist = sqrt(dist);
          
          if dist < minDist {
            minDist = dist;
            nearestCluster = k;
          }
        }
        
        cluster_assignments[i] = nearestCluster;
      }
      
      // Update step: Recalculate centroids
      for k in 1..numClusters {
        var clusterSize = 0;
        var newCentroid: [1..numFeatures] real = 0.0;
        
        for i in 1..numPatterns {
          if cluster_assignments[i] == k {
            clusterSize += 1;
            for j in 1..numFeatures {
              newCentroid[j] += patterns[i].feature_vector[j];
            }
          }
        }
        
        if clusterSize > 0 {
          forall j in 1..numFeatures {
            centroids[k, j] = newCentroid[j] / clusterSize;
          }
        }
      }
      
      // Check convergence
      var changed = 0;
      forall i in 1..numPatterns {
        if cluster_assignments[i] != prevAssignments[i] {
          changed += 1;
        }
      }
      
      converged = (changed: real / numPatterns: real) < convergenceThreshold;
      prevAssignments = cluster_assignments;
      
      if iteration % 5 == 0 || iteration == 1 {
        writeln("  Iteration ", iteration, ": changed=", changed);
      }
    }
    
    writeln("  ✅ Clustering complete (", iteration, " iterations)");
  }
  
  // Calculate cluster metrics (size, silhouette score, etc)
  proc calculateClusterMetrics() {
    writeln("📈 Calculating cluster metrics...");
    
    for k in 1..numClusters {
      var metrics = new ClusterMetrics();
      metrics.cluster_id = k;
      metrics.size = 0;
      
      // Count patterns in cluster
      for i in 1..numPatterns {
        if cluster_assignments[i] == k {
          metrics.size += 1;
        }
      }
      
      // Calculate centroid
      metrics.centroid = centroids[k, ..];
      
      // Calculate intra-cluster distance (average distance to centroid)
      var totalDist = 0.0;
      if metrics.size > 0 {
        for i in 1..numPatterns {
          if cluster_assignments[i] == k {
            var dist = 0.0;
            for j in 1..numFeatures {
              dist += (patterns[i].feature_vector[j] - centroids[k, j]) ** 2;
            }
            totalDist += sqrt(dist);
          }
        }
        metrics.intra_distance = totalDist / metrics.size;
      }
      
      // Silhouette score (simplified)
      var silhouette = 0.0;
      var count = 0;
      for i in 1..numPatterns {
        if cluster_assignments[i] == k {
          // Intra-cluster distance
          var intra = 0.0;
          for j in 1..numFeatures {
            intra += (patterns[i].feature_vector[j] - centroids[k, j]) ** 2;
          }
          intra = sqrt(intra);
          
          // Inter-cluster distance (to nearest other cluster)
          var inter = 1e10;
          for kk in 1..numClusters {
            if kk != k {
              var dist = 0.0;
              for j in 1..numFeatures {
                dist += (patterns[i].feature_vector[j] - centroids[kk, j]) ** 2;
              }
              inter = min(inter, sqrt(dist));
            }
          }
          
          if inter > intra {
            silhouette += (inter - intra) / inter;
          }
          count += 1;
        }
      }
      
      if count > 0 {
        metrics.silhouette_score = silhouette / count;
      }
      
      cluster_metrics[k] = metrics;
    }
    
    writeln("  ✅ Metrics calculated for ", numClusters, " clusters");
  }
  
  // Detect anomalies using Z-score method
  proc detectAnomalies(threshold: real = 3.0) {
    writeln("🚨 Detecting anomalies (Z-score threshold=", threshold, ")");
    
    // Calculate statistics
    calculateStatistics();
    
    var anomalyList: [1..0] AnomalyInstance;
    
    for i in 1..numPatterns {
      var isAnomaly = false;
      var maxZScore = 0.0;
      
      for j in 1..numFeatures {
        if stats.stddev[j] > 0.0 {
          var zScore = abs((patterns[i].feature_vector[j] - stats.mean[j]) / stats.stddev[j]);
          if zScore > threshold {
            isAnomaly = true;
            maxZScore = max(maxZScore, zScore);
          }
        }
      }
      
      if isAnomaly {
        var anomaly = new AnomalyInstance();
        anomaly.pattern_id = patterns[i].id;
        anomaly.technique = patterns[i].technique;
        anomaly.anomaly_score = maxZScore;
        anomaly.deviation_factor = maxZScore / threshold;
        anomaly.detected_at = getCurrentTime();
        
        anomalyList.push_back(anomaly);
      }
    }
    
    anomalies = anomalyList;
    writeln("  ✅ Detected ", anomalies.size, " anomalies");
  }
  
  // Calculate statistics (mean, stddev, correlation)
  proc calculateStatistics() {
    writeln("📊 Calculating pattern statistics...");
    
    // Initialize
    forall j in 1..numFeatures {
      stats.mean[j] = 0.0;
      stats.min[j] = 1e10;
      stats.max[j] = -1e10;
    }
    
    // Calculate mean
    for i in 1..numPatterns {
      for j in 1..numFeatures {
        stats.mean[j] += patterns[i].feature_vector[j];
        stats.min[j] = min(stats.min[j], patterns[i].feature_vector[j]);
        stats.max[j] = max(stats.max[j], patterns[i].feature_vector[j]);
      }
    }
    forall j in 1..numFeatures {
      stats.mean[j] /= numPatterns;
    }
    
    // Calculate standard deviation
    forall j in 1..numFeatures {
      var variance = 0.0;
      for i in 1..numPatterns {
        variance += (patterns[i].feature_vector[j] - stats.mean[j]) ** 2;
      }
      stats.stddev[j] = sqrt(variance / numPatterns);
    }
    
    writeln("  ✅ Statistics calculated");
  }
  
  // Export clustering results
  proc exportResults(outputFile: string) {
    writeln("💾 Exporting results to ", outputFile);
    
    var f = open(outputFile, iomode.cw);
    var w = f.writer();
    
    w.writeln("{");
    w.writeln('  "summary": {');
    w.writeln('    "total_patterns": ', numPatterns, ',');
    w.writeln('    "num_clusters": ', numClusters, ',');
    w.writeln('    "num_anomalies": ', anomalies.size, ',');
    w.writeln('    "clustering_method": "K-Means++"');
    w.writeln("  },");
    
    w.writeln('  "cluster_metrics": [');
    for k in 1..numClusters {
      w.writeln("    {");
      w.writeln('      "cluster_id": ', k, ',');
      w.writeln('      "size": ', cluster_metrics[k].size, ',');
      w.writeln('      "silhouette_score": ', cluster_metrics[k].silhouette_score, ',');
      w.writeln('      "intra_distance": ', cluster_metrics[k].intra_distance);
      if k < numClusters {
        w.writeln("    },");
      } else {
        w.writeln("    }");
      }
    }
    w.writeln("  ]");
    w.writeln("}");
    
    f.close();
    writeln("  ✅ Results exported");
  }
}

// ============================================================================
// MAIN: Execute data mining pipeline
// ============================================================================

proc main() {
  writeln("\n", "="*70);
  writeln("🔬 NUCLEAR CHAPEL DATA MINING ENGINE");
  writeln("="*70, "\n");
  
  var startTime = getCurrentTime();
  
  // Initialize mining engine
  var engine = new DataMiningEngine();
  
  // Extract patterns (simulated from scraping data)
  engine.extractPatterns("scraped_data.json");
  
  // Initialize and run K-Means
  engine.initializeCentroids();
  engine.kMeansClustering();
  
  // Calculate metrics and detect anomalies
  engine.calculateClusterMetrics();
  engine.detectAnomalies(threshold=3.0);
  
  // Export results
  engine.exportResults("mining_results.json");
  
  var endTime = getCurrentTime();
  var elapsedTime = endTime - startTime;
  
  writeln("\n", "="*70);
  writeln("✅ MINING COMPLETE");
  writeln("  Time: ", elapsedTime, " seconds");
  writeln("  Patterns: ", engine.numPatterns);
  writeln("  Clusters: ", engine.numClusters);
  writeln("  Anomalies: ", engine.anomalies.size);
  writeln("="*70, "\n");
}
