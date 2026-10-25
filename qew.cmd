@echo off
rem shortcut: runs qew from source on Python 3.14 (qew asks 3.15), offline, no GitHub sync. Install qew properly once 3.15 is out.
setlocal
set QEW_OFFLINE=1
set PYTHONPATH=C:\Users\Lavender\Documents\qew-openplush\src
py -3.14 -m qew %*
