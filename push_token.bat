@echo off
REM Push using Personal Access Token
REM Instructions: Create a PAT at https://github.com/settings/tokens with 'repo' scope

setlocal enabledelayedexpansion

cd /d "c:\Users\Admin\Desktop\StellarSplit\split-contracts"

echo.
echo ========================================================
echo PUSH WITH PERSONAL ACCESS TOKEN
echo ========================================================
echo.

REM Delete old credentials
echo Clearing old credentials...
cmdkey /delete:git:https://github.com >nul 2>&1
cmdkey /delete:https://github.com >nul 2>&1

echo Current branch:
git branch --show-current

echo.
echo To use this script:
echo 1. Create a PAT at https://github.com/settings/tokens
echo 2. Set environment variable: set GIT_CREDENTIALS_USERNAME=johnsaviour56-ship-it
echo 3. Set environment variable: set GIT_CREDENTIALS_PASSWORD=your_pat_token_here
echo 4. Run this script
echo.

REM Check if credentials are set
if not defined GIT_CREDENTIALS_USERNAME (
    echo ERROR: GIT_CREDENTIALS_USERNAME not set
    echo Please set it before running this script
    pause
    exit /b 1
)

if not defined GIT_CREDENTIALS_PASSWORD (
    echo ERROR: GIT_CREDENTIALS_PASSWORD not set
    echo Please set it before running this script
    pause
    exit /b 1
)

echo Using credentials for: !GIT_CREDENTIALS_USERNAME!
echo.
echo Attempting push...
echo.

REM Push with credentials from environment variables
git push -u origin add-fallback-action-for-auto-resolve

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================================
    echo SUCCESS! Branch pushed to GitHub!
    echo ========================================================
    echo.
    echo View at: https://github.com/johnsaviour56-ship-it/split-contracts/tree/add-fallback-action-for-auto-resolve
) else (
    echo.
    echo ========================================================
    echo PUSH FAILED
    echo ========================================================
)

echo.
pause
