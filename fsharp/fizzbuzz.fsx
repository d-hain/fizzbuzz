for number in 1..100 do
  let output =
    if number % 3 = 0 && number % 5 = 0 then "FizzBuzz"
    elif number % 3 = 0 then "Fizz"
    elif number % 5 = 0 then "Buzz"
    else string number

  printfn "%s" output
