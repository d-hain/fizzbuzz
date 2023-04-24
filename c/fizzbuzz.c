#include <stdio.h>
#include <string.h>

int main() {
  for (int number = 0; number <= 100; number++) {
    char output[9] = ""; // Size 9 because of the string terminator \0

    if (number % 3 == 0) {
      strcat(output, "Fizz");
    }
    if (number % 5 == 0) {
      strcat(output, "Buzz");
    }
    if (strlen(output) == 0) {
      sprintf(output, "%d", number);
    }

    printf("%s\n", output);
  }

  return 0;
}
