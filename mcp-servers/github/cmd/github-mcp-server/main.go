// github-mcp-server: MCP server exposing GitHub API tools over JSON-RPC 2.0 stdio.
//
// Usage:
//
//	export GITHUB_TOKEN="ghp_xxxxx"
//	./github-mcp-server
//
// The server reads JSON-RPC 2.0 requests from stdin and writes responses to stdout.
// Integrate with Claude Desktop, Cursor, or any MCP-compatible client.
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"

	ghtools "github.com/rigohl/nuclear-crawler-hybrid/mcp-servers/github/pkg/github"
	"github.com/rigohl/nuclear-crawler-hybrid/mcp-servers/github/internal/ghmcp"
)

var toolDefs = []ghmcp.ToolDefinition{
	{
		Name:        "search_code",
		Description: "Search GitHub code across all public repositories. Returns matching files with repo context.",
		InputSchema: json.RawMessage(`{
			"type": "object",
			"required": ["query"],
			"properties": {
				"query":    {"type": "string", "description": "GitHub code search query"},
				"per_page": {"type": "integer", "description": "Max results (default 30, max 100)"}
			}
		}`),
	},
	{
		Name:        "list_issues",
		Description: "List issues for a GitHub repository.",
		InputSchema: json.RawMessage(`{
			"type": "object",
			"required": ["owner", "repo"],
			"properties": {
				"owner": {"type": "string"},
				"repo":  {"type": "string"},
				"state": {"type": "string", "enum": ["open", "closed", "all"], "default": "open"}
			}
		}`),
	},
	{
		Name:        "create_issue",
		Description: "Create a new issue in a GitHub repository.",
		InputSchema: json.RawMessage(`{
			"type": "object",
			"required": ["owner", "repo", "title"],
			"properties": {
				"owner":  {"type": "string"},
				"repo":   {"type": "string"},
				"title":  {"type": "string"},
				"body":   {"type": "string"},
				"labels": {"type": "array", "items": {"type": "string"}}
			}
		}`),
	},
	{
		Name:        "list_repos",
		Description: "List repositories for a GitHub user or organization.",
		InputSchema: json.RawMessage(`{
			"type": "object",
			"required": ["owner"],
			"properties": {
				"owner": {"type": "string"},
				"type":  {"type": "string", "enum": ["all", "public", "private"], "default": "public"}
			}
		}`),
	},
}

func main() {
	client, err := ghtools.New()
	if err != nil {
		log.Fatalf("GitHub MCP server startup failed: %v", err)
	}

	log.SetOutput(os.Stderr)
	log.Println("GitHub MCP Server started — listening on stdin")

	ghmcp.ServeStdio(func(req *ghmcp.JSONRPCRequest) *ghmcp.JSONRPCResponse {
		switch req.Method {
		case "initialize":
			return &ghmcp.JSONRPCResponse{
				JSONRPC: "2.0",
				ID:      req.ID,
				Result: map[string]interface{}{
					"protocolVersion": "2026-02-06",
					"capabilities":    map[string]interface{}{"tools": map[string]interface{}{}},
					"serverInfo":      map[string]interface{}{"name": "nuclear-github-mcp", "version": "1.0.0"},
				},
			}

		case "tools/list":
			return &ghmcp.JSONRPCResponse{
				JSONRPC: "2.0",
				ID:      req.ID,
				Result:  ghmcp.ToolsListResult{Tools: toolDefs},
			}

		case "tools/call":
			var params struct {
				Name      string          `json:"name"`
				Arguments json.RawMessage `json:"arguments"`
			}
			if err := json.Unmarshal(req.Params, &params); err != nil {
				return errResp(req.ID, -32600, fmt.Sprintf("invalid params: %v", err))
			}

			ctx := context.Background()
			var result interface{}
			var callErr error

			switch params.Name {
			case "search_code":
				result, callErr = client.SearchCode(ctx, params.Arguments)
			case "list_issues":
				result, callErr = client.ListIssues(ctx, params.Arguments)
			case "create_issue":
				result, callErr = client.CreateIssue(ctx, params.Arguments)
			case "list_repos":
				result, callErr = client.ListRepos(ctx, params.Arguments)
			default:
				return errResp(req.ID, -32601, fmt.Sprintf("unknown tool: %s", params.Name))
			}

			if callErr != nil {
				return errResp(req.ID, -32000, callErr.Error())
			}

			resultBytes, _ := json.Marshal(result)
			return &ghmcp.JSONRPCResponse{
				JSONRPC: "2.0",
				ID:      req.ID,
				Result: map[string]interface{}{
					"content": []map[string]interface{}{
						{"type": "text", "text": string(resultBytes)},
					},
				},
			}

		default:
			return errResp(req.ID, -32601, fmt.Sprintf("method not found: %s", req.Method))
		}
	})
}

func errResp(id interface{}, code int, msg string) *ghmcp.JSONRPCResponse {
	return &ghmcp.JSONRPCResponse{
		JSONRPC: "2.0",
		ID:      id,
		Error:   &ghmcp.RPCError{Code: code, Message: msg},
	}
}
