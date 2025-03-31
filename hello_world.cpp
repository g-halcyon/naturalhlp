#include <iostream>
#include <stdexcept> // For exception handling

int main() {
    try {
        // Print "Hello, World!"
        std::cout << "Hello, World!" << std::endl;

        // Calculate the sum of 5 and 10
        int num1 = 5;
        int num2 = 10;
        int sum = num1 + num2;

        // Print the result with a message
        std::cout << "The sum of " << num1 << " and " << num2 << " is: " << sum << std::endl;

        // Print success message
        std::cout << "Program completed successfully." << std::endl;

        return 0; // Indicate successful execution
    } catch (const std::exception& e) {
        // Handle any exceptions that might occur
        std::cerr << "An error occurred: " << e.what() << std::endl;
        return 1; // Indicate an error occurred
    } catch (...) {
        // Catch-all for any other unexpected exceptions
        std::cerr << "An unknown error occurred." << std::endl;
        return 1; // Indicate an error occurred
    }
}