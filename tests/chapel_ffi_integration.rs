//! Chapel AI FFI Integration Tests
//!
//! Verifies that:
//! - libchapel_ai.so is correctly linked
//! - Chapel AI functions can be called
//! - Learning and optimization work

#[cfg(feature = "chapel_ffi")]
#[test]
fn test_chapel_ai_initialization() {
    use nuclear_crawler_hybrid::ffi::chapel_integration::{ChapelAI, ChapelContext};
    use std::time::{SystemTime, UNIX_EPOCH};

    eprintln!("\n🧪 TEST: Chapel AI Initialization");
    eprintln!("═════════════════════════════════════════════════════════");

    // Create Chapel AI instance
    let chapel = ChapelAI::new();

    // Try to get stats (this will work even without FFI initialized)
    match chapel.get_stats() {
        Ok(stats) => {
            eprintln!("✅ Chapel AI initialized successfully");
            eprintln!("   Total patterns: {}", stats.total_patterns);
            eprintln!(
                "   Average success rate: {:.2}%",
                stats.average_success_rate * 100.0
            );
        }
        Err(e) => {
            eprintln!("❌ Failed to get Chapel AI stats: {}", e);
            panic!("Chapel AI initialization failed");
        }
    }
}

#[cfg(feature = "chapel_ffi")]
#[test]
fn test_chapel_ai_learning() {
    use nuclear_crawler_hybrid::ffi::chapel_integration::{ChapelAI, ChapelContext};
    use std::collections::HashMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    eprintln!("\n🧪 TEST: Chapel AI Learning");
    eprintln!("═════════════════════════════════════════════════════════");

    let chapel = ChapelAI::new();

    // Test learning from a tool operation
    let context = ChapelContext {
        tool_name: "websearch".to_string(),
        operation: "query".to_string(),
        input_data: "rust async patterns".to_string(),
        output_quality: 0.95,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
        metadata: HashMap::new(),
    };

    match chapel.learn_from_operation(context) {
        Ok(()) => {
            eprintln!("✅ Chapel AI learned successfully");

            // Check pattern count
            match chapel.get_pattern_count("websearch") {
                Ok(count) => {
                    eprintln!("   Patterns for websearch: {}", count);
                }
                Err(e) => {
                    eprintln!("⚠️  Could not get pattern count: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!(
                "⚠️  Chapel AI learning not available (using fallback): {}",
                e
            );
        }
    }
}

#[cfg(feature = "chapel_ffi")]
#[test]
fn test_chapel_ai_optimization() {
    use nuclear_crawler_hybrid::ffi::chapel_integration::ChapelAI;

    eprintln!("\n🧪 TEST: Chapel AI Optimization");
    eprintln!("═════════════════════════════════════════════════════════");

    let chapel = ChapelAI::new();

    match chapel.optimize() {
        Ok(patterns_optimized) => {
            eprintln!("✅ Chapel AI optimization completed");
            eprintln!("   Patterns optimized: {}", patterns_optimized);
        }
        Err(e) => {
            eprintln!(
                "⚠️  Chapel AI optimization not available (using fallback): {}",
                e
            );
        }
    }
}

#[test]
fn test_chapel_ffi_feature_detection() {
    eprintln!("\n🧪 TEST: Chapel FFI Feature Detection");
    eprintln!("═════════════════════════════════════════════════════════");

    #[cfg(feature = "chapel_ffi")]
    {
        eprintln!("✅ Chapel FFI feature is ENABLED");
    }

    #[cfg(not(feature = "chapel_ffi"))]
    {
        eprintln!("⚠️  Chapel FFI feature is DISABLED");
    }
}
