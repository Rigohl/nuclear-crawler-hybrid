// Package ghmcp implements the MCP JSON-RPC 2.0 protocol for the GitHub MCP server.
package ghmcp

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
)

// JSONRPCRequest is an incoming MCP JSON-RPC 2.0 request.
type JSONRPCRequest struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      interface{}     `json:"id"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params"`
}

// JSONRPCResponse is an outgoing MCP JSON-RPC 2.0 response.
type JSONRPCResponse struct {
	JSONRPC string      `json:"jsonrpc"`
	ID      interface{} `json:"id"`
	Result  interface{} `json:"result,omitempty"`
	Error   *RPCError   `json:"error,omitempty"`
}

// RPCError is an MCP JSON-RPC error object.
type RPCError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

// ToolsListResult is the response to tools/list.
type ToolsListResult struct {
	Tools []ToolDefinition `json:"tools"`
}

// ToolDefinition describes a single MCP tool.
type ToolDefinition struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ServeStdio reads JSON-RPC requests from stdin and writes responses to stdout.
func ServeStdio(handler func(*JSONRPCRequest) *JSONRPCResponse) {
	dec := json.NewDecoder(os.Stdin)
	enc := json.NewEncoder(os.Stdout)

	for {
		var req JSONRPCRequest
		if err := dec.Decode(&req); err != nil {
			if err == io.EOF {
				return
			}
			resp := &JSONRPCResponse{
				JSONRPC: "2.0",
				Error:   &RPCError{Code: -32700, Message: fmt.Sprintf("parse error: %v", err)},
			}
			_ = enc.Encode(resp)
			continue
		}
		resp := handler(&req)
		_ = enc.Encode(resp)
	}
}
