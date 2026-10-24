@echo off
setlocal
python -B -m unittest discover -s "%~dp0tests" -v
exit /b %errorlevel%
