# Push script with credential prompt
Set-Location "c:\Users\Admin\Desktop\StellarSplit\split-contracts"

Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "Git Push with Credential Prompt" -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""

# Disable credential helper temporarily
Write-Host "Disabling credential cache..." -ForegroundColor Yellow
$env:GIT_TERMINAL_PROMPT = "1"
git config --global --unset credential.helper

Write-Host ""
Write-Host "Current branch:" -ForegroundColor Cyan
git branch --show-current

Write-Host ""
Write-Host "Remote URL:" -ForegroundColor Cyan
git remote -v | Select-String "push"

Write-Host ""
Write-Host "========================================================" -ForegroundColor Yellow
Write-Host "PUSHING - Git will prompt for credentials" -ForegroundColor Yellow  
Write-Host "Use account: johnsaviour56-ship-it" -ForegroundColor Yellow
Write-Host "========================================================" -ForegroundColor Yellow
Write-Host ""

# Attempt push
$pushResult = git push -u origin add-fallback-action-for-auto-resolve 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "========================================================" -ForegroundColor Green
    Write-Host "SUCCESS! Branch pushed to GitHub!" -ForegroundColor Green
    Write-Host "========================================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Restoring credential helper..." -ForegroundColor Yellow
    git config --global credential.helper manager
    Write-Host ""
    Write-Host "View at: https://github.com/johnsaviour56-ship-it/split-contracts/tree/add-fallback-action-for-auto-resolve"
} else {
    Write-Host ""
    Write-Host "========================================================" -ForegroundColor Red
    Write-Host "Push failed" -ForegroundColor Red
    Write-Host "========================================================" -ForegroundColor Red
    Write-Host ""
    Write-Host "Error:" -ForegroundColor Red
    Write-Host $pushResult
    Write-Host ""
    Write-Host "Restoring credential helper..." -ForegroundColor Yellow
    git config --global credential.helper manager
    Write-Host ""
    Write-Host "You may need to manually clear credentials in Credential Manager"
}

Write-Host ""
Write-Host "Press any key to continue..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
