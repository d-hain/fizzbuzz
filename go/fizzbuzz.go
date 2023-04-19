package main

import (
	"fmt"
	"strconv"
)

func main() {
	for number := 1; number <= 100; number++ {
		output := ""
		if number%3 == 0 {
			output += "Fizz"
		}
		if number%5 == 0 {
			output += "Buzz"
		}
		if len(output) == 0 {
			output = strconv.Itoa(number)
		}

		fmt.Println(output)
	}
}
