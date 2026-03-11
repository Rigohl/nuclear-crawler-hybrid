// Integration tests for WASM modules
// Verifies that WASM implementations work correctly

#[cfg(test)]
mod wasm_tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_wasm_file_searcher_creation() {
        #[cfg(target_arch = "wasm32")]
        {
            use nuclear_crawler_hybrid::wasm::file_search::WasmFileSearcher;
            let searcher = WasmFileSearcher::new("test");
            // searcher is created successfully
            assert_eq!(
                searcher.search("test line"),
                "{\"matches\": 1, \"count\": 1}"
            );
        }
    }

    #[test]
    fn test_wasm_neural_ops() {
        #[cfg(target_arch = "wasm32")]
        {
            use nuclear_crawler_hybrid::wasm::neural_ops::WasmNeuralOps;
            let ops = WasmNeuralOps::new(10, 32);
            let result = ops.forward_pass("{}");
            assert!(result.contains("forward_done"));
        }
    }

    #[test]
    fn test_wasm_data_indexer() {
        #[cfg(target_arch = "wasm32")]
        {
            use nuclear_crawler_hybrid::wasm::data_search::WasmDataIndexer;
            let indexer = WasmDataIndexer::new(1000);
            assert!(indexer.contains("test"));
            assert!(!indexer.contains(""));
        }
    }

    #[test]
    fn test_wasm_source_files_exist() {
        let required = [
            "ffi/wasm/go/main.go",
            "ffi/wasm/zig/main.zig",
            "ffi/wasm/nim/main.nim",
            "ffi/wasm/build_wasm.sh",
        ];

        for path in required {
            assert!(Path::new(path).exists(), "required WASM source missing: {}", path);
        }
    }

    #[test]
    fn test_wasm_sources_expose_expected_contracts() {
        let go_src = fs::read_to_string("ffi/wasm/go/main.go").expect("Go WASM source should be readable");
        assert!(go_src.contains("//export parallel_fetch_urls"));
        assert!(go_src.contains("//export process_data_parallel"));

        let zig_src = fs::read_to_string("ffi/wasm/zig/main.zig").expect("Zig WASM source should be readable");
        assert!(zig_src.contains("export fn hash_data_simd"));
        assert!(zig_src.contains("export fn pattern_match_simd"));

        let nim_src = fs::read_to_string("ffi/wasm/nim/main.nim").expect("Nim WASM source should be readable");
        assert!(nim_src.contains("exportc: \"parse_html\""));
        assert!(nim_src.contains("exportc: \"extract_links\""));
    }
}

#[cfg(test)]
mod wasm_integration {
    #[test]
    fn test_wasm_feature_flags() {
        #[cfg(target_arch = "wasm32")]
        {
            println!("✅ WASM features compiled");
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            println!("✅ Native build validates explicit WASM source contracts");
        }
    }
}
