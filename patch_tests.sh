#!/bin/bash
cat << 'PATCH' > update.patch
--- src/mcp/protocol.rs
+++ src/mcp/protocol.rs
@@ -493,6 +493,26 @@
         assert!(!names.contains(&"websearch_complete".to_string()));
     }

+    #[test]
+    fn test_tool_exists() {
+        assert!(tool_exists("websearch"));
+        assert!(tool_exists("osint_intelligence"));
+        assert!(!tool_exists("non_existent_tool"));
+        assert!(!tool_exists(""));
+    }
+
+    #[test]
+    fn test_get_tool_definition() {
+        let def = get_tool_definition("websearch");
+        assert!(def.is_some());
+        assert_eq!(def.unwrap().name, "websearch");
+
+        let missing = get_tool_definition("fake_tool_123");
+        assert!(missing.is_none());
+    }
+
     #[test]
     fn test_request_validation() {
PATCH
patch src/mcp/protocol.rs < update.patch
