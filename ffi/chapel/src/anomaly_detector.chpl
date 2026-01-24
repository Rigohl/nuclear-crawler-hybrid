module AnomalyDetector {
    use Math;
    
    // Anomaly detection parameters
    record AnomalyParams {
        sensitivity: real,        // 0.0 - 1.0
        window_size: int,
        threshold_multiplier: real
    }
    
    // Detected anomaly
    record Anomaly {
        timestamp: int,
        module_name: string,
        metric_type: string,
        value: real,
        expected: real,
        deviation: real,
        severity: string       // LOW, MEDIUM, HIGH, CRITICAL
    }
    
    // Statistical baseline
    record Baseline {
        module_name: string,
        mean_accuracy: real,
        std_accuracy: real,
        mean_latency: real,
        std_latency: real,
        mean_memory: real,
        std_memory: real
    }
    
    class AnomalyDetector {
        var params: AnomalyParams;
        var baselines: [1..6] Baseline;
        var anomalies: [1..1000] Anomaly;
        var anomaly_count = 0;
        var modules = ["Unified Nuclear AI", "Nuclear Chapel AI", "Code Analyzer", 
                      "Code Repair", "Code Reviewer", "NLP Tokenizer"];
        
        proc init(sensitivity: real = 0.85) {
            params.sensitivity = sensitivity;
            params.window_size = 100;
            params.threshold_multiplier = 2.5;
            initializeBaselines();
        }
        
        // Initialize baselines from historical data
        proc initializeBaselines() {
            baselines[1] = new Baseline(
                module_name = "Unified Nuclear AI",
                mean_accuracy = 96.0, std_accuracy = 0.5,
                mean_latency = 12.5, std_latency = 0.3,
                mean_memory = 512.0, std_memory = 10.0
            );
            baselines[2] = new Baseline(
                module_name = "Nuclear Chapel AI",
                mean_accuracy = 92.0, std_accuracy = 0.8,
                mean_latency = 15.2, std_latency = 0.5,
                mean_memory = 384.0, std_memory = 15.0
            );
            baselines[3] = new Baseline(
                module_name = "Code Analyzer",
                mean_accuracy = 98.0, std_accuracy = 0.3,
                mean_latency = 8.3, std_latency = 0.2,
                mean_memory = 256.0, std_memory = 8.0
            );
            baselines[4] = new Baseline(
                module_name = "Code Repair",
                mean_accuracy = 94.0, std_accuracy = 1.0,
                mean_latency = 25.6, std_latency = 1.0,
                mean_memory = 640.0, std_memory = 20.0
            );
            baselines[5] = new Baseline(
                module_name = "Code Reviewer",
                mean_accuracy = 97.0, std_accuracy = 0.6,
                mean_latency = 18.9, std_latency = 0.7,
                mean_memory = 480.0, std_memory = 12.0
            );
            baselines[6] = new Baseline(
                module_name = "NLP Tokenizer",
                mean_accuracy = 99.0, std_accuracy = 0.2,
                mean_latency = 5.2, std_latency = 0.1,
                mean_memory = 128.0, std_memory = 5.0
            );
        }
        
        // Detect anomaly using Z-score method
        proc detectAnomalyZScore(module_id: int, metric_type: string, 
                                value: real, timestamp: int) {
            var baseline = baselines[module_id];
            var mean = 0.0;
            var std = 0.0;
            var z_score = 0.0;
            
            select metric_type {
                when "accuracy" {
                    mean = baseline.mean_accuracy;
                    std = baseline.std_accuracy;
                }
                when "latency" {
                    mean = baseline.mean_latency;
                    std = baseline.std_latency;
                }
                when "memory" {
                    mean = baseline.mean_memory;
                    std = baseline.std_memory;
                }
            }
            
            // Calculate Z-score
            z_score = abs((value - mean) / (std + 0.001));
            
            // Determine if anomaly
            var threshold = params.threshold_multiplier;
            if z_score > threshold {
                var deviation = ((value - mean) / mean) * 100.0;
                var severity = "LOW";
                
                if z_score > threshold * 1.5 {
                    severity = "MEDIUM";
                }
                if z_score > threshold * 2.0 {
                    severity = "HIGH";
                }
                if z_score > threshold * 3.0 {
                    severity = "CRITICAL";
                }
                
                if anomaly_count < 1000 {
                    anomaly_count += 1;
                    anomalies[anomaly_count] = new Anomaly(
                        timestamp = timestamp,
                        module_name = baseline.module_name,
                        metric_type = metric_type,
                        value = value,
                        expected = mean,
                        deviation = deviation,
                        severity = severity
                    );
                }
            }
        }
        
        // Detect anomalies in real-time stream
        proc detectAnomaliesInStream() {
            writeln("\n╔═════════════════════════════════════════════════════════════╗");
            writeln("║         ADVANCED ANOMALY DETECTION - REAL-TIME STREAM        ║");
            writeln("╚═════════════════════════════════════════════════════════════╝\n");
            
            writeln("🔍 SCANNING METRICS FOR ANOMALIES...\n");
            
            // Simulate streaming metrics with anomalies
            var test_data = [
                (1, "accuracy", 96.1),   // Normal
                (1, "latency", 12.6),    // Normal
                (1, "memory", 550.0),    // ANOMALY: 7.4% deviation
                (2, "accuracy", 91.5),   // Normal
                (2, "latency", 20.0),    // ANOMALY: 31% deviation
                (3, "accuracy", 97.9),   // Normal
                (3, "latency", 8.2),     // Normal
                (4, "accuracy", 85.0),   // ANOMALY: -9.6% deviation
                (5, "latency", 25.0),    // Normal
                (6, "memory", 100.0),    // ANOMALY: -21.9% deviation
                (1, "accuracy", 75.0),   // CRITICAL ANOMALY
                (2, "memory", 800.0),    // CRITICAL ANOMALY
            ];
            
            for i in 1..test_data.size {
                var (module_id, metric_type, value) = test_data[i];
                detectAnomalyZScore(module_id, metric_type, value, i);
            }
            
            // Report anomalies
            writeln("📊 ANOMALIES DETECTED: ", anomaly_count, "\n");
            
            for i in 1..anomaly_count {
                var anom = anomalies[i];
                writeln("[", anom.severity, "] Timestamp ", anom.timestamp);
                writeln("  Module: ", anom.module_name);
                writeln("  Metric: ", anom.metric_type);
                writeln("  Value: ", anom.value, " (Expected: ", anom.expected, ")");
                writeln("  Deviation: ", anom.deviation:2:2, "%");
                
                select anom.severity {
                    when "LOW" {
                        writeln("  ℹ️  Minor deviation - monitoring");
                    }
                    when "MEDIUM" {
                        writeln("  ⚠️  Moderate anomaly - investigate");
                    }
                    when "HIGH" {
                        writeln("  🔴 Significant anomaly - immediate action");
                    }
                    when "CRITICAL" {
                        writeln("  🛑 CRITICAL ANOMALY - emergency action required!");
                    }
                }
                writeln();
            }
            
            // Summary statistics
            writeln("╔═════════════════════════════════════════════════════════════╗");
            writeln("║                 ANOMALY DETECTION SUMMARY                   ║");
            writeln("╚═════════════════════════════════════════════════════════════╝\n");
            
            var critical_count = 0, high_count = 0, medium_count = 0;
            for i in 1..anomaly_count {
                select anomalies[i].severity {
                    when "CRITICAL" { critical_count += 1; }
                    when "HIGH" { high_count += 1; }
                    when "MEDIUM" { medium_count += 1; }
                }
            }
            
            writeln("🛑 CRITICAL: ", critical_count);
            writeln("🔴 HIGH: ", high_count);
            writeln("⚠️  MEDIUM: ", medium_count);
            writeln("ℹ️  LOW: ", anomaly_count - critical_count - high_count - medium_count);
            
            if critical_count > 0 {
                writeln("\n⛔ ALERT: CRITICAL ANOMALIES DETECTED - SYSTEM INTERVENTION RECOMMENDED");
            } else if high_count > 0 {
                writeln("\n⚠️  WARNING: High-severity anomalies detected");
            } else {
                writeln("\n✅ System operating within normal parameters");
            }
            writeln();
        }
    }
    
    proc main() {
        var detector = new AnomalyDetector(sensitivity = 0.90);
        detector.detectAnomaliesInStream();
    }
}
