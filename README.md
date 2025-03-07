# Veriview

Veriview is a hardware design visualization tool built with Tauri (Rust) and React. It allows engineers to visualize and interact with hardware netlists, making it easier to understand complex hardware designs.

## Features

- Interactive visualization of hardware netlists
- Module hierarchy exploration
- Signal tracing and analysis
- Cross-platform desktop application (Windows, macOS, Linux)

## Architecture

- **Frontend**: React with Cytoscape.js for graph visualization
- **Backend**: Tauri (Rust) for the desktop application framework
- **C++ Server**: Optional backend server for advanced netlist processing

## Prerequisites

- **Node.js** (v16 or later)
- **Rust** (1.60 or later)
- **Tauri CLI**

## Setup Instructions

### 1. Install Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# On Windows, download and run rustup-init.exe from https://rustup.rs/

# Verify installation
rustc --version
cargo --version
```

### 2. Install Node.js and npm

```bash
# Using a package manager or download from https://nodejs.org/
# Verify installation
node --version
npm --version
```

### 3. Install Tauri CLI

```bash
npm install -g @tauri-apps/cli
# Verify installation
tauri --version
```

### 4. Install Project Dependencies

```bash
# Clone the repository (if you haven't already)
git clone https://github.com/daniellovell/veriview.git
cd veriview

# Install npm dependencies
npm install
```

## Development

To run the application in development mode:

```bash
# Start the development server with hot-reload
npm run tauri dev
```

This will:
1. Start the Vite development server for the React frontend
2. Compile and run the Rust backend
3. Launch the application window

## Building for Production

To build the application for production:

```bash
# Build the optimized application
npm run tauri build
```

This creates platform-specific installers in the `src-tauri/target/release/bundle` directory.

## Other Commands

- `npm run dev` - Start the Vite development server without Tauri
- `npm run build` - Build the React frontend without Tauri
- `npm run preview` - Preview the built frontend

## Project Structure

- `src/` - React frontend code
- `src-tauri/` - Rust backend code
- `public/` - Static assets
- `.github/workflows/` - CI/CD configuration

## License

[BSD License](LICENSE)
