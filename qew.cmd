@echo off
rem Runs qew from its source folder; the GitHub token comes from the gh login.
setlocal
if not defined GITHUB_TOKEN for /f "delims=" %%t in ('gh auth token') do set GITHUB_TOKEN=%%t
set PYTHONPATH=C:\Users\Lavender\Documents\qew\src
py -3.15 -m qew %*
