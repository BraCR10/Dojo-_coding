use std::io;
use rand::Rng;

fn main() {
    let theNumber = rand::thread_rng().gen_range(1..=100);

    println!("Hi!, Could you guess the number?");
    println!("If you dare, input the number");
    let mut numberGuessed= String::new();

    //It is just a line 
    io::stdin()
    .read_line(&mut numberGuessed)
    .expect("Unvalid date!");

    println!("You guessed {numberGuessed}");
}
