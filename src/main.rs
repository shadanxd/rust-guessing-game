use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("hey rusty");
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);


    let mut count: u32 =0;

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        count = count+1;
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Attempts taken {count}");
                break;
            }
        };
    }
}