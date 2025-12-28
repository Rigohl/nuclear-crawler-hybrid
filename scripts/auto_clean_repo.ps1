# Auto Clean Repository Script
# Automates the removal of large files from git history

param(
    [string]$RepoPath = $PWD.Path
)

Write-Host "Starting automatic repository cleanup..." -ForegroundColor Green

# Change to repo directory
Set-Location $RepoPath

# Check if git repo
if (!(Test-Path ".git")) {
    Write-Host "Error: Not a git repository" -ForegroundColor Red
    exit 1
}

# Install git-filter-repo if not present
if (!(Get-Command git-filter-repo -ErrorAction SilentlyContinue)) {
    Write-Host "Installing git-filter-repo..." -ForegroundColor Yellow
    pip install git-filter-repo
}

# Find large files in git history (>50MB)
Write-Host "Finding large files in git history..." -ForegroundColor Yellow
$largeFiles = git rev-list --objects --all | git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' | Where-Object { $_ -match '^blob' } | ForEach-Object {
    $parts = $_ -split ' '
    $size = [long]$parts[2]
    $path = $parts[3]
    if ($size -gt 50MB) {
        $path
    }
} | Select-Object -Unique

if ($largeFiles.Count -eq 0) {
    Write-Host "No large files found in history. Repository is clean." -ForegroundColor Green
    exit 0
}

Write-Host "Found $($largeFiles.Count) large files in history:" -ForegroundColor Yellow
$largeFiles | ForEach-Object { Write-Host "  $_" }

# Backup current branch
$currentBranch = git branch --show-current
$backupBranch = "backup-$(Get-Date -Format 'yyyyMMdd-HHmmss')"
Write-Host "Creating backup branch: $backupBranch" -ForegroundColor Yellow
git branch $backupBranch

# Remove large files from history
Write-Host "Removing large files from git history..." -ForegroundColor Yellow
$largeFiles | Out-File -FilePath "large_files_to_remove.txt" -Encoding UTF8
git filter-repo --force --invert-paths --paths-from-file=large_files_to_remove.txt

# Clean up
Remove-Item "large_files_to_remove.txt" -ErrorAction SilentlyContinue

# Clean up
Remove-Item "large_files.txt" -ErrorAction SilentlyContinue

# Force push (warning: this rewrites history)
Write-Host "Force pushing cleaned repository..." -ForegroundColor Yellow
git push origin $currentBranch --force

Write-Host "Repository cleanup completed!" -ForegroundColor Green
Write-Host "Backup branch created: $backupBranch" -ForegroundColor Cyan
