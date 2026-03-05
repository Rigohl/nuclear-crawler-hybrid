<<<<<<< SEARCH
    #[test]
    fn test_health_assessment() {
        let stats = vec![FolderStats {
            name: "test".to_string(),
            file_count: 100,
            total_size: 1_000_000,
            file_types: HashMap::new(),
            health_score: 75.0,
        }];

        let health = AdvancedAnalysisEngine::calculate_health(&stats);
        assert!(health.overall_score > 0.0);
    }
}
=======
    #[test]
    fn test_health_assessment() {
        let stats = vec![FolderStats {
            name: "test".to_string(),
            file_count: 100,
            total_size: 1_000_000,
            file_types: HashMap::new(),
            health_score: 75.0,
        }];

        let health = AdvancedAnalysisEngine::calculate_health(&stats);
        assert!(health.overall_score > 0.0);
    }

    #[test]
    fn test_folder_stats_size_calculations() {
        let stats = FolderStats {
            name: "test_sizes".to_string(),
            file_count: 10,
            total_size: 1024 * 1024 * 1536, // 1.5 GB
            file_types: HashMap::new(),
            health_score: 100.0,
        };

        // 1536 MB
        assert!((stats.size_mb() - 1536.0).abs() < f64::EPSILON);
        // 1.5 GB
        assert!((stats.size_gb() - 1.5).abs() < f64::EPSILON);

        let zero_stats = FolderStats {
            name: "empty".to_string(),
            file_count: 0,
            total_size: 0,
            file_types: HashMap::new(),
            health_score: 0.0,
        };

        assert_eq!(zero_stats.size_mb(), 0.0);
        assert_eq!(zero_stats.size_gb(), 0.0);
    }
}
>>>>>>> REPLACE
