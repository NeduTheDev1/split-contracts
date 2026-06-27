# Clear Git credentials for GitHub and push

Set-Location "c:\Users\Admin\Desktop\StellarSplit\split-contracts"

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "Clearing cached Git credentials" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

# Method 1: Try using git credential-manager
Write-Host "Attempting to clear cached credentials..." -ForegroundColor Yellow
try {
    # Clear using git credential-manager if available
    & git credential-manager delete https://github.com 2>$null
} catch {
    Write-Host "Git credential-manager not available, using alternative method" -ForegroundColor Yellow
}

# Method 2: Use git credential reject
Write-Host "Using git credential reject..." -ForegroundColor Yellow
$credInput = @"
protocol=https
host=github.com

"@
$credInput | git credential reject 2>$null

Write-Host ""
Write-Host "Credentials cleared. Git will prompt for fresh credentials." -ForegroundColor Green
Write-Host ""

# Show current status
Write-Host "Current branch:" -ForegroundColor Cyan
git branch --show-current

Write-Host ""
Write-Host "Remote URL:" -ForegroundColor Cyan
git remote -v

Write-Host ""
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "Pushing branch to GitHub..." -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "When prompted, use account: johnsaviour56-ship-it" -ForegroundColor Yellow
Write-Host ""

# Attempt push
git push -u origin add-fallback-action-for-auto-resolve

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "================================================" -ForegroundColor Green
    Write-Host "SUCCESS! Branch pushed to GitHub" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "View at: https://github.com/johnsaviour56-ship-it/split-contracts/tree/add-fallback-action-for-auto-resolve"
} else {
    Write-Host ""
    Write-Host "================================================" -ForegroundColor Red
    Write-Host "Push failed" -ForegroundColor Red
    Write-Host "================================================" -ForegroundColor Red
    Write-Host ""
    Write-Host "Manual steps:" -ForegroundColor Yellow
    Write-Host "1. Open Windows Credential Manager (search in Start Menu)"
    Write-Host "2. Find 'git:https://github.com' entry and delete it"
    Write-Host "3. Run: git push -u origin add-fallback-action-for-auto-resolve"
    Write-Host "4. Enter credentials for: johnsaviour56-ship-it"
}

Write-Host ""
Read-Host "Press Enter to continue"
