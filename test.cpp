#include <iostream>
#include <string>
#include <stdexcept> // For exception handling

// Function to calculate the sum of two numbers
double calculate_sum(double num1, double num2) {
  // Consider using static_assert for compile-time checks if the input types are known at compile time.
  // Example: static_assert(std::is_arithmetic<decltype(num1)>::value, "num1 must be arithmetic");

  // Optional: Add input validation (e.g., check for NaN or infinity if needed)
  if (std::isnan(num1) || std::isnan(num2)) {
    throw std::invalid_argument("Input numbers cannot be NaN.");
  }
  if (std::isinf(num1) || std::isinf(num2)) {
    throw std::overflow_error("Input numbers cannot be infinite.");
  }

  return num1 + num2;
}

// Function to create a greeting message
std::string greet(const std::string& name) {
  if (name.empty()) {
    throw std::invalid_argument("Name cannot be empty.");
  }
  return "Hello, " + name + "!";
}

int main() {
  try {
    // Calculate the sum of 5 and 3
    double sum = calculate_sum(5, 3);

    // Create a greeting message for "DSHPC"
    std::string message = greet("DSHPC");

    // Print the result and the message
    std::cout << "The sum is: " << sum << std::endl;
    std::cout << message << std::endl;

    return 0; // Indicate successful execution
  } catch (const std::exception& e) {
    std::cerr << "Error: " << e.what() << std::endl; // Print the error message to the standard error stream.
    return 1; // Indicate an error occurred
  }
}