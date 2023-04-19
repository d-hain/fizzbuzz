use anyhow::Result;

fn main() -> Result<()> {
    println!("FizzBuzz written in Rust.\n");

    loop {
        println!("Which version do you want to run?");
        println!("\t(1) simple");
        println!("\t(2) ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => {
                simple();
                return Ok(());
            },
            "2" => {
                println!("Not implemented yet!");
                return Ok(());
            },
            _ => println!("Invalid input!\n")
        }
    }
}

fn simple() {
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
