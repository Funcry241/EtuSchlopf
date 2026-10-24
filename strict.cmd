@echo off
setlocal EnableExtensions
call "%~dp0run.cmd" --strict %*
exit /b %errorlevel%
