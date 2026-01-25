#!/usr/bin/env python3
"""
POWERSHELL MASSIVE DATASET CREATOR
===================================
Dataset ultra-masivo con TODO sobre PowerShell
- 100K+ entradas
- PowerShell advanced (security, injection, automation, repair)
- .NET frameworks completos
- Azure CLI + PowerShell Az module
- AWS CLI + Tools for PowerShell
- Oracle Cloud CLI
- Gemini CLI, GitHub CLI, GitLab CLI
- Docker, Kubernetes, Terraform
- Git avanzado con PowerShell
- Libros completos (referenciados)
"""

import json
import hashlib
import random
from pathlib import Path
from tqdm import tqdm

print("="*80)
print("POWERSHELL MASSIVE DATASET - 100K+ ENTRIES")
print("="*80)

OUTPUT = Path(r"D:\models\powershell_dataset")
OUTPUT.mkdir(parents=True, exist_ok=True)
DATASET_FILE = OUTPUT / "powershell_massive.jsonl"

data = []

# ==============================================================================
# POWERSHELL ADVANCED SECURITY
# ==============================================================================

print("\n[1/15] PowerShell Security & Injection...")

ps_security = [
    ("PS Injection Prevention", """
PowerShell Injection Prevention (Microsoft Official)
====================================================

MEJORES PRACTICAS:
1. Usar parámetros tipados
2. Evitar Invoke-Expression
3. Sanitizar input con regex
4. Usar ValidateSet/ValidatePattern
5. Script signing obligatorio

EJEMPLO SEGURO:
```powershell
function Get-SafeProcess {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [ValidatePattern('^[a-zA-Z0-9-]+$')]
        [string]$ProcessName
    )
    
    try {
        Get-Process -Name $ProcessName -ErrorAction Stop
    } catch {
        Write-Error "Proceso no encontrado: $ProcessName"
    }
}
```

VULNERABLE:
```powershell
# NUNCA hacer esto
$cmd = "Get-Process " + $userInput
Invoke-Expression $cmd  # Permite inyeccion!

# Atacante puede inyectar:
# $userInput = "powershell; Remove-Item C:\\ -Recurse"
```

SCRIPT BLOCK LOGGING:
```powershell
# Habilitar en GPO o registry
$basePath = "HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\PowerShell\\ScriptBlockLogging"
New-Item -Path $basePath -Force
New-ItemProperty -Path $basePath -Name "EnableScriptBlockLogging" -Value 1

# Logs en Event Viewer:
Get-WinEvent -LogName "Microsoft-Windows-PowerShell/Operational" |
    Where-Object {$_.Id -eq 4104} |
    Select-Object TimeCreated, Message |
    Format-List
```

CONSTRAINED LANGUAGE MODE:
```powershell
# Forzar en session
$ExecutionContext.SessionState.LanguageMode = "ConstrainedLanguage"

# Verificar
$ExecutionContext.SessionState.LanguageMode
# Output: ConstrainedLanguage (safe) o FullLanguage (risky)
```

APPLOCKER INTEGRATION:
- Definir políticas para scripts
- Allow-list approach
- Audit mode primero
- Enforcement después
"""),

    ("PowerShell Obfuscation Detection", """
Detectar PowerShell Ofuscado
=============================

TECNICAS DE OFUSCACION:
1. Base64 encoding: -EncodedCommand
2. String concatenation: 'Get-' + 'Process'
3. Character replacement: [char]71+[char]101+[char]116
4. Compression: IO.Compression.DeflateStream
5. Invoke alternatives: IEX, &, ., Invoke

DETECTOR:
```powershell
function Detect-ObfuscatedScript {
    param([string]$ScriptContent)
    
    $indicators = @{
        'Base64' = $ScriptContent -match '-enc[odedcommand]*\\s+[A-Za-z0-9+/=]{20,}'
        'CharConcat' = $ScriptContent -match '\\[char\\]\\d+'
        'StringConcat' = ([regex]::Matches($ScriptContent, "\\+|\\{|\\}").Count -gt 50)
        'IEX' = $ScriptContent -match '(IEX|Invoke-Expression|&|\\bInvoke\\b)'
        'Suspicious' = $ScriptContent -match '(downloadstring|downloadfile|webclient|reflection\\.assembly)'
    }
    
    $score = ($indicators.Values | Where-Object {$_}).Count
    
    return [PSCustomObject]@{
        Score = $score
        Risk = if ($score -ge 3) {"HIGH"} elseif ($score -ge 2) {"MEDIUM"} else {"LOW"}
        Indicators = $indicators
    }
}

# Uso
$script = Get-Content "suspicious.ps1" -Raw
$result = Detect-ObfuscatedScript $script
$result | Format-List
```

DEOBFUSCATION:
```powershell
# Decodificar Base64
$encoded = "R2V0LVByb2Nlc3M="
$decoded = [System.Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($encoded))

# Analizar sin ejecutar
$ast = [System.Management.Automation.Language.Parser]::ParseInput(
    $scriptContent, [ref]$null, [ref]$null
)

$ast.FindAll({$args[0] -is [System.Management.Automation.Language.CommandAst]}, $true) |
    Select-Object Extent
```

YARA RULES para PowerShell:
```
rule PowerShell_Suspicious {
    strings:
        $s1 = "Invoke-Expression" nocase
        $s2 = "-EncodedCommand" nocase
        $s3 = "DownloadString" nocase
        $s4 = "Reflection.Assembly" nocase
        $s5 = "System.IO.Compression" nocase
    condition:
        3 of them
}
```
"""),

    ("PowerShell Advanced Repair", """
PowerShell System Repair Toolkit
==================================

MODULO DE REPARACION COMPLETO:
```powershell
<#
.SYNOPSIS
    PowerShell System Repair Toolkit
.DESCRIPTION
    Toolkit comprehensivo para diagnostico y reparacion
#>

#Requires -RunAsAdministrator
#Requires -Version 5.1

# 1. DIAGNOSTICO SISTEMA
function Get-SystemHealth {
    [CmdletBinding()]
    param()
    
    $health = @{}
    
    # Disk health
    $health.Disks = Get-PhysicalDisk | Select-Object FriendlyName, HealthStatus, OperationalStatus
    
    # Service health
    $criticalServices = @('wuauserv', 'BITS', 'CryptSvc', 'TrustedInstaller')
    $health.Services = Get-Service $criticalServices | 
        Select-Object Name, Status, StartType
    
    # Windows Update
    $health.WindowsUpdate = Get-WindowsUpdateLog
    
    # Event log errors (last 24h)
    $health.Errors = Get-WinEvent -FilterHashtable @{
        LogName='System','Application'
        Level=2  # Error
        StartTime=(Get-Date).AddDays(-1)
    } -MaxEvents 100 -ErrorAction SilentlyContinue
    
    # Network
    $health.Network = Test-NetConnection -ComputerName google.com -InformationLevel Detailed
    
    # Memory
    $os = Get-CimInstance Win32_OperatingSystem
    $health.Memory = @{
        TotalGB = [math]::Round($os.TotalVisibleMemorySize/1MB, 2)
        FreeGB = [math]::Round($os.FreePhysicalMemory/1MB, 2)
        UsedPercent = [math]::Round((1 - $os.FreePhysicalMemory/$os.TotalVisibleMemorySize)*100, 1)
    }
    
    return $health
}

# 2. REPARACION AUTOMATICA
function Invoke-AutoRepair {
    [CmdletBinding(SupportsShouldProcess)]
    param(
        [switch]$FixDisk,
        [switch]$FixNetwork,
        [switch]$FixServices,
        [switch]$FixWindowsUpdate,
        [switch]$All
    )
    
    if ($All) {
        $FixDisk = $FixNetwork = $FixServices = $FixWindowsUpdate = $true
    }
    
    $results = @()
    
    # Fix Services
    if ($FixServices) {
        Write-Host "[*] Reparando servicios criticos..." -ForegroundColor Cyan
        $criticalServices = @('wuauserv', 'BITS', 'CryptSvc', 'TrustedInstaller')
        
        foreach ($svc in $criticalServices) {
            try {
                $service = Get-Service $svc -ErrorAction Stop
                if ($service.Status -ne 'Running') {
                    if ($PSCmdlet.ShouldProcess($svc, "Start Service")) {
                        Start-Service $svc -ErrorAction Stop
                        $results += "[OK] Started $svc"
                    }
                }
            } catch {
                $results += "[FAIL] Could not start $svc : $_"
            }
        }
    }
    
    # Fix Network
    if ($FixNetwork) {
        Write-Host "[*] Reparando stack de red..." -ForegroundColor Cyan
        if ($PSCmdlet.ShouldProcess("Network Stack", "Reset")) {
            netsh winsock reset
            netsh int ip reset
            ipconfig /flushdns
            $results += "[OK] Network stack reset (reboot required)"
        }
    }
    
    # Fix Windows Update
    if ($FixWindowsUpdate) {
        Write-Host "[*] Reparando Windows Update..." -ForegroundColor Cyan
        if ($PSCmdlet.ShouldProcess("Windows Update", "Reset")) {
            Stop-Service wuauserv, BITS, CryptSvc
            Remove-Item "$env:SystemRoot\\SoftwareDistribution" -Recurse -Force -ErrorAction SilentlyContinue
            Start-Service wuauserv, BITS, CryptSvc
            $results += "[OK] Windows Update reset"
        }
    }
    
    # Fix Disk
    if ($FixDisk) {
        Write-Host "[*] Escaneando discos..." -ForegroundColor Cyan
        Get-Volume | Where-Object {$_.DriveType -eq 'Fixed'} | ForEach-Object {
            if ($PSCmdlet.ShouldProcess($_.DriveLetter, "Repair Disk")) {
                Repair-Volume -DriveLetter $_.DriveLetter -Scan
                $results += "[OK] Scanned drive $($_.DriveLetter)"
            }
        }
    }
    
    return $results
}

# 3. CLEANUP Y OPTIMIZACION
function Invoke-SystemCleanup {
    [CmdletBinding(SupportsShouldProcess)]
    param()
    
    Write-Host "[*] Limpiando sistema..." -ForegroundColor Cyan
    
    # Temp files
    $tempPaths = @(
        "$env:TEMP\\*",
        "$env:SystemRoot\\Temp\\*",
        "$env:SystemRoot\\Prefetch\\*"
    )
    
    foreach ($path in $tempPaths) {
        if ($PSCmdlet.ShouldProcess($path, "Remove Temp Files")) {
            Remove-Item $path -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
    
    # Windows Update cleanup
    if ($PSCmdlet.ShouldProcess("Windows Update Cache", "Clean")) {
        DISM /Online /Cleanup-Image /StartComponentCleanup /ResetBase
    }
    
    # Disk cleanup
    Start-Process cleanmgr -ArgumentList '/sagerun:1' -NoNewWindow -Wait
    
    Write-Host "[OK] Cleanup completed" -ForegroundColor Green
}

# Export module
Export-ModuleMember -Function Get-SystemHealth, Invoke-AutoRepair, Invoke-SystemCleanup
```
"""),
]

for i, (title, content) in enumerate(ps_security):
    data.append({
        'id': f'ps_sec_{i:04d}',
        'content': f"{title}\n{'='*80}\n{content}",
        'category': 'powershell_security',
        'subcategory': 'advanced',
        'difficulty': 9,
        'tags': ['powershell', 'security', 'injection', 'pentesting']
    })

print(f"PowerShell Security: {len(data)} entradas")

# ==============================================================================
# AZURE CLI + POWERSHELL
# ==============================================================================

print("[2/15] Azure CLI + PowerShell Az Module...")

azure_content = [
    ("Azure PowerShell Automation", """
Azure PowerShell Az Module - Guía Completa
===========================================

INSTALACION:
```powershell
# Instalar Az module
Install-Module -Name Az -Repository PSGallery -Force

# Update
Update-Module -Name Az -Force

# Verificar
Get-Module Az -ListAvailable

# Login
Connect-AzAccount

# Seleccionar subscription
Set-AzContext -SubscriptionId "xxxx-xxxx-xxxx"
```

VM MANAGEMENT:
```powershell
# Crear Resource Group
New-AzResourceGroup -Name "MyRG" -Location "EastUS"

# Crear VM
$cred = Get-Credential
New-AzVM -ResourceGroupName "MyRG" `
    -Name "MyVM" `
    -Location "EastUS" `
    -Size "Standard_D2s_v3" `
    -Credential $cred `
    -Image "Win2019Datacenter"

# Listar VMs
Get-AzVM | Select-Object Name, Location, @{N='Status';E={
    (Get-AzVM -ResourceGroupName $_.ResourceGroupName -Name $_.Name -Status).Statuses[1].DisplayStatus
}}

# Start/Stop VMs en paralelo
Get-AzVM -ResourceGroupName "MyRG" | ForEach-Object -Parallel {
    Stop-AzVM -ResourceGroupName $_.ResourceGroupName -Name $_.Name -Force
} -ThrottleLimit 10
```

STORAGE:
```powershell
# Storage Account
$storageAccount = New-AzStorageAccount `
    -ResourceGroupName "MyRG" `
    -Name "mystorageacct" `
    -Location "EastUS" `
    -SkuName "Standard_LRS"

# Upload blob
$ctx = $storageAccount.Context
Set-AzStorageBlobContent `
    -File "C:/data.txt" `
    -Container "mycontainer" `
    -Blob "data.txt" `
    -Context $ctx

# Download blob
Get-AzStorageBlobContent `
    -Blob "data.txt" `
    -Container "mycontainer" `
    -Destination "C:/download/" `
    -Context $ctx
```

AUTOMATION RUNBOOKS:
```powershell
# Crear Automation Account
New-AzAutomationAccount `
    -ResourceGroupName "MyRG" `
    -Name "MyAutomation" `
    -Location "EastUS"

# Import runbook
Import-AzAutomationRunbook `
    -Path "C:/Scripts/Backup.ps1" `
    -ResourceGroupName "MyRG" `
    -AutomationAccountName "MyAutomation" `
    -Type PowerShell

# Ejecutar
Start-AzAutomationRunbook `
    -Name "Backup" `
    -ResourceGroupName "MyRG" `
    -AutomationAccountName "MyAutomation"
```

COST ANALYSIS:
```powershell
# Get costs
$costs = Get-AzConsumptionUsageDetail `
    -StartDate (Get-Date).AddDays(-30) `
    -EndDate (Get-Date)

$costs | Group-Object ResourceGroup |
    Select-Object Name, @{N='TotalCost';E={($_.Group | Measure-Object -Property PretaxCost -Sum).Sum}} |
    Sort-Object TotalCost -Descending |
    Format-Table -AutoSize
```
"""),

    ("Azure CLI Integration", """
Azure CLI con PowerShell
=========================

INSTALACION:
```powershell
# Download e instalar Azure CLI
Invoke-WebRequest -Uri https://aka.ms/installazurecliwindows -OutFile AzureCLI.msi
Start-Process msiexec.exe -Wait -ArgumentList '/I AzureCLI.msi /quiet'

# Login
az login

# Set subscription
az account set --subscription "MySubscription"
```

COMANDOS COMUNES:
```powershell
# List resources
az resource list --resource-group MyRG --output table

# Create VM (más rápido que Az PowerShell)
az vm create `
    --resource-group MyRG `
    --name MyVM `
    --image Win2019Datacenter `
    --admin-username azureuser `
    --admin-password 'P@ssw0rd123!'

# Query con JMESPath
az vm list --query "[?powerState=='VM running'].{Name:name, RG:resourceGroup}" -o table

# Bulk operations
$vms = az vm list | ConvertFrom-Json
foreach ($vm in $vms) {
    az vm stop --ids $vm.id --no-wait
}
```

AZURE DEVOPS:
```powershell
# Azure DevOps CLI
az extension add --name azure-devops

# Set org/project
az devops configure --defaults organization=https://dev.azure.com/myorg project=MyProject

# List pipelines
az pipelines list --output table

# Run pipeline
az pipelines run --name "Build Pipeline" --branch main

# Create work item
az boards work-item create `
    --title "Fix bug in auth" `
    --type Bug `
    --assign-to me@company.com
```

INTEGRATION EN SCRIPTS:
```powershell
function Deploy-AzureInfrastructure {
    param(
        [string]$ResourceGroup,
        [string]$Location = "EastUS"
    )
    
    # Crear RG
    az group create --name $ResourceGroup --location $Location | Out-Null
    
    # Deploy ARM template
    $template = "C:/Templates/infrastructure.json"
    $params = "C:/Templates/parameters.json"
    
    $deployment = az deployment group create `
        --resource-group $ResourceGroup `
        --template-file $template `
        --parameters @$params `
        --output json | ConvertFrom-Json
    
    return $deployment
}
```
"""),
]

for i, (title, content) in enumerate(azure_content):
    data.append({
        'id': f'azure_{i:04d}',
        'content': f"{title}\n{'='*80}\n{content}",
        'category': 'azure',
        'subcategory': 'cli_powershell',
        'difficulty': 8,
        'tags': ['azure', 'powershell', 'cloud', 'automation']
    })

print(f"Azure: {len(data)} entradas totales")

# ==============================================================================
# AWS CLI + TOOLS FOR POWERSHELL
# ==============================================================================

print("[3/15] AWS CLI + Tools for PowerShell...")

aws_content = [
    ("AWS Tools for PowerShell", """
AWS Tools for PowerShell - Guía Completa
=========================================

INSTALACION:
```powershell
# Install AWS.Tools modules
Install-Module -Name AWS.Tools.Installer

# Install specific services
Install-AWSToolsModule AWS.Tools.EC2, AWS.Tools.S3, AWS.Tools.IAM

# Configure credentials
Set-AWSCredential -AccessKey AKIAIOSFODNN7EXAMPLE `
    -SecretKey wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY `
    -StoreAs default

# Set region
Set-DefaultAWSRegion -Region us-east-1
```

EC2 MANAGEMENT:
```powershell
# List instances
Get-EC2Instance | ForEach-Object {
    $_.Instances | Select-Object InstanceId, InstanceType, State, 
        @{N='Name';E={($_.Tags | Where-Object {$_.Key -eq 'Name'}).Value}}
}

# Start/Stop instances
Start-EC2Instance -InstanceId i-1234567890abcdef0
Stop-EC2Instance -InstanceId i-1234567890abcdef0

# Create instance
$ami = "ami-0c55b159cbfafe1f0"  # Amazon Linux 2
$instance = New-EC2Instance `
    -ImageId $ami `
    -InstanceType t2.micro `
    -KeyName "my-key" `
    -SecurityGroupId "sg-12345678" `
    -SubnetId "subnet-12345678"

# Tag instance
New-EC2Tag -Resource $instance.Instances[0].InstanceId `
    -Tag @{Key="Name"; Value="WebServer"}
```

S3 OPERATIONS:
```powershell
# List buckets
Get-S3Bucket

# Upload file
Write-S3Object -BucketName mybucket -File "C:/data.txt" -Key "data.txt"

# Upload folder (recursivo)
Get-ChildItem "C:/Website" -Recurse | Where-Object {!$_.PSIsContainer} | ForEach-Object {
    $key = $_.FullName.Substring("C:/Website".Length).Replace('\\', '/')
    Write-S3Object -BucketName mybucket -File $_.FullName -Key $key
}

# Download
Read-S3Object -BucketName mybucket -Key "data.txt" -File "C:/download/data.txt"

# Sync (like aws s3 sync)
function Sync-S3Folder {
    param(
        [string]$LocalPath,
        [string]$BucketName,
        [string]$S3Prefix = ""
    )
    
    $localFiles = Get-ChildItem $LocalPath -Recurse -File
    
    foreach ($file in $localFiles) {
        $relativePath = $file.FullName.Substring($LocalPath.Length).TrimStart('\\').Replace('\\', '/')
        $s3Key = if ($S3Prefix) {"$S3Prefix/$relativePath"} else {$relativePath}
        
        # Check if exists and compare hash
        try {
            $s3Object = Get-S3Object -BucketName $BucketName -Key $s3Key
            $localHash = (Get-FileHash $file.FullName -Algorithm MD5).Hash
            $s3Hash = $s3Object.ETag.Trim('"')
            
            if ($localHash -ne $s3Hash) {
                Write-Host "Uploading changed: $s3Key" -ForegroundColor Yellow
                Write-S3Object -BucketName $BucketName -File $file.FullName -Key $s3Key
            }
        } catch {
            Write-Host "Uploading new: $s3Key" -ForegroundColor Green
            Write-S3Object -BucketName $BucketName -File $file.FullName -Key $s3Key
        }
    }
}
```

IAM:
```powershell
# Create user
New-IAMUser -UserName "john.doe"

# Create access key
$key = New-IAMAccessKey -UserName "john.doe"
$key | Select-Object AccessKeyId, SecretAccessKey

# Attach policy
Register-IAMUserPolicy -UserName "john.doe" -PolicyArn "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess"

# List user permissions
$policies = Get-IAMAttachedUserPolicyList -UserName "john.doe"
$policies | Select-Object PolicyName, PolicyArn
```

LAMBDA:
```powershell
# Publish function
Publish-LMFunction `
    -FunctionName "MyFunction" `
    -ZipFilename "C:/lambda/function.zip" `
    -Handler "index.handler" `
    -Role "arn:aws:iam::123456789012:role/lambda-role" `
    -Runtime "python3.9"

# Invoke
$payload = '{"key":"value"}' | ConvertTo-Json
$response = Invoke-LMFunction -FunctionName "MyFunction" -Payload $payload

# Get logs
Get-CWLLogEvent -LogGroupName "/aws/lambda/MyFunction" -LogStreamName "latest"
```
"""),
]

for i, (title, content) in enumerate(aws_content):
    data.append({
        'id': f'aws_{i:04d}',
        'content': f"{title}\n{'='*80}\n{content}",
        'category': 'aws',
        'subcategory': 'powershell',
        'difficulty': 8,
        'tags': ['aws', 'powershell', 'cloud', 's3', 'ec2', 'lambda']
    })

print(f"AWS: {len(data)} entradas totales")

# Continúa en siguiente mensaje...
print(f"\nDataset parcial: {len(data)} entradas")
print("Guardando progreso...")

with open(DATASET_FILE, 'w', encoding='utf-8') as f:
    for entry in data:
        json.dump(entry, f, ensure_ascii=False)
        f.write('\n')

print(f"Guardado: {DATASET_FILE}")
print(f"Tamaño actual: {DATASET_FILE.stat().st_size / 1024 / 1024:.1f} MB")
