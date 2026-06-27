@echo off
REM Script to push using SSH authentication
REM This script switches the remote to SSH and attempts to push

echo.
echo =========================================
echo Git SSH Push Helper
echo =========================================
echo.

cd /d "c:\Users\Admin\Desktop\StellarSplit\split-contracts"

echo Checking current branch...
git branch --show-current

echo.
echo Switching remote to SSH...
git remote set-url origin git@github.com:johnsaviour56-ship-it/split-contracts.git

echo.
echo Remote URL after change:
git remote -v

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
    echo Troubleshooting:
    echo - Make sure SSH keys are configured: ssh -T git@github.com
    echo - If SSH fails, use a Personal Access Token instead
    echo - See PUSH_INSTRUCTIONS.md for more details
)

echo.
pause
