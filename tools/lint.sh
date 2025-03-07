#!/bin/bash

# Exit on error
set -e

# Default to fix mode
CHECK_MODE=false

# Track if any checks failed
CHECKS_FAILED=0

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --check) CHECK_MODE=true ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
done

echo "Running linting checks..."

# Rust checks
echo "Running Rust format..."
cd src-tauri
if [ "$CHECK_MODE" = true ]; then
    cargo fmt --all -- --check || CHECKS_FAILED=1
else
    cargo fmt --all
fi

echo "Running Rust clippy check..."
if [ "$CHECK_MODE" = true ]; then
    cargo clippy -- -D warnings || CHECKS_FAILED=1
else
    cargo clippy --fix --allow-dirty -- -D warnings || true
fi
cd ..

# Frontend checks
echo "Running ESLint..."
if [ "$CHECK_MODE" = true ]; then
    npx eslint "src/**/*.{js,jsx}" --max-warnings 0 || CHECKS_FAILED=1
else
    npx eslint "src/**/*.{js,jsx}" --fix || true
fi

echo "Running TypeScript check..."
# Use --pretty for better formatted output and --noEmit since we don't need the output files
npx tsc --noEmit --pretty || CHECKS_FAILED=1

echo "Running Prettier..."
if [ "$CHECK_MODE" = true ]; then
    # Use --loglevel warn to reduce verbosity
    npx prettier --log-level warn --check "src/**/*.{js,jsx,css,html}" || CHECKS_FAILED=1
else
    npx prettier --log-level warn --write "src/**/*.{js,jsx,css,html}" || true
fi

if [ "$CHECK_MODE" = true ]; then
    echo "All linting checks completed!"
    if [ $CHECKS_FAILED -eq 1 ]; then
        echo "Some checks failed. Please fix the issues or run without --check to auto-fix."
        exit 1
    fi
else
    echo "All linting fixes applied!"
fi
