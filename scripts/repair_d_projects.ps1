<#
.SYNOPSIS
  Scan a root path (default D:\\) and D:\\spark.bak for Maven projects (pom.xml),
  list repair commands and optionally execute them.

.DESCRIPTION
  This script finds all pom.xml files under the provided root and under D:\\spark.bak,
  prints a plan to run Maven on each project and can optionally execute the commands
  when -Execute switch is passed. It also reports Java helper scripts in the repo
  (e.g. `NuclearProjectsScanner.java`) and how to compile/run them.

.PARAMETER RootPath
  Root path to scan for Maven projects. Defaults to D:\\

.PARAMETER Execute
  If present, actually runs the Maven build commands. Otherwise the script only
  prints the planned commands (safe default).

.EXAMPLE
  .\\repair_d_projects.ps1 -RootPath D:\\projects -Execute
#>

[CmdletBinding()]
param(
    [string]$RootPath = 'D:\\',
    [switch]$Execute
)

function Find-PomFiles {
    param([string]$PathToScan)
    try {
        Get-ChildItem -Path $PathToScan -Filter pom.xml -Recurse -ErrorAction SilentlyContinue -Force | Select-Object -ExpandProperty FullName
    } catch {
        return @()
    }
}

Write-Host "Scanning root: $RootPath"

$allPoms = @()

if (Test-Path 'D:\\spark.bak') {
    Write-Host "Including D:\\\\spark.bak"
    $allPoms += Find-PomFiles -PathToScan 'D:\\spark.bak'
}

$allPoms += Find-PomFiles -PathToScan $RootPath

$allPoms = $allPoms | Sort-Object -Unique

if (-not $allPoms) {
    Write-Host "No pom.xml files found under $RootPath or D:\\spark.bak"
} else {
    Write-Host "Found $($allPoms.Count) pom.xml files:" -ForegroundColor Green
    $allPoms | ForEach-Object { Write-Host "  $_" }

    Write-Host "`nPlanned repair commands (won't run unless -Execute is provided):`n" -ForegroundColor Cyan
    foreach ($pom in $allPoms) {
        $projDir = Split-Path -Parent $pom
        $cmd = "mvn -f `"$pom`" -DskipTests clean install"
        Write-Host "Project: $projDir"
        Write-Host "  $cmd`n"
        if ($Execute) {
            Write-Host "Executing Maven for $projDir..." -ForegroundColor Yellow
            Push-Location $projDir
            try {
                & mvn -f "$pom" -DskipTests clean install
            } catch {
                Write-Host "Maven failed for $projDir: $_" -ForegroundColor Red
            }
            Pop-Location
        }
    }
}

# Show Java helper scripts in repo (informational)
Write-Host "`nRepository Java helpers (informational):`n" -ForegroundColor Cyan
if (Test-Path "$PSScriptRoot\\..\\NuclearProjectsScanner.java") {
    $scanner = Resolve-Path "$PSScriptRoot\\..\\NuclearProjectsScanner.java"
    Write-Host "  $scanner"
    Write-Host "Compile: javac -d out \"$scanner\""
    Write-Host "Run: java -cp out NuclearProjectsScanner <args>"
} else {
    Write-Host "  No top-level Java helper found at NuclearProjectsScanner.java"
}

Write-Host "`nReport: To actually perform repairs, re-run with -Execute. Be careful: this will invoke Maven builds.`n" -ForegroundColor Yellow
