use std::io;

fn main() {
    println!("Lets guess a number");
    println!("Hello Ann, what number of guesses do you want?");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failure 101");
    println!("HERE IS NUMBER HAHA: {}", guess);
}
