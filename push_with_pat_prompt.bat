@echo off
setlocal enabledelayedexpansion

cd /d "c:\Users\Admin\Desktop\StellarSplit\split-contracts"

echo.
echo ========================================================
echo PUSH WITH PERSONAL ACCESS TOKEN
echo ========================================================
echo.
echo This script will push the branch using your Personal Access Token
echo.

echo Step 1: Create a PAT if you don't have one
echo   Go to: https://github.com/settings/tokens
echo   - Click "Generate new token (classic)"
echo   - Select scope: repo
echo   - Copy the token (ghp_...)
echo.

set /p TOKEN="Enter your Personal Access Token (ghp_...): "

if "!TOKEN!"=="" (
    echo ERROR: No token provided
    pause
    exit /b 1
)

echo.
echo Clearing old credentials...
git config --global --unset credential.helper >nul 2>&1
cmdkey /delete:git:https://github.com >nul 2>&1
cmdkey /delete:https://github.com >nul 2>&1

echo.
echo Current branch:
git branch --show-current

echo.
echo ========================================================
echo PUSHING BRANCH
echo ========================================================
echo.

git push https://johnsaviour56-ship-it:!TOKEN!@github.com/johnsaviour56-ship-it/split-contracts.git add-fallback-action-for-auto-resolve:refs/heads/add-fallback-action-for-auto-resolve

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================================
    echo SUCCESS! Branch pushed to GitHub!
    echo ========================================================
    echo.
    echo Restoring credential helper...
    git config --global credential.helper manager
    echo.
    echo View at: https://github.com/johnsaviour56-ship-it/split-contracts/tree/add-fallback-action-for-auto-resolve
) else (
    echo.
    echo ========================================================
    echo PUSH FAILED
    echo ========================================================
    echo.
    echo Possible reasons:
    echo - Invalid token
    echo - Token doesn't have 'repo' scope
    echo - Token has expired
    echo.
    echo Restoring credential helper...
    git config --global credential.helper manager
)

echo.
pause
