#!/bin/bash

# ANSI color codes
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
MAGENTA="\033[0;35m"
CYAN="\033[0;36m"
RESET="\033[0m"

# Function to print colored text
print_color() {
    echo -e "${1}${2}${RESET}"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Print header
print_color "$BLUE" "========================================"
print_color "$BLUE" "  Natural High Level Programming Language"
print_color "$BLUE" "  Build Script"
print_color "$BLUE" "========================================"
echo

# Check for Rust/Cargo
if ! command_exists cargo; then
    print_color "$RED" "Error: Rust and Cargo are required but not installed."
    print_color "$YELLOW" "Please install Rust by following the instructions at: https://www.rust-lang.org/tools/install"
    exit 1
fi

# Check for environment variables
if [ ! -f ".env" ]; then
    print_color "$YELLOW" "Warning: .env file not found. Creating a template..."
    echo "GEMINI_API_KEY=your_api_key_here" > .env
    print_color "$YELLOW" "Please edit the .env file and add your Gemini API key."
fi

# Build the project
print_color "$CYAN" "Building the NHLP compiler..."
cargo build --release

if [ $? -ne 0 ]; then
    print_color "$RED" "Build failed! Please check the errors above."
    exit 1
fi

print_color "$GREEN" "Build successful!"

# Create symbolic link
print_color "$CYAN" "Creating symbolic link for easy access..."
if [ -f "target/release/nhlp" ]; then
    if [ -f "./nhlp" ]; then
        rm ./nhlp
    fi
    ln -s "$(pwd)/target/release/nhlp" ./nhlp
    print_color "$GREEN" "Symbolic link created. You can now use './nhlp' to run the compiler."
else
    print_color "$RED" "Error: Compiled binary not found at expected location."
    exit 1
fi

# Print usage instructions
print_color "$MAGENTA" "Usage Instructions:"
print_color "$YELLOW" "  ./nhlp input.dshp        # Compile a .dshp file to C++"
print_color "$YELLOW" "  ./nhlp input.dshp -l rust # Compile to Rust instead"
print_color "$YELLOW" "  ./nhlp input.dshp -r     # Compile and run the code"
print_color "$YELLOW" "  ./nhlp -h                # Show help"

# Check for dependencies based on supported target languages
echo
print_color "$CYAN" "Checking for target language compilers/tools..."

# Check for C++ compiler (g++ or clang++)
if command_exists g++; then
    print_color "$GREEN" "✓ g++ found (C++ target support)"
elif command_exists clang++; then
    print_color "$GREEN" "✓ clang++ found (C++ target support)"
else
    print_color "$YELLOW" "⚠ No C++ compiler found. Install g++ or clang++ for C++ target support."
fi

# Check for Rust compiler
if command_exists rustc; then
    print_color "$GREEN" "✓ rustc found (Rust target support)"
else
    print_color "$YELLOW" "⚠ rustc not found. Install Rust for Rust target support."
fi

# Check for NASM (assembly support)
if command_exists nasm; then
    print_color "$GREEN" "✓ nasm found (Assembly target support)"
else
    print_color "$YELLOW" "⚠ nasm not found. Install NASM for Assembly target support."
fi

# Check for ld (linker for assembly)
if command_exists ld; then
    print_color "$GREEN" "✓ ld found (Assembly target support)"
else
    print_color "$YELLOW" "⚠ ld not found. Install binutils for Assembly target support."
fi

echo
print_color "$GREEN" "Setup complete! You can now use the NHLP compiler."
print_color "$BLUE" "For more information, see the README.md file." 