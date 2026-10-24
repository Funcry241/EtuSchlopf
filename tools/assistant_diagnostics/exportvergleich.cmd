@echo off
setlocal
python -B "%~dp0compare_exports.py" %*
exit /b %errorlevel%
