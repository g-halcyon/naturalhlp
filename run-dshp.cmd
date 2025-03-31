@echo off
:: NHLP direct execution script for Windows
:: This script allows running .dshp files directly

:: Check if a file was provided
if "%~1"=="" (
    echo Error: No .dshp file specified.
    echo Usage: run-dshp filename.dshp [options]
    exit /b 1
)

:: Get the script directory
set "SCRIPT_DIR=%~dp0"

:: Find the NHLP interpreter - try several locations
set "NHLP_INTERPRETER="
if exist "%SCRIPT_DIR%nhlp.exe" (
    set "NHLP_INTERPRETER=%SCRIPT_DIR%nhlp.exe"
) else if exist "%SCRIPT_DIR%target\release\nhlp.exe" (
    set "NHLP_INTERPRETER=%SCRIPT_DIR%target\release\nhlp.exe"
) else (
    where nhlp.exe >nul 2>&1
    if %ERRORLEVEL% EQU 0 (
        set "NHLP_INTERPRETER=nhlp.exe"
    ) else (
        echo Error: Could not find the NHLP interpreter.
        echo Please build the interpreter with 'cargo build --release' first.
        exit /b 1
    )
)

:: Run the file directly
"%NHLP_INTERPRETER%" %* 