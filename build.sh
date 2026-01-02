#!/bin/bash

# Source Rust environment
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Build Java backend
echo "Building Java backend..."
cd java-backend
mvn clean package -DskipTests
cd ..

# Build Rust models
echo "Building Rust models..."
cd rust-model
if command -v cargo &> /dev/null; then
    cargo build --release
else
    echo "Warning: cargo not found, skipping Rust build"
fi
cd ..

echo "Build complete!"
