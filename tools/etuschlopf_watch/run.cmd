@echo off
setlocal
cd /d "%~dp0"
set "ETUSCHLOPF_WATCH_TARGET=%~dp0..\..\target\etuschlopf_watch_runtime"
cargo build --quiet --locked --profile strict --target-dir "%ETUSCHLOPF_WATCH_TARGET%"
if errorlevel 1 exit /b %errorlevel%
"%ETUSCHLOPF_WATCH_TARGET%\strict\etuschlopf-watch.exe"
exit /b %errorlevel%
