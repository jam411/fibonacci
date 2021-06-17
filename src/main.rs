use std::io;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Please type \"quit\", if you stop program.");

    loop {
        let mut n = String::new();

        println!("Enter a positive number:");

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line.");

        if n.trim() == "quit" {
            break;
        }

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input.");
                continue
            },
        };

        println!("{}", fibonacci(n));
    }
}