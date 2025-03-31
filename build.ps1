#
# Superior High-Level Programming Language Compiler Build Script (PowerShell)
#

# Colors for output
$Green = [ConsoleColor]::Green
$Yellow = [ConsoleColor]::Yellow
$Red = [ConsoleColor]::Red

# Display help
function Show-Help {
    Write-Host "Superior High-Level Programming Language Compiler Build Script" -ForegroundColor $Yellow
    Write-Host ""
    Write-Host "Usage: .\build.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -Help            Display this help message"
    Write-Host "  -Build           Build the compiler"
    Write-Host "  -Install         Build and install the compiler"
    Write-Host "  -Docker          Build Docker image"
    Write-Host "  -Run FILE        Compile a .dshp file"
    Write-Host "  -Clean           Clean build artifacts"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\build.ps1 -Build"
    Write-Host "  .\build.ps1 -Run examples\hello_world.dshp"
}

# Build the project
function Build-Project {
    Write-Host "Building dshpc..." -ForegroundColor $Green
    cargo build --release
    Write-Host "Build complete. Binary available at target\release\dshpc.exe" -ForegroundColor $Green
}

# Install the compiler
function Install-Project {
    Write-Host "Building and installing dshpc..." -ForegroundColor $Green
    cargo install --path .
    Write-Host "Installation complete." -ForegroundColor $Green
}

# Build Docker image
function Build-Docker {
    Write-Host "Building Docker image..." -ForegroundColor $Green
    docker build -t dshpc .
    Write-Host "Docker image built. Run with: docker run -v ${PWD}:/app dshpc your_file.dshp" -ForegroundColor $Green
}

# Run the compiler on a file
function Run-File($file) {
    if (-not (Test-Path $file)) {
        Write-Host "Error: File $file does not exist" -ForegroundColor $Red
        exit 1
    }

    if (-not (Test-Path "target\release\dshpc.exe")) {
        Write-Host "Compiler not found. Building first..." -ForegroundColor $Yellow
        Build-Project
    }

    Write-Host "Compiling $file..." -ForegroundColor $Green
    .\target\release\dshpc.exe $file
}

# Clean build artifacts
function Clean-Project {
    Write-Host "Cleaning build artifacts..." -ForegroundColor $Green
    cargo clean
    Write-Host "Clean complete." -ForegroundColor $Green
}

# Main logic
if ($args.Count -eq 0) {
    Show-Help
    exit 0
}

switch ($args[0]) {
    "-Help" {
        Show-Help
    }
    "-Build" {
        Build-Project
    }
    "-Install" {
        Install-Project
    }
    "-Docker" {
        Build-Docker
    }
    "-Run" {
        if ($args.Count -lt 2) {
            Write-Host "Error: No file specified for -Run" -ForegroundColor $Red
            exit 1
        }
        Run-File $args[1]
    }
    "-Clean" {
        Clean-Project
    }
    default {
        Write-Host "Unknown option: $($args[0])" -ForegroundColor $Red
        Show-Help
        exit 1
    }
} 