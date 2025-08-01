# Render Build Command Guide for Zklear

## Overview

This guide explains the build process for deploying Zklear to Render, including the RISC Zero toolchain requirements and different build command options.

## Build Requirements

Zklear requires several components to be built:

1. **RISC Zero Toolchain** - For ZK proof generation
2. **Rust Backend** - The main application server
3. **React Frontend** - The web interface
4. **Runtime Files** - Initial data files

## Build Command Options

### Option 1: Simple Build Command (Recommended)

Use this in Render's build settings:

```bash
cargo install rzup && rzup install rust && rustup target add riscv32imac-unknown-none-elf && cargo build --release && cd zklear-frontend && npm ci && npm run build && cd .. && mkdir -p data && echo '[]' > accounts.json && echo '[]' > transactions.json
```

### Option 2: Using the Build Script

Set the build command to:
```bash
./build-render.sh
```

### Option 3: Using Docker (Alternative)

If you prefer Docker-based builds, use:
```bash
docker build -f Dockerfile.render -t zklear .
```

## Detailed Build Process

### Step 1: RISC Zero Toolchain Installation

```bash
# Install rzup (RISC Zero package manager)
cargo install rzup

# Install RISC Zero Rust toolchain
rzup install rust

# Add RISC-V target for guest code compilation
rustup target add riscv32imac-unknown-none-elf
```

### Step 2: Rust Backend Compilation

```bash
# Build the Rust application in release mode
cargo build --release
```

This creates the binary at `target/release/host`

### Step 3: React Frontend Build

```bash
# Navigate to frontend directory
cd zklear-frontend

# Install dependencies
npm ci

# Build the React application
npm run build

# Return to root directory
cd ..
```

### Step 4: Runtime Setup

```bash
# Create data directory
mkdir -p data

# Create initial data files
echo '[]' > accounts.json
echo '[]' > transactions.json
```

## Render Build Configuration

### Environment Variables

Set these in Render's environment variables:

```
RUST_LOG=info
RUST_BACKTRACE=1
```

### Build Settings

- **Build Command**: Use one of the options above
- **Start Command**: `./start.sh`
- **Dockerfile Path**: `./Dockerfile.render` (if using Docker)

## Troubleshooting

### Common Build Issues

1. **RISC Zero Toolchain Not Found**
   ```bash
   # Solution: Ensure rzup is installed first
   cargo install rzup
   rzup install rust
   ```

2. **RISC-V Target Missing**
   ```bash
   # Solution: Add the target
   rustup target add riscv32imac-unknown-none-elf
   ```

3. **Node.js Dependencies**
   ```bash
   # Solution: Use npm ci for clean installs
   npm ci
   ```

4. **Build Timeout**
   - RISC Zero compilation can take 10-15 minutes
   - Consider using Render's Standard or Pro plans for faster builds

### Build Verification

After a successful build, verify these files exist:

```bash
# Check Rust binary
ls -la target/release/host

# Check React build
ls -la zklear-frontend/build/

# Check data files
ls -la accounts.json transactions.json
```

## Performance Optimization

### Build Time Optimization

1. **Use Cargo Caching**
   ```bash
   # Add to build command
   cargo build --release --jobs 4
   ```

2. **Parallel npm install**
   ```bash
   # Use npm ci with parallel flag
   npm ci --prefer-offline --no-audit
   ```

3. **Docker Layer Caching**
   - The Dockerfile.render uses multi-stage builds
   - Dependencies are cached in separate layers

### Memory Optimization

- **Rust Build**: ~2GB RAM recommended
- **Node.js Build**: ~1GB RAM recommended
- **Total**: ~4GB RAM for optimal build performance

## Alternative Build Commands

### Minimal Build Command

```bash
cargo install rzup && rzup install rust && rustup target add riscv32imac-unknown-none-elf && cargo build --release && cd zklear-frontend && npm ci && npm run build && cd ..
```

### Verbose Build Command

```bash
set -e && echo "Installing RISC Zero..." && cargo install rzup && rzup install rust && rustup target add riscv32imac-unknown-none-elf && echo "Building Rust..." && cargo build --release && echo "Building React..." && cd zklear-frontend && npm ci && npm run build && cd .. && echo "Build complete!"
```

### Production Build Command

```bash
cargo install rzup && rzup install rust && rustup target add riscv32imac-unknown-none-elf && RUSTFLAGS="-C target-cpu=native" cargo build --release && cd zklear-frontend && npm ci --production && npm run build && cd .. && mkdir -p data && echo '[]' > accounts.json && echo '[]' > transactions.json
```

## Monitoring Build Progress

### Build Logs

Monitor these key indicators in Render's build logs:

1. **RISC Zero Installation**
   ```
   Installing rzup...
   Installing RISC Zero Rust toolchain...
   ```

2. **Rust Compilation**
   ```
   Compiling risc-zero-payment v0.1.0
   Finished release [optimized] target(s)
   ```

3. **React Build**
   ```
   Compiled successfully!
   ```

4. **Final Verification**
   ```
   ✅ Build completed successfully!
   ```

## Best Practices

1. **Use the build script** for consistent builds
2. **Monitor build logs** for any errors
3. **Test locally** before deploying
4. **Use appropriate Render plan** for build resources
5. **Set up proper environment variables**

## Support

If you encounter build issues:

1. Check the build logs in Render dashboard
2. Verify all dependencies are correctly specified
3. Test the build process locally first
4. Ensure sufficient build resources are allocated

---

**Happy building! 🚀** 