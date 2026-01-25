# AI Shell Skills Module
# Skills especializadas para GitHub MCP operations

$ErrorActionPreference = 'Stop'

function New-GitHubAnalysisSkill {
    <#
    .SYNOPSIS
    Crea skill para análisis de repositorios GitHub
    #>
    [CmdletBinding()]
    param()
    
    if (Get-Command New-AIShellSkill -ErrorAction SilentlyContinue) {
        Import-Module .\AIShell-Advanced.psm1 -ErrorAction SilentlyContinue
        
        New-AIShellSkill `
            -Name "github-repo-analysis" `
            -Description "Analyze GitHub repositories using MCP tools" `
            -Instructions @"
When asked to analyze a GitHub repository:
1. Use GitHub MCP tools to get repository information
2. Get issues, pull requests, and workflow status
3. Provide comprehensive analysis including:
   - Repository health metrics
   - Open issues and PRs
   - Recent activity
   - Workflow status
4. Use PowerShell cmdlets from GitHubMcp module when appropriate
"@ `
            -Script {
                param($owner, $repo)
                
                Write-Host "Analyzing repository: $owner/$repo" -ForegroundColor Cyan
                
                # Usar funciones del módulo si están disponibles
                if (Get-Command Get-GitHubRepositoryAnalysis -ErrorAction SilentlyContinue) {
                    return Get-GitHubRepositoryAnalysis -Owner $owner -Repo $repo
                }
                else {
                    Write-Warning "GitHubMcp module not loaded"
                }
            }
        
        Write-Host "✓ GitHub Analysis Skill created" -ForegroundColor Green
    }
    else {
        Write-Warning "AIShell-Advanced module not loaded"
    }
}

function New-GitHubWorkflowSkill {
    <#
    .SYNOPSIS
    Crea skill para gestión de workflows GitHub
    #>
    [CmdletBinding()]
    param()
    
    if (Get-Command New-AIShellSkill -ErrorAction SilentlyContinue) {
        Import-Module .\AIShell-Advanced.psm1 -ErrorAction SilentlyContinue
        
        New-AIShellSkill `
            -Name "github-workflow-management" `
            -Description "Manage GitHub Actions workflows" `
            -Instructions @"
When asked about GitHub workflows:
1. Use GitHub MCP tools to list workflow runs
2. Check workflow status and results
3. Provide workflow execution history
4. Help troubleshoot failed workflows
5. Use Get-GitHubWorkflowRuns for detailed information
"@ `
            -Script {
                param($owner, $repo, $workflowId)
                
                if (Get-Command Get-GitHubWorkflowRuns -ErrorAction SilentlyContinue) {
                    return Get-GitHubWorkflowRuns -Owner $owner -Repo $repo -WorkflowId $workflowId
                }
            }
        
        Write-Host "✓ GitHub Workflow Skill created" -ForegroundColor Green
    }
}

function New-GitHubIssueManagementSkill {
    <#
    .SYNOPSIS
    Crea skill para gestión de issues GitHub
    #>
    [CmdletBinding()]
    param()
    
    if (Get-Command New-AIShellSkill -ErrorAction SilentlyContinue) {
        Import-Module .\AIShell-Advanced.psm1 -ErrorAction SilentlyContinue
        
        New-AIShellSkill `
            -Name "github-issue-management" `
            -Description "Manage GitHub issues efficiently" `
            -Instructions @"
When working with GitHub issues:
1. List issues using Get-GitHubIssues
2. Filter by state, labels, assignees
3. Create issues with New-GitHubIssue
4. Provide issue summaries and trends
5. Help prioritize issues based on labels and assignees
"@ `
            -Script {
                param($owner, $repo, $action, $params)
                
                switch ($action) {
                    "list" {
                        if (Get-Command Get-GitHubIssues -ErrorAction SilentlyContinue) {
                            return Get-GitHubIssues -Owner $owner -Repo $repo @params
                        }
                    }
                    "create" {
                        if (Get-Command New-GitHubIssue -ErrorAction SilentlyContinue) {
                            return New-GitHubIssue -Owner $owner -Repo $repo @params
                        }
                    }
                }
            }
        
        Write-Host "✓ GitHub Issue Management Skill created" -ForegroundColor Green
    }
}

function Install-AllGitHubSkills {
    <#
    .SYNOPSIS
    Instala todas las skills de GitHub
    #>
    [CmdletBinding()]
    param()
    
    Write-Host "Installing GitHub skills for AI Shell..." -ForegroundColor Cyan
    
    New-GitHubAnalysisSkill
    New-GitHubWorkflowSkill
    New-GitHubIssueManagementSkill
    
    Write-Host "`n✓ All GitHub skills installed" -ForegroundColor Green
    Write-Host "  Skills are now available in AI Shell" -ForegroundColor Yellow
}

Export-ModuleMember -Function @(
    'New-GitHubAnalysisSkill',
    'New-GitHubWorkflowSkill',
    'New-GitHubIssueManagementSkill',
    'Install-AllGitHubSkills'
)
