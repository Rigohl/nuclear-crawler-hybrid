using System;
using System.Data;
using System.Collections.Generic;
using Microsoft.Extensions.Logging;
using DuckDB.NET.Data;

namespace GitHubMcpAgent
{
    public class DatabaseIntegration
    {
        private readonly string _connectionString;
        private readonly ILogger<DatabaseIntegration> _logger;

        public DatabaseIntegration(string dbPath, ILogger<DatabaseIntegration> logger)
        {
            _connectionString = $"Data Source={dbPath}";
            _logger = logger;
        }

        public DuckDBConnection GetConnection()
        {
            return new DuckDBConnection(_connectionString);
        }

        public IDataReader ExecuteQuery(string query)
        {
            try
            {
                var connection = GetConnection();
                connection.Open();
                var command = connection.CreateCommand();
                command.CommandText = query;
                return command.ExecuteReader(CommandBehavior.CloseConnection);
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error executing query: {Query}", query);
                throw;
            }
        }
        
        public List<string> Search(string keyword) {
             // Example search implementation (assumes there is a table to search)
             // This needs to be adapted to the actual schema of master_database.duckdb
             var results = new List<string>();
             try {
                // Determine tables first? For now, let's just allow raw SQL or implement a generic search if schema is known.
                // Since I don't know the schema, I will just provide a stub for now.
                _logger.LogWarning("Search not fully implemented without schema knowledge.");
             } catch (Exception ex) {
                 _logger.LogError(ex, "Search failed");
             }
             return results;
        }
    }
}
