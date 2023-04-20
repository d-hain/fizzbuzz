if __name__ == "__main__":
    for number in range(0, 100):
        output = ""

        if number % 3 == 0:
            output += "Fizz"
        if number % 5 == 0:
            output += "Buzz"
        if len(output) == 0:
            output = number
        print(output)
