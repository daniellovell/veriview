@echo off
setlocal

echo Building Veriview backend...

:: Check if VCPKG_ROOT is set
if "%VCPKG_ROOT%"=="" (
    echo VCPKG_ROOT environment variable not set.
    echo Please set it to your vcpkg installation directory or edit this script.
    echo For example: set VCPKG_ROOT=C:\vcpkg
    exit /b 1
)

:: Create build directory if it doesn't exist
if not exist build mkdir build

:: Build with CMake
cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE=%VCPKG_ROOT%\scripts\buildsystems\vcpkg.cmake -DCMAKE_BUILD_TYPE=Release
if %ERRORLEVEL% neq 0 (
    echo CMake configuration failed!
    exit /b %ERRORLEVEL%
)

cmake --build . --config Release
if %ERRORLEVEL% neq 0 (
    echo Build failed!
    exit /b %ERRORLEVEL%
)

echo.
echo Build successful! The executable is in build\Release\veriview.exe
echo. 