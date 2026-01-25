# PowerShell .NET Frameworks - Ejemplos Completos
# Demostraciones prácticas de todos los frameworks
#Requires -Version 7.2

$ErrorActionPreference = 'Stop'

# =============================================================================
# 1. ASP.NET CORE - Crear API REST desde PowerShell
# =============================================================================

function New-AspNetCoreApi {
    param([string]$Name = "PowerShellApi")
    
    Write-Host "Creando ASP.NET Core API: $Name" -ForegroundColor Cyan
    
    # Crear proyecto
    dotnet new webapi -n $Name -o $Name
    
    # Agregar paquetes
    Push-Location $Name
    dotnet add package Swashbuckle.AspNetCore
    dotnet add package Microsoft.EntityFrameworkCore.InMemory
    
    # Crear controlador PowerShell
    $controllerCode = @"
using Microsoft.AspNetCore.Mvc;
using System.Management.Automation;

[ApiController]
[Route("api/[controller]")]
public class PowerShellController : ControllerBase {
    [HttpGet]
    public IActionResult Get() => Ok("PowerShell API Running!");
    
    [HttpPost("execute")]
    public IActionResult Execute([FromBody] ExecuteRequest request) {
        using var ps = PowerShell.Create();
        ps.AddScript(request.Script);
        var results = ps.Invoke();
        return Ok(new { Results = results.Select(r => r.ToString()).ToArray() });
    }
}

public class ExecuteRequest {
    public string Script { get; set; }
}
"@
    
    $controllerCode | Out-File -FilePath "Controllers/PowerShellController.cs" -Encoding UTF8
    Pop-Location
    
    Write-Host "✓ API creada. Ejecuta: cd $Name && dotnet run" -ForegroundColor Green
}

# =============================================================================
# 2. ENTITY FRAMEWORK CORE - Base de Datos
# =============================================================================

function New-EntityFrameworkProject {
    param([string]$Name = "EfApp")
    
    Write-Host "Creando proyecto Entity Framework: $Name" -ForegroundColor Cyan
    
    dotnet new console -n $Name -o $Name
    Push-Location $Name
    
    # Agregar EF Core
    dotnet add package Microsoft.EntityFrameworkCore.SqlServer
    dotnet add package Microsoft.EntityFrameworkCore.Design
    dotnet add package Microsoft.EntityFrameworkCore.Tools
    
    # Crear modelo
    $modelCode = @"
using Microsoft.EntityFrameworkCore;

public class User {
    public int Id { get; set; }
    public string Name { get; set; }
    public string Email { get; set; }
}

public class AppDbContext : DbContext {
    public DbSet<User> Users { get; set; }
    
    protected override void OnConfiguring(DbContextOptionsBuilder options) {
        options.UseSqlServer("Server=localhost;Database=PowerShellDb;Trusted_Connection=True;");
    }
}
"@
    
    $modelCode | Out-File -FilePath "AppDbContext.cs" -Encoding UTF8
    
    # Crear migración
    dotnet ef migrations add InitialCreate
    dotnet ef database update
    
    Pop-Location
    Write-Host "✓ Proyecto EF Core creado" -ForegroundColor Green
}

# =============================================================================
# 3. WPF - Interfaz Gráfica
# =============================================================================

function New-WpfApplication {
    Write-Host "Creando aplicación WPF..." -ForegroundColor Cyan
    
    Add-Type -AssemblyName PresentationFramework, PresentationCore, WindowsBase
    
    $window = New-Object System.Windows.Window
    $window.Title = "PowerShell WPF Application"
    $window.Width = 800
    $window.Height = 600
    
    # Grid principal
    $grid = New-Object System.Windows.Controls.Grid
    $grid.ColumnDefinitions.Add((New-Object System.Windows.Controls.ColumnDefinition))
    $grid.ColumnDefinitions.Add((New-Object System.Windows.Controls.ColumnDefinition))
    
    # ListView con datos
    $listView = New-Object System.Windows.Controls.ListView
    $listView.View = New-Object System.Windows.Controls.GridView
    
    $nameColumn = New-Object System.Windows.Controls.GridViewColumn
    $nameColumn.Header = "Name"
    $nameColumn.DisplayMemberBinding = New-Object System.Windows.Data.Binding("Name")
    $listView.View.Columns.Add($nameColumn)
    
    $valueColumn = New-Object System.Windows.Controls.GridViewColumn
    $valueColumn.Header = "Value"
    $valueColumn.DisplayMemberBinding = New-Object System.Windows.Data.Binding("Value")
    $listView.View.Columns.Add($valueColumn)
    
    # Datos
    $items = @(
        [PSCustomObject]@{Name="CPU"; Value="45%"},
        [PSCustomObject]@{Name="Memory"; Value="60%"},
        [PSCustomObject]@{Name="Disk"; Value="30%"}
    )
    $listView.ItemsSource = $items
    
    # Botón
    $button = New-Object System.Windows.Controls.Button
    $button.Content = "Refresh"
    $button.Height = 30
    $button.Margin = New-Object System.Windows.Thickness(10)
    $button.Add_Click({
        $listView.ItemsSource = @(
            [PSCustomObject]@{Name="CPU"; Value="$((Get-Counter '\Processor(_Total)\% Processor Time').CounterSamples[0].CookedValue)%"},
            [PSCustomObject]@{Name="Memory"; Value="$([math]::Round((Get-Counter '\Memory\Available MBytes').CounterSamples[0].CookedValue / 1MB, 2)) GB"}
        )
    })
    
    # StackPanel
    $stackPanel = New-Object System.Windows.Controls.StackPanel
    $stackPanel.Children.Add($listView)
    $stackPanel.Children.Add($button)
    
    $grid.Children.Add($stackPanel)
    [System.Windows.Controls.Grid]::SetColumn($stackPanel, 0)
    
    $window.Content = $grid
    $window.ShowDialog()
}

# =============================================================================
# 4. ML.NET - Machine Learning
# =============================================================================

function New-MLNetProject {
    param([string]$Name = "MLApp")
    
    Write-Host "Creando proyecto ML.NET: $Name" -ForegroundColor Cyan
    
    dotnet new console -n $Name -o $Name
    Push-Location $Name
    
    dotnet add package Microsoft.ML
    dotnet add package Microsoft.ML.FastTree
    
    $mlCode = @"
using Microsoft.ML;
using Microsoft.ML.Data;

public class HouseData {
    [LoadColumn(0)]
    public float Size { get; set; }
    [LoadColumn(1)]
    public float Price { get; set; }
}

public class Prediction {
    [ColumnName("Score")]
    public float Price { get; set; }
}

class Program {
    static void Main() {
        var mlContext = new MLContext();
        
        // Datos de entrenamiento
        var trainingData = new[] {
            new HouseData { Size = 1000, Price = 200000 },
            new HouseData { Size = 1500, Price = 300000 },
            new HouseData { Size = 2000, Price = 400000 }
        };
        
        var data = mlContext.Data.LoadFromEnumerable(trainingData);
        
        var pipeline = mlContext.Transforms.Concatenate("Features", "Size")
            .Append(mlContext.Regression.Trainers.Sdca());
        
        var model = pipeline.Fit(data);
        
        var predictionEngine = mlContext.Model.CreatePredictionEngine<HouseData, Prediction>(model);
        
        var prediction = predictionEngine.Predict(new HouseData { Size = 2500 });
        System.Console.WriteLine($"Predicted price: {prediction.Price:C}");
    }
}
"@
    
    $mlCode | Out-File -FilePath "Program.cs" -Encoding UTF8 -Force
    Pop-Location
    
    Write-Host "✓ Proyecto ML.NET creado" -ForegroundColor Green
}

# =============================================================================
# 5. SIGNALR - Real-time Communication
# =============================================================================

function New-SignalRProject {
    param([string]$Name = "SignalRApp")
    
    Write-Host "Creando proyecto SignalR: $Name" -ForegroundColor Cyan
    
    dotnet new webapi -n $Name -o $Name
    Push-Location $Name
    
    dotnet add package Microsoft.AspNetCore.SignalR
    
    $hubCode = @"
using Microsoft.AspNetCore.SignalR;

public class ChatHub : Hub {
    public async Task SendMessage(string user, string message) {
        await Clients.All.SendAsync("ReceiveMessage", user, message);
    }
    
    public async Task SendToGroup(string group, string message) {
        await Clients.Group(group).SendAsync("ReceiveMessage", Context.ConnectionId, message);
    }
    
    public async Task JoinGroup(string group) {
        await Groups.AddToGroupAsync(Context.ConnectionId, group);
    }
}
"@
    
    New-Item -ItemType Directory -Path "Hubs" -Force | Out-Null
    $hubCode | Out-File -FilePath "Hubs/ChatHub.cs" -Encoding UTF8
    
    Pop-Location
    Write-Host "✓ Proyecto SignalR creado" -ForegroundColor Green
}

# =============================================================================
# 6. BLAZOR - Web UI
# =============================================================================

function New-BlazorProject {
    param(
        [string]$Name = "BlazorApp",
        [ValidateSet("Server", "WebAssembly")]
        [string]$Type = "Server"
    )
    
    Write-Host "Creando proyecto Blazor $Type: $Name" -ForegroundColor Cyan
    
    if ($Type -eq "Server") {
        dotnet new blazorserver -n $Name -o $Name
    }
    else {
        dotnet new blazorwasm -n $Name -o $Name
    }
    
    Write-Host "✓ Proyecto Blazor creado. Ejecuta: cd $Name && dotnet run" -ForegroundColor Green
}

# =============================================================================
# 7. AZURE INTEGRATION
# =============================================================================

function New-AzureIntegration {
    param([string]$Name = "AzureApp")
    
    Write-Host "Creando proyecto con Azure SDK: $Name" -ForegroundColor Cyan
    
    dotnet new console -n $Name -o $Name
    Push-Location $Name
    
    dotnet add package Azure.Storage.Blobs
    dotnet add package Azure.Identity
    dotnet add package Azure.Messaging.ServiceBus
    
    $azureCode = @"
using Azure.Storage.Blobs;
using Azure.Identity;
using System;

class Program {
    static async System.Threading.Tasks.Task Main() {
        var credential = new DefaultAzureCredential();
        var client = new BlobServiceClient(
            new Uri("https://YOUR_STORAGE_ACCOUNT.blob.core.windows.net"),
            credential
        );
        
        var container = client.GetBlobContainerClient("mycontainer");
        await container.CreateIfNotExistsAsync();
        
        var blob = container.GetBlobClient("myfile.txt");
        var content = System.Text.Encoding.UTF8.GetBytes("Hello from PowerShell + Azure!");
        await blob.UploadAsync(new System.IO.MemoryStream(content));
        
        Console.WriteLine("File uploaded to Azure Blob Storage!");
    }
}
"@
    
    $azureCode | Out-File -FilePath "Program.cs" -Encoding UTF8 -Force
    Pop-Location
    
    Write-Host "✓ Proyecto Azure creado" -ForegroundColor Green
}

# =============================================================================
# 8. GRPC - High Performance RPC
# =============================================================================

function New-GrpcProject {
    param([string]$Name = "GrpcApp")
    
    Write-Host "Creando proyecto gRPC: $Name" -ForegroundColor Cyan
    
    dotnet new grpc -n $Name -o $Name
    
    Write-Host "✓ Proyecto gRPC creado" -ForegroundColor Green
}

# =============================================================================
# 9. MAUI - Multi-platform Apps
# =============================================================================

function New-MauiProject {
    param([string]$Name = "MauiApp")
    
    Write-Host "Creando proyecto MAUI: $Name" -ForegroundColor Cyan
    
    # Instalar workload
    dotnet workload install maui
    
    dotnet new maui -n $Name -o $Name
    
    Write-Host "✓ Proyecto MAUI creado" -ForegroundColor Green
}

# =============================================================================
# 10. GRAPHQL con HotChocolate
# =============================================================================

function New-GraphQLProject {
    param([string]$Name = "GraphQLApp")
    
    Write-Host "Creando proyecto GraphQL: $Name" -ForegroundColor Cyan
    
    dotnet new webapi -n $Name -o $Name
    Push-Location $Name
    
    dotnet add package HotChocolate.AspNetCore
    dotnet add package HotChocolate.Data.EntityFramework
    
    Pop-Location
    Write-Host "✓ Proyecto GraphQL creado" -ForegroundColor Green
}

# =============================================================================
# DEMO COMPLETO
# =============================================================================

function Show-AllFrameworksDemo {
    Write-Host @"
╔════════════════════════════════════════════════════════════════╗
║     PowerShell .NET Frameworks - Demo Completo               ║
╚════════════════════════════════════════════════════════════════╝
"@ -ForegroundColor Cyan
    
    Write-Host "`nFrameworks disponibles:" -ForegroundColor Yellow
    Write-Host "  1. ASP.NET Core API" -ForegroundColor White
    Write-Host "  2. Entity Framework Core" -ForegroundColor White
    Write-Host "  3. WPF Application" -ForegroundColor White
    Write-Host "  4. ML.NET" -ForegroundColor White
    Write-Host "  5. SignalR" -ForegroundColor White
    Write-Host "  6. Blazor" -ForegroundColor White
    Write-Host "  7. Azure Integration" -ForegroundColor White
    Write-Host "  8. gRPC" -ForegroundColor White
    Write-Host "  9. MAUI" -ForegroundColor White
    Write-Host "  10. GraphQL" -ForegroundColor White
    
    Write-Host "`nPara crear un proyecto:" -ForegroundColor Cyan
    Write-Host "  New-AspNetCoreApi -Name MyApi" -ForegroundColor Gray
    Write-Host "  New-EntityFrameworkProject -Name MyDb" -ForegroundColor Gray
    Write-Host "  New-WpfApplication" -ForegroundColor Gray
    Write-Host "  New-MLNetProject -Name MyML" -ForegroundColor Gray
}

# Si se ejecuta directamente
if ($MyInvocation.InvocationName -ne '.') {
    Show-AllFrameworksDemo
}
