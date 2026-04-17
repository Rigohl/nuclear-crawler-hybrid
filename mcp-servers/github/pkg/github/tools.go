// Package github implements the GitHub API MCP tools.
package github

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	gh "github.com/google/go-github/v58/github"
	"golang.org/x/oauth2"
)

// Client wraps the GitHub API client.
type Client struct {
	gh *gh.Client
}

// New creates a GitHub client using the GITHUB_TOKEN environment variable.
func New() (*Client, error) {
	token := os.Getenv("GITHUB_TOKEN")
	if token == "" {
		return nil, fmt.Errorf("GITHUB_TOKEN environment variable is required")
	}
	ts := oauth2.StaticTokenSource(&oauth2.Token{AccessToken: token})
	tc := oauth2.NewClient(context.Background(), ts)
	return &Client{gh: gh.NewClient(tc)}, nil
}

// ─────────────────────────────────────────────
// TOOL: search_code
// ─────────────────────────────────────────────

type SearchCodeArgs struct {
	Query   string `json:"query"`
	PerPage int    `json:"per_page,omitempty"`
}

type SearchCodeResult struct {
	TotalCount int          `json:"total_count"`
	Items      []CodeResult `json:"items"`
}

type CodeResult struct {
	Name       string `json:"name"`
	Path       string `json:"path"`
	HTMLURL    string `json:"html_url"`
	Repository string `json:"repository"`
}

func (c *Client) SearchCode(ctx context.Context, raw json.RawMessage) (interface{}, error) {
	var args SearchCodeArgs
	if err := json.Unmarshal(raw, &args); err != nil {
		return nil, fmt.Errorf("invalid args: %w", err)
	}
	if args.PerPage == 0 {
		args.PerPage = 30
	}
	opts := &gh.SearchOptions{ListOptions: gh.ListOptions{PerPage: args.PerPage}}
	results, _, err := c.gh.Search.Code(ctx, args.Query, opts)
	if err != nil {
		return nil, fmt.Errorf("GitHub search_code: %w", err)
	}
	out := SearchCodeResult{TotalCount: results.GetTotal()}
	for _, item := range results.CodeResults {
		out.Items = append(out.Items, CodeResult{
			Name:       item.GetName(),
			Path:       item.GetPath(),
			HTMLURL:    item.GetHTMLURL(),
			Repository: item.GetRepository().GetFullName(),
		})
	}
	return out, nil
}

// ─────────────────────────────────────────────
// TOOL: list_issues
// ─────────────────────────────────────────────

type ListIssuesArgs struct {
	Owner string `json:"owner"`
	Repo  string `json:"repo"`
	State string `json:"state,omitempty"` // "open", "closed", "all"
}

type IssueResult struct {
	Number int    `json:"number"`
	Title  string `json:"title"`
	State  string `json:"state"`
	URL    string `json:"html_url"`
	Body   string `json:"body"`
}

func (c *Client) ListIssues(ctx context.Context, raw json.RawMessage) (interface{}, error) {
	var args ListIssuesArgs
	if err := json.Unmarshal(raw, &args); err != nil {
		return nil, fmt.Errorf("invalid args: %w", err)
	}
	if args.State == "" {
		args.State = "open"
	}
	opts := &gh.IssueListByRepoOptions{State: args.State}
	issues, _, err := c.gh.Issues.ListByRepo(ctx, args.Owner, args.Repo, opts)
	if err != nil {
		return nil, fmt.Errorf("GitHub list_issues: %w", err)
	}
	var out []IssueResult
	for _, iss := range issues {
		out = append(out, IssueResult{
			Number: iss.GetNumber(),
			Title:  iss.GetTitle(),
			State:  iss.GetState(),
			URL:    iss.GetHTMLURL(),
			Body:   iss.GetBody(),
		})
	}
	return out, nil
}

// ─────────────────────────────────────────────
// TOOL: create_issue
// ─────────────────────────────────────────────

type CreateIssueArgs struct {
	Owner  string   `json:"owner"`
	Repo   string   `json:"repo"`
	Title  string   `json:"title"`
	Body   string   `json:"body,omitempty"`
	Labels []string `json:"labels,omitempty"`
}

func (c *Client) CreateIssue(ctx context.Context, raw json.RawMessage) (interface{}, error) {
	var args CreateIssueArgs
	if err := json.Unmarshal(raw, &args); err != nil {
		return nil, fmt.Errorf("invalid args: %w", err)
	}
	req := &gh.IssueRequest{
		Title:  gh.String(args.Title),
		Body:   gh.String(args.Body),
		Labels: &args.Labels,
	}
	issue, _, err := c.gh.Issues.Create(ctx, args.Owner, args.Repo, req)
	if err != nil {
		return nil, fmt.Errorf("GitHub create_issue: %w", err)
	}
	return IssueResult{
		Number: issue.GetNumber(),
		Title:  issue.GetTitle(),
		State:  issue.GetState(),
		URL:    issue.GetHTMLURL(),
		Body:   issue.GetBody(),
	}, nil
}

// ─────────────────────────────────────────────
// TOOL: list_repos
// ─────────────────────────────────────────────

type ListReposArgs struct {
	Owner string `json:"owner"`
	Type  string `json:"type,omitempty"` // "all", "public", "private"
}

type RepoResult struct {
	Name        string `json:"name"`
	FullName    string `json:"full_name"`
	Description string `json:"description"`
	HTMLURL     string `json:"html_url"`
	Stars       int    `json:"stars"`
	Language    string `json:"language"`
}

func (c *Client) ListRepos(ctx context.Context, raw json.RawMessage) (interface{}, error) {
	var args ListReposArgs
	if err := json.Unmarshal(raw, &args); err != nil {
		return nil, fmt.Errorf("invalid args: %w", err)
	}
	if args.Type == "" {
		args.Type = "public"
	}
	opts := &gh.RepositoryListByUserOptions{Type: args.Type}
	repos, _, err := c.gh.Repositories.ListByUser(context.Background(), args.Owner, opts)
	if err != nil {
		return nil, fmt.Errorf("GitHub list_repos: %w", err)
	}
	var out []RepoResult
	for _, r := range repos {
		out = append(out, RepoResult{
			Name:        r.GetName(),
			FullName:    r.GetFullName(),
			Description: r.GetDescription(),
			HTMLURL:     r.GetHTMLURL(),
			Stars:       r.GetStargazersCount(),
			Language:    r.GetLanguage(),
		})
	}
	return out, nil
}
