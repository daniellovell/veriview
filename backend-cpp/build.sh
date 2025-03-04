#!/bin/bash

echo "Building Veriview backend..."

# Check if VCPKG_ROOT is set
if [ -z "$VCPKG_ROOT" ]; then
    echo "VCPKG_ROOT environment variable not set."
    echo "Please set it to your vcpkg installation directory or edit this script."
    echo "For example: export VCPKG_ROOT=~/vcpkg"
    exit 1
fi

# Create build directory if it doesn't exist
mkdir -p build

# Build with CMake
cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE=$VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake -DCMAKE_BUILD_TYPE=Release
if [ $? -ne 0 ]; then
    echo "CMake configuration failed!"
    exit $?
fi

cmake --build . --config Release
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit $?
fi

echo
echo "Build successful! The executable is in build/veriview"
echo 