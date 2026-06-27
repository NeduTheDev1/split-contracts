@echo off
echo.
echo ========================================================
echo PUSHING BRANCH - johnsaviour56-ship-it
echo ========================================================
echo.

cd /d "c:\Users\Admin\Desktop\StellarSplit\split-contracts"

echo Temporarily disabling credential caching...
git config --global credential.helper ""

echo.
echo Current branch:
git branch --show-current

echo.
echo ========================================================
echo PUSHING NOW - Enter johnsaviour56-ship-it credentials
echo ========================================================
echo.

git push -u origin add-fallback-action-for-auto-resolve

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================================
    echo SUCCESS! Branch is now on GitHub
    echo ========================================================
    echo.
    echo Re-enabling credential helper...
    git config --global credential.helper manager
    echo.
    echo View your branch at:
    echo https://github.com/johnsaviour56-ship-it/split-contracts/tree/add-fallback-action-for-auto-resolve
    echo.
) else (
    echo.
    echo ========================================================
    echo PUSH FAILED
    echo ========================================================
    echo.
    echo Restoring credential helper...
    git config --global credential.helper manager
    echo.
    echo Try manually:
    echo 1. Open Credential Manager in Windows
    echo 2. Delete git:https://github.com entry
    echo 3. Run this script again
)

echo.
pause
