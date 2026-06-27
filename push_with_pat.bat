@echo off
REM Script to push using Personal Access Token (PAT)
REM This will prompt for username and password (use PAT as password)

echo.
echo =========================================
echo Git PAT (Personal Access Token) Push Helper
echo =========================================
echo.

cd /d "c:\Users\Admin\Desktop\StellarSplit\split-contracts"

echo Checking current branch...
git branch --show-current

echo.
echo Remote URL:
git remote -v

echo.
echo Instructions:
echo 1. Get your Personal Access Token from: https://github.com/settings/tokens
echo 2. When prompted for password, paste the token
echo.
echo Attempting to push branch: add-fallback-action-for-auto-resolve
echo.

git push -u origin add-fallback-action-for-auto-resolve

if %ERRORLEVEL% EQU 0 (
    echo.
    echo =========================================
    echo SUCCESS! Branch pushed to GitHub
    echo =========================================
    echo.
    echo View your branch at:
    echo https://github.com/johnsaviour56-ship-it/split-contracts/tree/add-fallback-action-for-auto-resolve
) else (
    echo.
    echo =========================================
    echo FAILED! Push did not succeed
    echo =========================================
    echo.
    echo Possible issues:
    echo - Invalid credentials
    echo - Token doesn't have repo scope
    echo - Network connectivity
    echo.
    echo See PUSH_INSTRUCTIONS.md for more details
)

echo.
pause
