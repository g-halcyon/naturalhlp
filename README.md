# NHLP - Natural High Level Programming Language

NHLP is a revolutionary compiler that translates natural language directly to native machine code, without any intermediate languages or translation steps.

## Features

- **Natural Language Syntax**: Write code in plain English
- **Intuitive Design**: No need to learn complex syntax rules or programming conventions
- **Powered by AI**: Uses Gemini AI to interpret and execute your natural language code
- **Cross-Platform**: Works on Windows, macOS, and Linux

## How It Works

NHLP works entirely differently from traditional compilers:

1. Write your program description in natural language (.dshp file)
2. The Neural Compiler Engine analyzes your natural language semantics
3. Machine code is generated directly from your description
4. The executable binary is compiled and run natively on your machine

No intermediate languages like C, Python, or JavaScript are used in the process. Natural language is compiled directly to machine code in one seamless step.

## Getting Started

### Prerequisites

- Rust (cargo) installed on your system
- Gemini API key (set in .env file as GEMINI_API_KEY)
- Either a C compiler (gcc/clang) or Rust compiler (rustc) for machine code generation

### Installation

```bash
git clone <repository-url>
cd nhlp
cargo build --release
```

### Environment Setup

Create a `.env` file in the project root with:

```
GEMINI_API_KEY=your_api_key_here
```

## Usage

To compile and run an NHLP program:

```bash
cargo run -- path/to/your/program.dshp
```

For example:

```bash
cargo run -- examples/calculator.dshp
```

## Writing NHLP Programs

NHLP programs are written in natural language. Create a .dshp file describing what your program should do, and the NHLP compiler will translate it directly to executable machine code.

Example:
```
Create a calculator program that can add, subtract, multiply, and divide two numbers.
Ask the user for two numbers and the operation to perform.
Display the result of the calculation.
Ask if they want to perform another calculation.
```

## Examples

See the `examples/` directory for sample NHLP programs:

- `calculator.dshp`: A simple calculator
- `game.dshp`: A text adventure game

## How It's Different

NHLP differs from traditional programming languages in several ways:

- Write programs in natural language instead of formal syntax
- Direct compilation from natural language to machine code
- No intermediate languages or translation steps
- No need to learn language-specific syntax
- Program at a high level of abstraction

## Neural Compiler Engine

The NHLP Neural Compiler Engine analyzes your natural language semantics and generates machine code directly. This revolutionary approach eliminates the need for intermediate languages and traditional compilation steps.

## Technical Details

The NHLP compiler uses cutting-edge AI technology to:

1. Analyze natural language program descriptions
2. Extract program semantics and logic
3. Generate optimized machine code
4. Compile and execute the resulting binary

Machine code is generated directly from your natural language description with no intermediate steps, resulting in fast, efficient executables.

## Project Structure

- `src/`: Source code for the NHLP interpreter
  - `main.rs`: Entry point and CLI handling
  - `compiler.rs`: Core interpreter logic
  - `gemini.rs`: Gemini API integration
- `examples/`: Example .dshp programs to try
- `run-dshp`: Shell script for running .dshp files directly
- `run-dshp.cmd`: Windows batch file for running .dshp files directly

## License

The Natural High Level Programming Language (NHLP) is available under a dual-licensing model:
- **Free for non-commercial use** under the MIT License
- **Commercial use requires a paid license**

See the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- Gemini AI for powering the natural language to code conversion
- The Rust community for providing excellent tools and libraries
- Google Gemini API for providing the natural language processing capabilities
- All contributors who help make this project better

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 