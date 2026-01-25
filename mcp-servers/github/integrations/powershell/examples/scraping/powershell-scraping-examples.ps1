# PowerShell Scraping Examples - Ejemplos de scraping para entrenamiento
# Ejemplos completos de scraping en PowerShell
#Requires -Version 7.2

$ErrorActionPreference = 'Stop'

# =============================================================================
# 1. Scraping Básico con Invoke-WebRequest
# =============================================================================

function Invoke-BasicWebScraping {
    <#
    .SYNOPSIS
    Scraping básico usando Invoke-WebRequest
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Url
    )
    
    try {
        # Obtener contenido HTML
        $response = Invoke-WebRequest -Uri $Url -UseBasicParsing -TimeoutSec 30 -ErrorAction Stop
        
        # Extraer HTML
        $html = $response.Content
        
        # Extraer título usando regex
        if ($html -match '<title>(.*?)</title>') {
            $title = $matches[1]
        }
        
        # Extraer links
        $links = [regex]::Matches($html, '<a[^>]+href=["'']([^"'']+)["'']') | 
            ForEach-Object { $_.Groups[1].Value }
        
        return @{
            Title = $title
            Links = $links
            Content = $html
        }
    }
    catch {
        Write-Error "Error en scraping: $_"
        throw
    }
}

# =============================================================================
# 2. Scraping con Parsing HTML Avanzado
# =============================================================================

function Invoke-HtmlParsing {
    <#
    .SYNOPSIS
    Parsing HTML usando Select-String y regex
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Url
    )
    
    $response = Invoke-WebRequest -Uri $Url -UseBasicParsing
    $html = $response.Content
    
    # Extraer todos los elementos con clase específica
    $elements = [regex]::Matches($html, '<div[^>]*class=["'']([^"'']+)["'']') | 
        ForEach-Object { $_.Groups[1].Value }
    
    # Extraer texto de párrafos
    $paragraphs = [regex]::Matches($html, '<p[^>]*>(.*?)</p>', [System.Text.RegularExpressions.RegexOptions]::Singleline) | 
        ForEach-Object { $_.Groups[1].Value -replace '<[^>]+>', '' }
    
    # Extraer imágenes
    $images = [regex]::Matches($html, '<img[^>]+src=["'']([^"'']+)["'']') | 
        ForEach-Object { $_.Groups[1].Value }
    
    return @{
        Elements = $elements
        Paragraphs = $paragraphs
        Images = $images
    }
}

# =============================================================================
# 3. Scraping con HtmlAgilityPack (.NET)
# =============================================================================

function Invoke-HtmlAgilityPackScraping {
    <#
    .SYNOPSIS
    Scraping avanzado usando HtmlAgilityPack desde NuGet
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Url
    )
    
    # Instalar HtmlAgilityPack si no está disponible
    if (-not ([System.Management.Automation.PSTypeName]'HtmlAgilityPack.HtmlDocument').Type) {
        Write-Host "Instalando HtmlAgilityPack..." -ForegroundColor Yellow
        Install-Package HtmlAgilityPack -Scope CurrentUser -Force
        
        $packagePath = "$env:USERPROFILE\.nuget\packages\htmlagilitypack"
        $latestVersion = Get-ChildItem $packagePath | Sort-Object Name -Descending | Select-Object -First 1
        $dllPath = Join-Path $latestVersion.FullName "lib\netstandard2.0\HtmlAgilityPack.dll"
        
        Add-Type -Path $dllPath
    }
    
    $web = New-Object HtmlAgilityPack.HtmlWeb
    $doc = $web.Load($Url)
    
    # Extraer elementos por selector CSS
    $titles = $doc.DocumentNode.SelectNodes("//h1") | ForEach-Object { $_.InnerText }
    $links = $doc.DocumentNode.SelectNodes("//a[@href]") | ForEach-Object { $_.Attributes['href'].Value }
    
    return @{
        Titles = $titles
        Links = $links
        Document = $doc
    }
}

# =============================================================================
# 4. Scraping con Selenium (PowerShell + .NET)
# =============================================================================

function Invoke-SeleniumScraping {
    <#
    .SYNOPSIS
    Scraping avanzado con Selenium WebDriver
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Url,
        
        [string]$Browser = "Chrome"
    )
    
    # Instalar Selenium WebDriver
    if (-not (Get-Package -Name Selenium.WebDriver -ErrorAction SilentlyContinue)) {
        Install-Package Selenium.WebDriver -Scope CurrentUser -Force
    }
    
    $packagePath = "$env:USERPROFILE\.nuget\packages\selenium.webdriver"
    $latestVersion = Get-ChildItem $packagePath | Sort-Object Name -Descending | Select-Object -First 1
    $dllPath = Join-Path $latestVersion.FullName "lib\net48\WebDriver.dll"
    
    Add-Type -Path $dllPath
    
    # Crear driver
    $options = New-Object OpenQA.Selenium.Chrome.ChromeOptions
    $options.AddArgument("--headless")
    
    $driver = New-Object OpenQA.Selenium.Chrome.ChromeDriver($options)
    
    try {
        $driver.Navigate().GoToUrl($Url)
        
        # Esperar a que cargue
        Start-Sleep -Seconds 2
        
        # Extraer elementos
        $title = $driver.Title
        $links = $driver.FindElements([OpenQA.Selenium.By]::TagName("a")) | 
            ForEach-Object { $_.GetAttribute("href") }
        
        return @{
            Title = $title
            Links = $links
        }
    }
    finally {
        $driver.Quit()
    }
}

# =============================================================================
# 5. Scraping de APIs REST
# =============================================================================

function Invoke-RestApiScraping {
    <#
    .SYNOPSIS
    Obtener datos de APIs REST
    #>
    param(
        [Parameter(Mandatory)]
        [string]$ApiUrl,
        
        [hashtable]$Headers = @{},
        
        [string]$Method = "GET"
    )
    
    try {
        $params = @{
            Uri = $ApiUrl
            Method = $Method
            Headers = $Headers
            ContentType = "application/json"
        }
        
        $response = Invoke-RestMethod @params
        
        return $response
    }
    catch {
        Write-Error "Error en API request: $_"
        throw
    }
}

# =============================================================================
# 6. Scraping con HttpClient (.NET)
# =============================================================================

function Invoke-HttpClientScraping {
    <#
    .SYNOPSIS
    Scraping usando HttpClient de .NET
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Url
    )
    
    $client = [System.Net.Http.HttpClient]::new()
    
    try {
        $response = $client.GetAsync($Url).Result
        $content = $response.Content.ReadAsStringAsync().Result
        
        return $content
    }
    finally {
        $client.Dispose()
    }
}

# =============================================================================
# 7. Scraping con Playwright (PowerShell + Node.js)
# =============================================================================

function Invoke-PlaywrightScraping {
    <#
    .SYNOPSIS
    Scraping avanzado con Playwright (requiere Node.js)
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Url
    )
    
    $playwrightScript = @"
const { chromium } = require('playwright');

(async () => {
    const browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    await page.goto('$Url');
    
    const title = await page.title();
    const links = await page.$$eval('a', links => links.map(l => l.href));
    
    console.log(JSON.stringify({ title, links }));
    
    await browser.close();
})();
"@
    
    $scriptPath = Join-Path $env:TEMP "playwright_scrape_$([guid]::NewGuid().ToString('N')).js"
    $playwrightScript | Out-File -FilePath $scriptPath -Encoding UTF8
    
    try {
        $output = node $scriptPath 2>&1
        $result = $output | ConvertFrom-Json
        
        return $result
    }
    finally {
        Remove-Item $scriptPath -ErrorAction SilentlyContinue
    }
}

# =============================================================================
# Exportar funciones
# =============================================================================

Export-ModuleMember -Function @(
    'Invoke-BasicWebScraping',
    'Invoke-HtmlParsing',
    'Invoke-HtmlAgilityPackScraping',
    'Invoke-SeleniumScraping',
    'Invoke-RestApiScraping',
    'Invoke-HttpClientScraping',
    'Invoke-PlaywrightScraping'
)
