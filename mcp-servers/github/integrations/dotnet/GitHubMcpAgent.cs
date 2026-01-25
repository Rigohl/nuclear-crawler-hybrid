using System;
using System.Net.Http;
using System.Threading.Tasks;
using System.Text.Json;
using Microsoft.Extensions.Logging;

namespace GitHubMcpAgent
{
    /// <summary>
    /// .NET agent for interacting with GitHub MCP Server
    /// </summary>
    public class GitHubMcpAgent
    {
        private readonly HttpClient _httpClient;
        private readonly ILogger<GitHubMcpAgent> _logger;
        private readonly string _mcpServerUrl;
        private readonly string _apiToken;

        public GitHubMcpAgent(
            HttpClient httpClient,
            ILogger<GitHubMcpAgent> logger,
            string mcpServerUrl,
            string apiToken)
        {
            _httpClient = httpClient;
            _logger = logger;
            _mcpServerUrl = mcpServerUrl;
            _apiToken = apiToken;
            
            _httpClient.DefaultRequestHeaders.Authorization = 
                new System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", _apiToken);
        }

        /// <summary>
        /// Get file contents from a GitHub repository
        /// </summary>
        public async Task<string> GetFileContentsAsync(string owner, string repo, string path, string? reference = null)
        {
            var request = new
            {
                tool = "get_file_contents",
                arguments = new
                {
                    owner = owner,
                    repo = repo,
                    path = path,
                    @ref = reference
                }
            };

            return await CallMcpToolAsync(request);
        }

        /// <summary>
        /// List issues from a repository
        /// </summary>
        public async Task<JsonElement> ListIssuesAsync(string owner, string repo, string? state = null)
        {
            var request = new
            {
                tool = "list_issues",
                arguments = new
                {
                    owner = owner,
                    repo = repo,
                    state = state ?? "open"
                }
            };

            var response = await CallMcpToolAsync(request);
            return JsonDocument.Parse(response).RootElement;
        }

        /// <summary>
        /// Create a new issue
        /// </summary>
        public async Task<string> CreateIssueAsync(string owner, string repo, string title, string body)
        {
            var request = new
            {
                tool = "issue_write",
                arguments = new
                {
                    owner = owner,
                    repo = repo,
                    method = "create",
                    title = title,
                    body = body
                }
            };

            return await CallMcpToolAsync(request);
        }

        /// <summary>
        /// Get pull request details
        /// </summary>
        public async Task<JsonElement> GetPullRequestAsync(string owner, string repo, int pullNumber)
        {
            var request = new
            {
                tool = "pull_request_read",
                arguments = new
                {
                    owner = owner,
                    repo = repo,
                    pullNumber = pullNumber,
                    method = "get"
                }
            };

            var response = await CallMcpToolAsync(request);
            return JsonDocument.Parse(response).RootElement;
        }

        /// <summary>
        /// List workflow runs
        /// </summary>
        public async Task<JsonElement> ListWorkflowRunsAsync(string owner, string repo, string workflowId)
        {
            var request = new
            {
                tool = "list_workflow_runs",
                arguments = new
                {
                    owner = owner,
                    repo = repo,
                    workflow_id = workflowId
                }
            };

            var response = await CallMcpToolAsync(request);
            return JsonDocument.Parse(response).RootElement;
        }

        /// <summary>
        /// Generic method to call any MCP tool
        /// </summary>
        private async Task<string> CallMcpToolAsync(object request)
        {
            try
            {
                var json = JsonSerializer.Serialize(request);
                var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");

                var response = await _httpClient.PostAsync($"{_mcpServerUrl}/tools/call", content);
                response.EnsureSuccessStatusCode();

                return await response.Content.ReadAsStringAsync();
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error calling MCP tool: {Tool}", request);
                throw;
            }
        }
    }
}
