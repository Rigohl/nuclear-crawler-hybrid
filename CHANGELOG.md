# Changelog

All notable changes to Nuclear Crawler Hybrid will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **HuggingFace Integration**: Full integration with HuggingFace Hub for AI model training and deployment
  - Dataset upload to HuggingFace Hub
  - Model fine-tuning capabilities
  - Inference API support for chat
  - Export Chapel AI learning data in HF format
  
- **Interactive Chatbot**: AI-powered chatbot with tool integration
  - Natural language conversational interface
  - Integration with all 5 MCP tools
  - Chapel AI learning from conversations
  - Support for HuggingFace models or local mode
  - Conversation history management
  - Quality assessment and statistics
  
- **Enhanced Chapel AI**: Continuous learning improvements
  - Persistent storage (save/load learning data)
  - HuggingFace export format
  - Training data generation for model fine-tuning
  - Statistics and analytics
  
- **FFI Skeleton Implementations**: Example implementations for:
  - Go: Parallel processing with goroutines
  - Zig: SIMD hashing and pattern matching
  - Nim: HTML parsing and extraction
  
- **Improved Dependabot Configuration**:
  - Intelligent dependency grouping (network, serialization, FFI, etc.)
  - Daily updates for security-critical dependencies
  - Ignore rules for problematic dependencies (bincode v3)
  - Separate groups for better PR management
  
- **Comprehensive Documentation**:
  - HuggingFace integration guide with examples
  - Chatbot usage guide
  - FFI build instructions
  - Updated README with new features

### Changed
- Enhanced `lib.rs` to include new modules (chatbot, HuggingFace integration)
- Updated MCP tools module to include chatbot tool
- Improved Chapel AI with persistence and export capabilities

### Dependencies
- Added `base64` 0.22 for HuggingFace API
- Added `mime` 0.3 for content type handling
- Added `toml` 0.8 for configuration support

### Fixed
- Type inference errors in chatbot quality assessment
- Lifetime issues with temporary values in chatbot responses
- Unused import warnings in AI dataset trainer
- Compilation errors across new modules

## [0.1.0] - Previous Release

### Added
- Initial MCP server implementation with 5 tools
- Chapel AI learning system
- FFI integration hooks (Go, Zig, Nim, JAX)
- Web search with 55+ engines
- Premium content extraction
- File search and analysis
- Workspace scanning
- AI dataset generation

[Unreleased]: https://github.com/Rigohl/nuclear-crawler-hybrid/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Rigohl/nuclear-crawler-hybrid/releases/tag/v0.1.0
