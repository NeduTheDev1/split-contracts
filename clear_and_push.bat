@echo off
echo ================================================
echo Clearing cached Git credentials and pushing
echo ================================================
echo.

cd /d "c:\Users\Admin\Desktop\StellarSplit\split-contracts"

echo Step 1: Removing cached GitHub credentials from Git Credential Manager...
echo.

REM Clear the cached credential
git credential-manager delete https://github.com
git credential reject <<EOF
protocol=https
host=github.com

EOF

echo.
echo Step 2: Verifying current branch...
git branch --show-current

echo.
echo Step 3: Attempting to push (you may be prompted for credentials)...
echo Use your johnsaviour56-ship-it account credentials when prompted
echo.

git push -u origin add-fallback-action-for-auto-resolve

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ================================================
    echo SUCCESS! Branch pushed to GitHub
    echo ================================================
    echo.
    echo View at: https://github.com/johnsaviour56-ship-it/split-contracts/tree/add-fallback-action-for-auto-resolve
) else (
    echo.
    echo ================================================
    echo Push failed. Try opening Windows Credential Manager manually
    echo ================================================
    echo.
    echo Search for "Credential Manager" in Windows Start Menu
    echo Find "git:https://github.com" and delete it
    echo Then run: git push -u origin add-fallback-action-for-auto-resolve
)

echo.
pause
