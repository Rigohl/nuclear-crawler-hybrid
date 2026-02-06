# Resultados - Search Results Output

This directory stores search results and outputs from the Nuclear Crawler Hybrid system.

## Purpose

All search operations, data mining, and analysis results are stored here to keep the project root clean and organized.

## Contents

Search results will include:
- Web search results (websearch tool)
- Premium content extractions (premium tool)
- File search results (file_search tool)
- Workspace scans (scan tool)
- AI dataset training outputs (ai_dataset_trainer tool)
- Chapel AI training results
- Data mining outputs
- Scientific analysis results

## Structure

Results are automatically organized by:
- Timestamp
- Search type
- Query parameters
- Tool used

## Cleanup

This directory can be safely cleaned periodically without affecting the core functionality:

```bash
# Remove old results (older than 30 days)
find resultados/ -type f -mtime +30 -delete

# Clear all results
rm -rf resultados/*
```

## Integration

The MCP server and Chapel AI engines automatically write their results to this directory.
