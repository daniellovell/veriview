@echo off
setlocal enabledelayedexpansion

REM Default to fix mode
set CHECK_MODE=false
set CHECKS_FAILED=0

REM Parse arguments
if "%1"=="--check" set CHECK_MODE=true

echo Running linting checks...

REM Rust checks
echo Running Rust format...
cd src-tauri
if "%CHECK_MODE%"=="true" (
    cargo fmt --all -- --check || set CHECKS_FAILED=1
) else (
    cargo fmt --all
)

echo Running Rust clippy check...
if "%CHECK_MODE%"=="true" (
    cargo clippy -- -D warnings || set CHECKS_FAILED=1
) else (
    cargo clippy --fix --allow-dirty -- -D warnings || echo Clippy fixes attempted
)
cd ..

REM Frontend checks
echo Running ESLint...
if "%CHECK_MODE%"=="true" (
    npm exec eslint "./src/**/*.{js,jsx,ts,tsx}" || set CHECKS_FAILED=1
) else (
    npm exec eslint "./src/**/*.{js,jsx,ts,tsx}" --fix || echo ESLint fixes attempted
)

echo Running TypeScript check...
npm exec tsc --noEmit || set CHECKS_FAILED=1

echo Running Prettier...
if "%CHECK_MODE%"=="true" (
    npm exec prettier --check "./src/**/*.{js,jsx,ts,tsx,css,html}" || set CHECKS_FAILED=1
) else (
    npm exec prettier --write "./src/**/*.{js,jsx,ts,tsx,css,html}" || echo Prettier fixes attempted
)

if "%CHECK_MODE%"=="true" (
    echo All linting checks completed!
    if !CHECKS_FAILED! equ 1 (
        echo Some checks failed. Please fix the issues or run without --check to auto-fix.
        exit /b 1
    )
) else (
    echo All linting fixes applied!
) 