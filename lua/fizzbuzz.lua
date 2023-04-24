for number = 1, 100 do
   local output = ""

   if number % 3 == 0 then
      output = output .. "Fizz"
   end
   if number % 5 == 0 then
      output = output .. "Buzz"
   end
   if string.len(output) == 0 then
      output = number
   end

   print(output)
end