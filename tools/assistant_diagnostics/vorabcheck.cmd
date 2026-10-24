@echo off
setlocal
python -B "%~dp0diagnose_preflight.py" %*
exit /b %errorlevel%
