public class FizzBuzz {
  public static void main(String[] args) {
    for (int number = 0; number <= 100; number++) {
      String output = "";

      if (number % 3 == 0) {
        output += "Fizz";
      }
      if (number % 5 == 0) {
        output += "Buzz";
      }
      if (output.length() == 0) {
        output = String.valueOf(number);
      }

      System.out.println(output);
    }
  }
}
