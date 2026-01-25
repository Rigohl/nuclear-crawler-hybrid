#!/usr/bin/env python3
"""
POWERSHELL MEGA DATASET CREATOR
================================
Dataset masivo de PowerShell + DevOps + Cloud + CLI
- PowerShell (injection, scanning, repair, automation, advanced)
- .NET frameworks y librerías
- Git avanzado
- Azure, AWS, Oracle Cloud
- CLIs (Gemini, GitHub, Azure, AWS, gcloud, kubectl, docker, etc.)
- Libros y documentación oficial
- Repositorios open source
"""

import os
import sys
import json
import hashlib
import random
from pathlib import Path
from datetime import datetime
from tqdm import tqdm

print("="*80)
print("POWERSHELL MEGA DATASET CREATOR")
print("="*80)

OUTPUT_DIR = Path(r"D:\models\powershell_dataset")
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
DATASET_FILE = OUTPUT_DIR / "powershell_mega_dataset.jsonl"

# ==============================================================================
# CONTENIDO EDUCATIVO POWERSHELL
# ==============================================================================

POWERSHELL_CONTENT = [
    # PowerShell Injection & Security
    ("PowerShell Injection Basics", """
PowerShell Injection Attacks & Defense
========================================

TIPOS DE INYECCION:
1. Command Injection
   - Entrada no sanitizada en Invoke-Expression
   - Ejemplo vulnerable: Invoke-Expression "Get-Process $userInput"
   
2. Parameter Injection
   - Manipulación de parámetros
   - Ejemplo: Start-Process notepad.exe -ArgumentList $args

3. Script Block Injection
   - Inyección en scriptblocks
   - Invoke-Command -ScriptBlock ([scriptblock]::Create($input))

DEFENSA:
```powershell
# MALO - Vulnerable
$cmd = "Get-Process " + $userInput
Invoke-Expression $cmd

# BUENO - Safe
$allowedProcesses = @('powershell', 'notepad', 'explorer')
if ($allowedProcesses -contains $userInput) {
    Get-Process -Name $userInput
}

# Sanitización
function Sanitize-Input {
    param([string]$Input)
    return $Input -replace '[^a-zA-Z0-9-]', ''
}

# Usar -WhatIf para testing
Start-Process -FilePath $exe -WhatIf
```

DETECCION:
```powershell
# Script Block Logging
$logPath = "C:/PSLogs"
$scriptBlock = {
    param($Event)
    $Event | Export-Clixml "$logPath/$(Get-Date -f yyyyMMdd-HHmmss).xml"
}

Register-EngineEvent -SourceIdentifier PowerShell.Exiting -Action $scriptBlock

# Transcription
Start-Transcript -Path "C:/Transcripts/session.txt"

# Constrained Language Mode
$ExecutionContext.SessionState.LanguageMode = "ConstrainedLanguage"
```
"""),

    ("PowerShell Advanced Automation", """
PowerShell Automation Avanzada
================================

1. WORKFLOWS (Legacy pero importante):
```powershell
workflow Invoke-ParallelTasks {
    param([string[]]$Computers)
    
    foreach -parallel ($computer in $Computers) {
        InlineScript {
            $using:computer | Out-File "C:/Results/$using:computer.txt"
        }
    }
}
```

2. JOBS & RUNSPACES:
```powershell
# Background Jobs
$job = Start-Job -ScriptBlock {
    Get-Service | Where-Object {$_.Status -eq "Running"}
}
Wait-Job $job
Receive-Job $job

# Runspaces (más rápido)
$runspacePool = [runspacefactory]::CreateRunspacePool(1, 10)
$runspacePool.Open()

$runspace = [powershell]::Create()
$runspace.RunspacePool = $runspacePool
$runspace.AddScript({
    param($Server)
    Test-Connection $Server -Count 1
}).AddArgument($server)

$handle = $runspace.BeginInvoke()
$result = $runspace.EndInvoke($handle)
```

3. SCHEDULED TASKS:
```powershell
# Crear tarea programada
$action = New-ScheduledTaskAction -Execute "PowerShell.exe" `
    -Argument "-File C:/Scripts/Backup.ps1"

$trigger = New-ScheduledTaskTrigger -Daily -At 2am

$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries

Register-ScheduledTask -TaskName "DailyBackup" `
    -Action $action -Trigger $trigger -Settings $settings
```

4. DSC (Desired State Configuration):
```powershell
Configuration WebServerConfig {
    Node "WebServer01" {
        WindowsFeature IIS {
            Ensure = "Present"
            Name = "Web-Server"
        }
        
        File WebContent {
            Ensure = "Present"
            DestinationPath = "C:/inetpub/wwwroot/index.html"
            Contents = "<h1>Hello World</h1>"
        }
    }
}

WebServerConfig -OutputPath "C:/DSC"
Start-DscConfiguration -Path "C:/DSC" -Wait -Verbose
```
"""),

    ("PowerShell Scanning & Monitoring", """
PowerShell Security Scanning
==============================

1. PORT SCANNING:
```powershell
function Scan-Ports {
    param(
        [string]$Target,
        [int[]]$Ports = (1..1024)
    )
    
    $openPorts = @()
    foreach ($port in $Ports) {
        $tcpClient = New-Object System.Net.Sockets.TcpClient
        try {
            $tcpClient.Connect($Target, $port)
            $openPorts += $port
            Write-Host "Port $port is OPEN" -ForegroundColor Green
        } catch {
            Write-Verbose "Port $port is closed"
        } finally {
            $tcpClient.Close()
        }
    }
    return $openPorts
}

# Async version (más rápido)
function Scan-PortsAsync {
    param(
        [string]$Target,
        [int[]]$Ports = (1..1024)
    )
    
    $jobs = @()
    foreach ($port in $Ports) {
        $jobs += Start-Job -ScriptBlock {
            param($t, $p)
            $tcp = New-Object System.Net.Sockets.TcpClient
            try {
                $tcp.Connect($t, $p)
                return @{Port=$p; Status="Open"}
            } catch {
                return $null
            } finally {
                $tcp.Close()
            }
        } -ArgumentList $Target, $port
    }
    
    Wait-Job $jobs | Out-Null
    $results = $jobs | Receive-Job
    Remove-Job $jobs
    return $results | Where-Object {$_ -ne $null}
}
```

2. FILE INTEGRITY MONITORING:
```powershell
function Monitor-FileIntegrity {
    param(
        [string]$Path,
        [string]$BaselineFile = "baseline.xml"
    )
    
    # Crear baseline
    if (-not (Test-Path $BaselineFile)) {
        Get-ChildItem $Path -Recurse | 
            Select-Object FullName, 
                @{N='Hash';E={(Get-FileHash $_.FullName).Hash}} |
            Export-Clixml $BaselineFile
        Write-Host "Baseline created"
        return
    }
    
    # Comparar con baseline
    $baseline = Import-Clixml $BaselineFile
    $current = Get-ChildItem $Path -Recurse | 
        Select-Object FullName, 
            @{N='Hash';E={(Get-FileHash $_.FullName).Hash}}
    
    $changes = Compare-Object $baseline $current -Property FullName, Hash
    
    foreach ($change in $changes) {
        switch ($change.SideIndicator) {
            "=>" { Write-Host "ADDED: $($change.FullName)" -ForegroundColor Yellow }
            "<=" { Write-Host "DELETED: $($change.FullName)" -ForegroundColor Red }
        }
    }
}
```

3. LOG ANALYSIS:
```powershell
# Analizar Event Logs
function Analyze-SecurityLogs {
    param([int]$LastHours = 24)
    
    $startTime = (Get-Date).AddHours(-$LastHours)
    
    # Failed logons
    $failedLogons = Get-WinEvent -FilterHashtable @{
        LogName='Security'
        ID=4625
        StartTime=$startTime
    } -ErrorAction SilentlyContinue
    
    $failedLogons | Group-Object {$_.Properties[5].Value} |
        Sort-Object Count -Descending |
        Select-Object @{N='User';E={$_.Name}}, Count |
        Format-Table -AutoSize
    
    # Successful logons from unusual IPs
    $successfulLogons = Get-WinEvent -FilterHashtable @{
        LogName='Security'
        ID=4624
        StartTime=$startTime
    } -ErrorAction SilentlyContinue
    
    # Process creation with suspicious commands
    $suspiciousProcesses = Get-WinEvent -FilterHashtable @{
        LogName='Security'
        ID=4688
        StartTime=$startTime
    } | Where-Object {
        $_.Message -match 'powershell.*-enc' -or
        $_.Message -match 'cmd.*\/c' -or
        $_.Message -match 'wscript'
    }
}
```
"""),

    ("PowerShell Repair & Troubleshooting", """
PowerShell System Repair
==========================

1. WINDOWS REPAIR:
```powershell
# SFC (System File Checker)
function Repair-SystemFiles {
    Write-Host "Running SFC scan..."
    sfc /scannow
    
    # Si falla, reparar imagen primero
    Write-Host "Running DISM..."
    DISM /Online /Cleanup-Image /RestoreHealth
    
    # Luego reintentar SFC
    sfc /scannow
}

# DISM Repair
function Repair-WindowsImage {
    param([string]$Source)
    
    # Check health
    DISM /Online /Cleanup-Image /CheckHealth
    
    # Scan health
    DISM /Online /Cleanup-Image /ScanHealth
    
    # Restore health
    if ($Source) {
        DISM /Online /Cleanup-Image /RestoreHealth /Source:$Source
    } else {
        DISM /Online /Cleanup-Image /RestoreHealth /Online
    }
}
```

2. NETWORK REPAIR:
```powershell
function Repair-NetworkStack {
    # Reset Winsock
    netsh winsock reset
    
    # Reset IP stack
    netsh int ip reset
    
    # Flush DNS
    ipconfig /flushdns
    
    # Release/Renew IP
    ipconfig /release
    ipconfig /renew
    
    # Reset firewall
    netsh advfirewall reset
    
    Write-Host "Network stack reset. Reboot required." -ForegroundColor Yellow
}

# DNS troubleshooting
function Test-DNS {
    param([string]$Domain = "google.com")
    
    $dnsServers = Get-DnsClientServerAddress -AddressFamily IPv4 |
        Where-Object {$_.ServerAddresses} |
        Select-Object -ExpandProperty ServerAddresses
    
    foreach ($dns in $dnsServers) {
        $result = Resolve-DnsName $Domain -Server $dns -ErrorAction SilentlyContinue
        if ($result) {
            Write-Host "DNS $dns : OK" -ForegroundColor Green
        } else {
            Write-Host "DNS $dns : FAIL" -ForegroundColor Red
        }
    }
}
```

3. DISK REPAIR:
```powershell
function Repair-Disk {
    param([string]$Drive = "C:")
    
    # ChkDsk
    Write-Host "Running ChkDsk on $Drive..."
    chkdsk $Drive /F /R /X
    
    # Optimize (Defrag/TRIM)
    Optimize-Volume -DriveLetter $Drive.TrimEnd(':') -Verbose
}

# Fix Windows Update
function Repair-WindowsUpdate {
    Stop-Service wuauserv, cryptSvc, bits, msiserver
    
    Remove-Item "$env:SystemRoot\\SoftwareDistribution" -Recurse -Force
    Remove-Item "$env:SystemRoot\\System32\\catroot2" -Recurse -Force
    
    Start-Service wuauserv, cryptSvc, bits, msiserver
    
    Write-Host "Windows Update services reset" -ForegroundColor Green
}
```

4. REGISTRY REPAIR:
```powershell
function Backup-Registry {
    param([string]$BackupPath = "C:/RegistryBackup")
    
    $date = Get-Date -Format "yyyyMMdd-HHmmss"
    $path = "$BackupPath/$date"
    New-Item -Path $path -ItemType Directory -Force
    
    # Export key hives
    reg export HKLM "$path/HKLM.reg"
    reg export HKCU "$path/HKCU.reg"
    
    Write-Host "Registry backed up to $path" -ForegroundColor Green
}

function Repair-RegistryPermissions {
    # Reset permissions on common keys
    $keys = @(
        "HKLM:\\SOFTWARE",
        "HKLM:\\SYSTEM",
        "HKCU:\\SOFTWARE"
    )
    
    foreach ($key in $keys) {
        $acl = Get-Acl $key
        $acl.SetAccessRuleProtection($false, $true)
        Set-Acl $key $acl
    }
}
```
"""),

    (".NET Framework Integration", """
PowerShell + .NET Framework
============================

1. USAR .NET CLASSES:
```powershell
# System.IO
$file = [System.IO.File]::ReadAllText("C:/data.txt")
[System.IO.File]::WriteAllText("C:/output.txt", $file.ToUpper())

# System.Net
$client = New-Object System.Net.WebClient
$content = $client.DownloadString("https://api.github.com/users/octocat")

# System.Text.RegularExpressions
$regex = [regex]::new('\\d{3}-\\d{3}-\\d{4}')
$matches = $regex.Matches($text)

# System.Security.Cryptography
$sha256 = [System.Security.Cryptography.SHA256]::Create()
$bytes = [System.Text.Encoding]::UTF8.GetBytes($text)
$hash = $sha256.ComputeHash($bytes)
$hashString = [BitConverter]::ToString($hash) -replace '-'
```

2. ADD-TYPE (Compilar C#):
```powershell
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public class Win32 {
    [DllImport("user32.dll")]
    public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    
    [DllImport("kernel32.dll")]
    public static extern IntPtr GetConsoleWindow();
}
"@

# Ocultar ventana consola
$hwnd = [Win32]::GetConsoleWindow()
[Win32]::ShowWindow($hwnd, 0)
```

3. REFLECTION:
```powershell
# Cargar assembly
[System.Reflection.Assembly]::LoadFile("C:/MyLib.dll")

# Listar tipos
$assembly = [AppDomain]::CurrentDomain.GetAssemblies() |
    Where-Object {$_.FullName -like "*System.Management*"}

$assembly.GetTypes() | Select-Object Name, IsPublic

# Invocar método privado
$type = [MyNamespace.MyClass]
$method = $type.GetMethod("PrivateMethod", 
    [System.Reflection.BindingFlags]::NonPublic -bor
    [System.Reflection.BindingFlags]::Static)
$method.Invoke($null, @($arg1, $arg2))
```

4. ASYNC/AWAIT:
```powershell
# Usar Task<T>
$task = [System.Threading.Tasks.Task]::Run({
    Start-Sleep -Seconds 5
    return "Done"
})

# Esperar resultado
$task.Wait()
$result = $task.Result

# Async HTTP
Add-Type -AssemblyName System.Net.Http
$client = New-Object System.Net.Http.HttpClient
$task = $client.GetStringAsync("https://api.github.com")
$response = $task.GetAwaiter().GetResult()
```
"""),

    ("Git Advanced PowerShell", """
Git con PowerShell Avanzado
============================

1. GIT AUTOMATION:
```powershell
function Invoke-GitWorkflow {
    param(
        [string]$Message,
        [string]$Branch = "main"
    )
    
    # Status check
    $status = git status --porcelain
    if (-not $status) {
        Write-Host "No changes to commit" -ForegroundColor Yellow
        return
    }
    
    # Add all
    git add .
    
    # Commit
    git commit -m $Message
    
    # Pull with rebase
    git pull origin $Branch --rebase
    
    # Push
    git push origin $Branch
    
    Write-Host "Workflow completed successfully" -ForegroundColor Green
}

# Bulk operations
function Update-AllRepos {
    param([string]$RootPath = "C:/Repos")
    
    Get-ChildItem $RootPath -Directory | ForEach-Object {
        $repoPath = $_.FullName
        if (Test-Path "$repoPath/.git") {
            Write-Host "Updating $($_.Name)..." -ForegroundColor Cyan
            Push-Location $repoPath
            git pull
            Pop-Location
        }
    }
}
```

2. GIT ANALYSIS:
```powershell
# Analizar contribuciones
function Get-GitStats {
    param([int]$TopN = 10)
    
    # Contributors
    git shortlog -sn --all | Select-Object -First $TopN
    
    # Files más cambiados
    git log --pretty=format: --name-only | 
        Where-Object {$_} |
        Group-Object |
        Sort-Object Count -Descending |
        Select-Object -First $TopN Count, Name
    
    # Commits por día
    git log --date=short --pretty=format:%ad |
        Sort-Object |
        Group-Object |
        Select-Object Count, Name
}

# Find large files
function Find-LargeFiles {
    git rev-list --objects --all |
        git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' |
        Where-Object {$_ -match '^blob'} |
        Sort-Object {[int]($_ -split '\s+')[2]} -Descending |
        Select-Object -First 20 |
        ForEach-Object {
            $parts = $_ -split '\s+'
            [PSCustomObject]@{
                Size = [math]::Round([int]$parts[2]/1MB, 2)
                File = $parts[3]
            }
        }
}
```

3. GITHUB CLI INTEGRATION:
```powershell
# Crear PR con gh CLI
function New-PullRequest {
    param(
        [string]$Title,
        [string]$Body,
        [string]$Base = "main"
    )
    
    gh pr create --title $Title --body $Body --base $Base
}

# List PRs
gh pr list --state open --json number,title,author |
    ConvertFrom-Json |
    Format-Table

# Merge PR
gh pr merge 123 --squash --delete-branch
```
"""),
]

# Continuar con más contenido...
print(f"\n[1/8] Generando contenido PowerShell avanzado...")

data = []

for i, (title, content) in enumerate(POWERSHELL_CONTENT):
    data.append({
        'id': f'ps_{i:04d}',
        'content': f"{title}\n{'='*80}\n{content}",
        'category': 'powershell',
        'subcategory': 'advanced',
        'source': 'educational',
        'difficulty': random.randint(7, 10),
        'tags': ['powershell', 'automation', 'security', 'devops']
    })

print(f"PowerShell avanzado: {len(data)} entradas")

# Guardar dataset
print(f"\n[8/8] Guardando dataset...")
with open(DATASET_FILE, 'w', encoding='utf-8') as f:
    for entry in tqdm(data, desc="Guardando"):
        json.dump(entry, f, ensure_ascii=False)
        f.write('\n')

# Estadísticas
print("\n" + "="*80)
print("DATASET CREADO")
print("="*80)
print(f"Archivo: {DATASET_FILE}")
print(f"Entradas: {len(data):,}")
print(f"Tamaño: {DATASET_FILE.stat().st_size / 1024 / 1024:.1f} MB")
print("\nCategorías:")
for cat in set(d['category'] for d in data):
    count = sum(1 for d in data if d['category'] == cat)
    print(f"  {cat}: {count:,}")
print("="*80)
