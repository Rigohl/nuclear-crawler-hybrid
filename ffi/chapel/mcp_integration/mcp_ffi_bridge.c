// MCP FFI Bridge - C interface para Chapel
// Permite a Chapel llamar MCPs directamente via HTTP/stdio

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>

// Estructura para almacenar respuestas HTTP
struct MemoryStruct {
    char *memory;
    size_t size;
};

// Callback para libcurl
static size_t WriteMemoryCallback(void *contents, size_t size, size_t nmem, void *userp) {
    size_t realsize = size * nmem;
    struct MemoryStruct *mem = (struct MemoryStruct *)userp;

    char *ptr = realloc(mem->memory, mem->size + realsize + 1);
    if (!ptr) {
        printf("ERROR: Not enough memory (realloc returned NULL)\n");
        return 0;
    }

    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

// MCP HTTP Call (para MCPs remotos: GitHub, Supabase, Hugging Face, etc.)
const char* mcp_call_http(const char* url, const char* method, const char* json_payload) {
    CURL *curl;
    CURLcode res;
    struct MemoryStruct chunk;

    chunk.memory = malloc(1);
    chunk.size = 0;

    curl_global_init(CURL_GLOBAL_ALL);
    curl = curl_easy_init();

    if (curl) {
        // Headers
        struct curl_slist *headers = NULL;
        headers = curl_slist_append(headers, "Content-Type: application/json");
        
        curl_easy_setopt(curl, CURLOPT_URL, url);
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteMemoryCallback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&chunk);
        curl_easy_setopt(curl, CURLOPT_USERAGENT, "Chapel-MCP-Bridge/1.0");

        if (strcmp(method, "POST") == 0 && json_payload) {
            curl_easy_setopt(curl, CURLOPT_POSTFIELDS, json_payload);
        }

        res = curl_easy_perform(curl);

        if (res != CURLE_OK) {
            fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
        }

        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
    }

    curl_global_cleanup();

    return chunk.memory;
}

// MCP Call dispatcher (llama al MCP correcto)
const char* mcp_call(const char* server_name, const char* tool_name, const char* args_json) {
    char url[512];
    
    // URLs de MCPs
    if (strcmp(server_name, "github") == 0) {
        snprintf(url, sizeof(url), "https://api.githubcopilot.com/mcp/");
    }
    else if (strcmp(server_name, "supabase") == 0) {
        // Usar SUPABASE_PROJECT_ID del env
        const char* project_id = getenv("SUPABASE_PROJECT_ID");
        if (project_id) {
            snprintf(url, sizeof(url), 
                "https://mcp.supabase.com/mcp?project_ref=%s&features=database", 
                project_id);
        } else {
            snprintf(url, sizeof(url), "https://mcp.supabase.com/mcp");
        }
    }
    else if (strcmp(server_name, "huggingface") == 0) {
        snprintf(url, sizeof(url), "https://huggingface.co/mcp");
    }
    else if (strcmp(server_name, "vercel") == 0) {
        snprintf(url, sizeof(url), "https://mcp.vercel.com");
    }
    else {
        return "{\"error\": \"Unknown MCP server\"}";
    }
    
    // Construir payload JSON
    char payload[2048];
    snprintf(payload, sizeof(payload), 
        "{\"method\": \"tools/call\", \"params\": {\"name\": \"%s\", \"arguments\": %s}}",
        tool_name, args_json);
    
    // Llamar HTTP
    return mcp_call_http(url, "POST", payload);
}

// MCP Search (wrapper simplificado)
const char* mcp_search(const char* server_name, const char* query, int max_results) {
    char args[512];
    snprintf(args, sizeof(args), "{\"query\": \"%s\", \"max_results\": %d}", query, max_results);
    
    return mcp_call(server_name, "search", args);
}

// MCP Store (guardar en Supabase)
int mcp_store(const char* table, const char* data_json) {
    char args[2048];
    snprintf(args, sizeof(args), "{\"table\": \"%s\", \"data\": %s}", table, data_json);
    
    const char* result = mcp_call("supabase", "insert", args);
    
    // Check if success
    if (strstr(result, "\"error\"") == NULL) {
        return 1;  // Success
    }
    
    return 0;  // Error
}
