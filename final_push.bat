@echo off
echo.
echo ========================================================
echo FINAL PUSH ATTEMPT
echo ========================================================
echo.

cd /d "c:\Users\Admin\Desktop\StellarSplit\split-contracts"

echo Step 1: Switch remote back to HTTPS...
git remote set-url origin https://github.com/johnsaviour56-ship-it/split-contracts.git

echo.
echo Step 2: Clear credential helper temporarily...
git config --global --unset credential.helper

echo.
echo Step 3: Attempting push without credential caching...
echo This will prompt for credentials - use johnsaviour56-ship-it account
echo.

git push -u origin add-fallback-action-for-auto-resolve

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================================
    echo SUCCESS! Branch pushed to GitHub!
    echo ========================================================
    echo.
    echo Restoring credential helper...
    git config --global credential.helper manager
    echo.
    echo View your branch at:
    echo https://github.com/johnsaviour56-ship-it/split-contracts/tree/add-fallback-action-for-auto-resolve
) else (
    echo.
    echo ========================================================
    echo Push failed - trying alternative method
    echo ========================================================
    echo.
    echo Restoring credential helper...
    git config --global credential.helper manager
)

echo.
pause
