use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");

    loop {
        println!("Please, input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        if guess.trim().cmp(&"quit".to_string()) == Ordering::Equal {
            println!("A see you later");
            break;
        }

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
