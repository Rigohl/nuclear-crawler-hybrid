using System;
using System.Net;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Xunit;
using GitHubMcpAgent;

namespace GitHubMcpAgent.Tests
{
    public class MockHttpMessageHandler : HttpMessageHandler
    {
        private readonly Func<HttpRequestMessage, HttpResponseMessage> _handler;

        public MockHttpMessageHandler(Func<HttpRequestMessage, HttpResponseMessage> handler)
        {
            _handler = handler;
        }

        protected override Task<HttpResponseMessage> SendAsync(HttpRequestMessage request, CancellationToken cancellationToken)
        {
            return Task.FromResult(_handler(request));
        }
    }

    public class AgentTests
    {
        [Fact]
        public async Task GetFileContentsAsync_ReturnsContent_WhenSuccess()
        {
            // Arrange
            var expectedResponse = "{\"content\": \"H4sIC...\"}"; // Mocked response from MCP server
            // The actual agent expects the raw string response from the MCP tool call
            var mockContent = "search engine optimization logic";
            
            var handler = new MockHttpMessageHandler(req =>
            {
                Assert.Equal("http://localhost:8080/mcp/tools/call", req.RequestUri.ToString());
                return new HttpResponseMessage(HttpStatusCode.OK)
                {
                    Content = new StringContent(mockContent)
                };
            });

            var httpClient = new HttpClient(handler);
            var logger = new LoggerFactory().CreateLogger<GitHubMcpAgent>();
            var agent = new GitHubMcpAgent(httpClient, logger, "http://localhost:8080/mcp", "fake-token");

            // Act
            var result = await agent.GetFileContentsAsync("memory-p", "skills", "9-motor-coordination.md");

            // Assert
            Assert.Equal(mockContent, result);
        }

        [Fact]
        public async Task ListIssuesAsync_ParsesJson_WhenSuccess()
        {
            // Arrange
            var jsonResponse = "[{\"title\": \"Fix Faiss GPU buffer overflow\", \"state\": \"open\"}]";
            
            var handler = new MockHttpMessageHandler(req =>
            {
                return new HttpResponseMessage(HttpStatusCode.OK)
                {
                    Content = new StringContent(jsonResponse)
                };
            });

            var httpClient = new HttpClient(handler);
            var logger = new LoggerFactory().CreateLogger<GitHubMcpAgent>();
            var agent = new GitHubMcpAgent(httpClient, logger, "http://localhost:8080/mcp", "fake-token");

            // Act
            var result = await agent.ListIssuesAsync("memory-p", "motores");

            // Assert
            Assert.Equal("open", result[0].GetProperty("state").GetString());
            Assert.Equal("Fix Faiss GPU buffer overflow", result[0].GetProperty("title").GetString());
        }
    }
}
