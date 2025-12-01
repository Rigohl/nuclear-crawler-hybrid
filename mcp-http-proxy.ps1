# MCP Proxy - Conecta a servidor HTTP Nuclear Crawler en puerto 3001
# Convierte JSON-RPC stdio en requests HTTP al servidor

$ErrorActionPreference = "Continue"
$uri = "http://localhost:3001/mcp/message"
$maxRetries = 3
$retryDelay = 1000

# Crear cliente HTTP reutilizable
$handler = New-Object System.Net.Http.HttpClientHandler
$handler.ServerCertificateCustomValidationCallback = { $true }
$client = New-Object System.Net.Http.HttpClient($handler)
$client.Timeout = [TimeSpan]::FromSeconds(300)
$client.DefaultRequestHeaders.Add("Content-Type", "application/json")

try {
    # Leer stdin línea por línea
    while ($true) {
        $line = [System.Console]::In.ReadLine()
        if ($null -eq $line) { break }
        
        if ([string]::IsNullOrWhiteSpace($line)) { continue }

        $retryCount = 0
        $success = $false

        while ($retryCount -lt $maxRetries -and -not $success) {
            try {
                # Enviar request HTTP POST con el JSON
                $content = New-Object System.Net.Http.StringContent($line, [System.Text.Encoding]::UTF8, "application/json")
                $response = $client.PostAsync($uri, $content).Result
                
                if ($response.IsSuccessStatusCode) {
                    $responseText = $response.Content.ReadAsStringAsync().Result
                    Write-Output $responseText
                    $success = $true
                }
                else {
                    $errorMsg = "HTTP {0}: {1}" -f $response.StatusCode, $response.Content.ReadAsStringAsync().Result
                    Write-Error $errorMsg -ErrorAction Continue
                    if ($retryCount -lt ($maxRetries - 1)) {
                        Start-Sleep -Milliseconds $retryDelay
                    }
                }
                $response.Dispose()
            }
            catch {
                $retryCount++
                if ($retryCount -ge $maxRetries) {
                    Write-Error "Error después de $maxRetries intentos: $_" -ErrorAction Continue
                }
                else {
                    Start-Sleep -Milliseconds $retryDelay
                }
            }
        }
    }
}
finally {
    $client.Dispose()
    $handler.Dispose()
}
