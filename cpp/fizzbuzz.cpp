#include <iostream>
#include <string>

int main() {
  for (int number = 0; number <= 100; number++) {
    std::string output;

    if (number % 3 == 0) {
      output += "Fizz";
    }
    if (number % 5 == 0) {
      output += "Buzz";
    }
    if (output.length() == 0) {
      output += std::to_string(number);
    }

    std::cout << output << '\n';
  }

  return 0;
}
