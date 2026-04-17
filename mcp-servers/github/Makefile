BINARY := github-mcp-server
GO := go

.PHONY: build test clean run

build:
	$(GO) build -o $(BINARY) ./cmd/github-mcp-server/

test:
	$(GO) test ./...

clean:
	rm -f $(BINARY)

run: build
	GITHUB_TOKEN=$${GITHUB_TOKEN} ./$(BINARY)
