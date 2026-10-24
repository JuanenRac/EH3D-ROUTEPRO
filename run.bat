@echo off
rem Runs ROUTEPRO. Builds it first if needed. Extra arguments are passed on:
rem   run.bat [FILE] [--lang en^|es^|it^|de^|fr^|pt^|zh^|ja] [--data DIR]
cd /d "%~dp0"
if not exist "target\release\routepro.exe" (
    set NOPAUSE=1
    call "%~dp0build.bat"
    set NOPAUSE=
)
if not exist "target\release\routepro.exe" (
    echo routepro.exe was not built.
    goto :end
)
set DATA_ARGS=--data "%~dp0testdata"
for %%A in (%*) do if /i "%%A"=="--data" set DATA_ARGS=
"target\release\routepro.exe" %DATA_ARGS% %*
:end
if not defined NOPAUSE pause
