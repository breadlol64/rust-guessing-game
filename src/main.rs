use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);

    // println!("secret number: {}", secret_num);

    loop {
        println!("guess a number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read a line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("you win");
                break;
            },
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("to small"),
        }
    }
}
