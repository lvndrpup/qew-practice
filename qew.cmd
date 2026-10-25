@echo off
rem Runs qew from its source folder, offline: nothing syncs to GitHub.
setlocal
set QEW_OFFLINE=1
set PYTHONPATH=C:\Users\Lavender\Documents\qew-openplush\src
py -3.15 -m qew %*
