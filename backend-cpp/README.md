# Veriview Backend

A simple C++ backend for the Veriview project.

## Dependencies

- CMake 3.15+
- C++17 compatible compiler
- vcpkg (for dependency management)
- Node.js and npm (for the React frontend)

## Building with vcpkg

1. **Install vcpkg** if you haven't already:

   ```bash
   git clone https://github.com/microsoft/vcpkg.git
   cd vcpkg
   ./bootstrap-vcpkg.bat  # Windows
   # OR
   ./bootstrap-vcpkg.sh   # Linux/macOS
   ```

2. **Set VCPKG_ROOT**:

   Add the following to your `~/.bashrc` (or equivalent) and system environment variables:

   ```bash
   export VCPKG_ROOT=[path/to/vcpkg]
   export PATH=$VCPKG_ROOT:$PATH
   ```

   Replace `[path/to/vcpkg]` with the actual path to your vcpkg installation.

3. **Install dependencies**:

   ```bash
   ./vcpkg install crow nlohmann-json
   ```

4. **Build the React frontend**:

   ```bash
   cd frontend-react
   npm install
   npm run build
   ```

   This will create a `build` directory in the frontend-react folder with the compiled frontend.

5. **Build the C++ backend**:

   ```bash
   cd backend-cpp
   mkdir build
   cd build
   cmake .. -DCMAKE_TOOLCHAIN_FILE=[path/to/vcpkg]/scripts/buildsystems/vcpkg.cmake
   cmake --build .
   ```

   Replace `[path/to/vcpkg]` with the actual path to your vcpkg installation.

## Running

After building, the executable will be in the `build` directory (or `build/Debug` on Windows):

```bash
./veriview  # Linux/macOS
# OR
.\veriview.exe  # Windows
```

The server will start on port 8080 by default. You can access the application by opening a browser and navigating to:

```
http://localhost:8080
```

The backend will serve both the API endpoints and the React frontend. 