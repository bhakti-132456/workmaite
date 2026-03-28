@echo off
setlocal

echo [ WORKMAITE PRO // INITIALIZING WINDOWS BOOTSTRAP ]
echo.

:: 1. Start the State Manager (Background)
echo [SYS]: Starting State Manager...
start /b python core/state_manager.py

:: 2. Start the Deck Server (Background)
echo [SYS]: Starting Command Center...
start /b python deck/server.py

:: 3. Start the Main Tauri App
echo [SYS]: Starting Main AI Engine...
start /b npm run tauri dev

:: 4. Open the Command Center in Browser
echo [SYS]: Opening Local GUI...
timeout /t 5 >nul
start http://localhost:8080

echo.
echo [ WORKMAITE PRO // SYSTEM_READY ]
echo Keep this window open or minimized to maintain the AI bridge.
echo.
pause
