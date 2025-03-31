#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Display help
show_help() {
    echo -e "${YELLOW}Superior High-Level Programming Language Compiler Build Script${NC}"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --help            Display this help message"
    echo "  --build           Build the compiler"
    echo "  --install         Build and install the compiler"
    echo "  --docker          Build Docker image"
    echo "  --run FILE        Compile a .dshp file"
    echo "  --clean           Clean build artifacts"
    echo ""
    echo "Examples:"
    echo "  $0 --build"
    echo "  $0 --run examples/hello_world.dshp"
}

# Build the project
build() {
    echo -e "${GREEN}Building dshpc...${NC}"
    cargo build --release
    echo -e "${GREEN}Build complete. Binary available at target/release/dshpc${NC}"
}

# Install the compiler
install() {
    echo -e "${GREEN}Building and installing dshpc...${NC}"
    cargo install --path .
    echo -e "${GREEN}Installation complete.${NC}"
}

# Build Docker image
build_docker() {
    echo -e "${GREEN}Building Docker image...${NC}"
    docker build -t dshpc .
    echo -e "${GREEN}Docker image built. Run with: docker run -v \$(pwd):/app dshpc your_file.dshp${NC}"
}

# Run the compiler on a file
run_file() {
    if [ ! -f "$1" ]; then
        echo -e "${RED}Error: File $1 does not exist${NC}"
        exit 1
    fi

    if [ ! -f "target/release/dshpc" ]; then
        echo -e "${YELLOW}Compiler not found. Building first...${NC}"
        build
    fi

    echo -e "${GREEN}Compiling $1...${NC}"
    ./target/release/dshpc "$1"
}

# Clean build artifacts
clean() {
    echo -e "${GREEN}Cleaning build artifacts...${NC}"
    cargo clean
    echo -e "${GREEN}Clean complete.${NC}"
}

# Main logic
if [ $# -eq 0 ]; then
    show_help
    exit 0
fi

case "$1" in
    --help)
        show_help
        ;;
    --build)
        build
        ;;
    --install)
        install
        ;;
    --docker)
        build_docker
        ;;
    --run)
        if [ -z "$2" ]; then
            echo -e "${RED}Error: No file specified for --run${NC}"
            exit 1
        fi
        run_file "$2"
        ;;
    --clean)
        clean
        ;;
    *)
        echo -e "${RED}Unknown option: $1${NC}"
        show_help
        exit 1
        ;;
esac 