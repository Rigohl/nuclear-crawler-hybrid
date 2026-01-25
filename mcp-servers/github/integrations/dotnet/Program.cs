using System;
using System.Threading.Tasks;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using GitHubMcpAgent;

// Example .NET application using GitHub MCP Agent
class Program
{
    static async Task Main(string[] args)
    {
        // Setup dependency injection
        var services = new ServiceCollection();
        services.AddLogging(builder => builder.AddConsole());
        services.AddHttpClient<GitHubMcpAgent.GitHubMcpAgent>();
        
        var serviceProvider = services.BuildServiceProvider();
        var logger = serviceProvider.GetRequiredService<ILogger<Program>>();
        
        // Get MCP server URL and token from environment
        var mcpServerUrl = Environment.GetEnvironmentVariable("GITHUB_MCP_SERVER_URL") 
            ?? "http://localhost:8080/mcp";
        var apiToken = Environment.GetEnvironmentVariable("GITHUB_PERSONAL_ACCESS_TOKEN")
            ?? Environment.GetEnvironmentVariable("GH_TOKEN")
            ?? throw new InvalidOperationException("GITHUB_PERSONAL_ACCESS_TOKEN or GH_TOKEN must be set");
        
        // Create agent
        var httpClientFactory = serviceProvider.GetRequiredService<IHttpClientFactory>();
        var httpClient = httpClientFactory.CreateClient();
        var agent = new GitHubMcpAgent.GitHubMcpAgent(
            httpClient,
            serviceProvider.GetRequiredService<ILogger<GitHubMcpAgent.GitHubMcpAgent>>(),
            mcpServerUrl,
            apiToken
        );
        
        // Example: Get file contents
        if (args.Length > 0 && args[0] == "inspect-db")
        {
            var dbPath = args.Length > 1 ? args[1] : @"d:\DATABASES\master_database.duckdb";
            var db = new DatabaseIntegration(dbPath, serviceProvider.GetRequiredService<ILogger<DatabaseIntegration>>());
            
            try 
            {
                using var reader = db.ExecuteQuery("SHOW TABLES");
                Console.WriteLine("Tables in database:");
                while (reader.Read())
                {
                    for (int i = 0; i < reader.FieldCount; i++)
                    {
                        Console.Write(reader.GetValue(i) + "\t");
                    }
                    Console.WriteLine();
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error inspecting DB: {ex.Message}");
            }
            return;
        }

        if (args.Length > 0 && args[0] == "query-db")
        {
            var dbPath = @"d:\DATABASES\master_database.duckdb";
            var query = args.Length > 1 ? args[1] : "SHOW TABLES";
            var db = new DatabaseIntegration(dbPath, serviceProvider.GetRequiredService<ILogger<DatabaseIntegration>>());
            
            try 
            {
                using var reader = db.ExecuteQuery(query);
                for (int i = 0; i < reader.FieldCount; i++)
                {
                    Console.Write(reader.GetName(i) + "\t");
                }
                Console.WriteLine();

                while (reader.Read())
                {
                    for (int i = 0; i < reader.FieldCount; i++)
                    {
                        Console.Write(reader.GetValue(i) + "\t");
                    }
                    Console.WriteLine();
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error executing query: {ex.Message}");
            }
            return;
        }

        if (args.Length >= 3)
        {
            var owner = args[0];
            var repo = args[1];
            var path = args[2];
            
            logger.LogInformation("Getting file contents: {Owner}/{Repo}/{Path}", owner, repo, path);
            
            try
            {
                var contents = await agent.GetFileContentsAsync(owner, repo, path);
                Console.WriteLine(contents);
            }
            catch (Exception ex)
            {
                logger.LogError(ex, "Error getting file contents");
                Environment.Exit(1);
            }
        }
        else
        {
            Console.WriteLine("Usage:");
            Console.WriteLine("  dotnet run <owner> <repo> <path>");
            Console.WriteLine("  dotnet run inspect-db [path_to_db]");
        }
    }
}
