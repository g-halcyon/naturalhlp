# Natural High Level Programming Language Build Script

# ANSI color codes for PowerShell
$colors = @{
    red = [System.Console]::ForegroundColor = "Red"
    green = [System.Console]::ForegroundColor = "Green"
    yellow = [System.Console]::ForegroundColor = "Yellow"
    blue = [System.Console]::ForegroundColor = "Blue"
    magenta = [System.Console]::ForegroundColor = "Magenta"
    cyan = [System.Console]::ForegroundColor = "Cyan"
    reset = [System.Console]::ResetColor()
}

# Function to print colored text
function Print-Color {
    param(
        [string]$color,
        [string]$text
    )
    
    $origColor = [System.Console]::ForegroundColor
    [System.Console]::ForegroundColor = $color
    Write-Host $text
    [System.Console]::ForegroundColor = $origColor
}

# Function to check if a command exists
function Test-Command {
    param(
        [string]$Command
    )
    
    if (Get-Command $Command -ErrorAction SilentlyContinue) {
        return $true
    }
    return $false
}

# Print header
Print-Color "Blue" "========================================"
Print-Color "Blue" "  Natural High Level Programming Language"
Print-Color "Blue" "  Build Script"
Print-Color "Blue" "========================================"
Write-Host ""

# Check for Rust/Cargo
if (-not (Test-Command "cargo")) {
    Print-Color "Red" "Error: Rust and Cargo are required but not installed."
    Print-Color "Yellow" "Please install Rust by following the instructions at: https://www.rust-lang.org/tools/install"
    exit 1
}

# Check for environment variables
if (-not (Test-Path ".env")) {
    Print-Color "Yellow" "Warning: .env file not found. Creating a template..."
    "GEMINI_API_KEY=your_api_key_here" | Out-File -FilePath ".env" -Encoding utf8
    Print-Color "Yellow" "Please edit the .env file and add your Gemini API key."
}

# Build the project
Print-Color "Cyan" "Building the NHLP compiler..."
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Print-Color "Red" "Build failed! Please check the errors above."
    exit 1
}

Print-Color "Green" "Build successful!"

# Create symbolic link or copy binary for easy access
Print-Color "Cyan" "Creating executable for easy access..."
if (Test-Path "target\release\nhlp.exe") {
    if (Test-Path ".\nhlp.exe") {
        Remove-Item ".\nhlp.exe"
    }
    Copy-Item "target\release\nhlp.exe" ".\nhlp.exe"
    Print-Color "Green" "Executable copied. You can now use '.\nhlp.exe' to run the compiler."
}
else {
    Print-Color "Red" "Error: Compiled binary not found at expected location."
    exit 1
}

# Print usage instructions
Print-Color "Magenta" "Usage Instructions:"
Print-Color "Yellow" "  .\nhlp.exe input.dshp        # Compile a .dshp file to C++"
Print-Color "Yellow" "  .\nhlp.exe input.dshp -l rust # Compile to Rust instead"
Print-Color "Yellow" "  .\nhlp.exe input.dshp -r     # Compile and run the code"
Print-Color "Yellow" "  .\nhlp.exe -h                # Show help"

# Check for dependencies based on supported target languages
Write-Host ""
Print-Color "Cyan" "Checking for target language compilers/tools..."

# Check for C++ compiler (g++ or clang++)
if (Test-Command "g++") {
    Print-Color "Green" "✓ g++ found (C++ target support)"
}
elseif (Test-Command "clang++") {
    Print-Color "Green" "✓ clang++ found (C++ target support)"
}
else {
    Print-Color "Yellow" "⚠ No C++ compiler found. Install g++ or clang++ for C++ target support."
}

# Check for Rust compiler
if (Test-Command "rustc") {
    Print-Color "Green" "✓ rustc found (Rust target support)"
}
else {
    Print-Color "Yellow" "⚠ rustc not found. Install Rust for Rust target support."
}

# Check for NASM (assembly support)
if (Test-Command "nasm") {
    Print-Color "Green" "✓ nasm found (Assembly target support)"
}
else {
    Print-Color "Yellow" "⚠ nasm not found. Install NASM for Assembly target support."
}

# Check for ld (linker for assembly)
if (Test-Command "ld") {
    Print-Color "Green" "✓ ld found (Assembly target support)"
}
else {
    Print-Color "Yellow" "⚠ ld not found. Install binutils for Assembly target support."
}

Write-Host ""
Print-Color "Green" "Setup complete! You can now use the NHLP compiler."
Print-Color "Blue" "For more information, see the README.md file." 