fn main() {
    for number in 1..=100 {
        let mut output = String::new();
        
        if number % 3 == 0 {
            output.push_str("Fizz");
        }
        if number % 5 == 0 {
            output.push_str("Buzz");
        }
        if output.is_empty() {
            output.push_str(&number.to_string());
        }
        
        println!("{}", output);
    }
}
