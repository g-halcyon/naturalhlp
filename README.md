# Natural High Level Programming Language (NHLP)

A high-level programming language that allows you to write code in natural language, making programming more accessible and intuitive.

## Features

- Write code in natural language
- Automatic translation to C++
- Built-in support for common programming patterns
- Fast compilation with Gemini 2.0 Flash model or any other llm model
- Error handling and validation

## Installation

1. Clone the repository:
```bash
git clone https://github.com/g-halcyon/naturalhlp.git
cd naturalhlp
```

2. Create a `.env` file in the project root and add your Gemini API key:
```
GEMINI_API_KEY=your_api_key_here
```

3. Build the project:
```bash
cargo build --release
```

## Usage

1. Write your code in a `.dshp` file using natural language. For example:

```dshp
Create a function that adds two numbers together and returns the result.
Name this function "calculate_sum".
It should take two numbers as input and return a number.

Create a function that creates a greeting message.
Name this function "greet".
It should take a name as input and return a greeting string.

Create a main function that:
1. Calls calculate_sum with numbers 5 and 3 and stores the result
2. Calls greet with the name "NHLP" and stores the message
3. Prints both the result and the message

Run the main function.
```

2. Compile your code:
```bash
cargo run --release -- examples/your_file.dshp
```

3. The compiler will generate a C++ file that you can compile and run:
```bash
g++ your_file.cpp -o your_file
./your_file
```

## Project Structure

```
naturalhlp/
├── src/
│   ├── main.rs           # Main entry point
│   ├── compiler.rs       # Compiler implementation
│   ├── parser.rs         # Natural language parser
│   ├── codegen.rs        # C++ code generator
│   └── gemini.rs         # Gemini API integration
├── examples/             # Example .dshp files
├── tests/               # Test files
├── Cargo.toml          # Rust project configuration
└── README.md           # This file
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

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