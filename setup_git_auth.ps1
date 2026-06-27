# Script to set up Git authentication for push
# This script helps configure credentials for the GitHub repository

$repoPath = "c:\Users\Admin\Desktop\StellarSplit\split-contracts"
Set-Location $repoPath

# Check current git config
Write-Host "Current Git Configuration:" -ForegroundColor Cyan
git config --list | Select-String "user|credential"

Write-Host "`nGit Remote:" -ForegroundColor Cyan
git remote -v

Write-Host "`nCurrent Branch:" -ForegroundColor Cyan
git branch --show-current

Write-Host "`nTo fix the authentication issue, you have options:" -ForegroundColor Yellow
Write-Host "1. Use SSH instead of HTTPS"
Write-Host "2. Update HTTPS credentials with a Personal Access Token (PAT)"
Write-Host "3. Update stored credentials in Windows Credential Manager"
Write-Host ""

Write-Host "Option 1: Switch to SSH (Recommended)" -ForegroundColor Green
Write-Host "If you have SSH keys configured for GitHub:"
Write-Host "  git remote set-url origin git@github.com:johnsaviour56-ship-it/split-contracts.git"
Write-Host "  git push -u origin add-fallback-action-for-auto-resolve"
Write-Host ""

Write-Host "Option 2: Use Personal Access Token" -ForegroundColor Green
Write-Host "Create a PAT at: https://github.com/settings/tokens"
Write-Host "Then when git prompts, enter:"
Write-Host "  Username: your-github-username"
Write-Host "  Password: your-github-personal-access-token"
Write-Host ""

Write-Host "Option 3: Update Windows Credential Manager" -ForegroundColor Green
Write-Host "Search for 'Credential Manager' in Windows and:"
Write-Host "  1. Find 'git:https://github.com'"
Write-Host "  2. Edit it with correct credentials"
Write-Host "  3. Then run: git push -u origin add-fallback-action-for-auto-resolve"
