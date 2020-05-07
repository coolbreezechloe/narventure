use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    println!(" - NARVENTURE -");
    println!();
    println!("You see a secret in front of you.");
    println!("Your flower number is {}", secret);
    loop {
        println!();
        println!("What would you like to do?");
        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read input");

        let action = action.trim();

        match action.cmp("quit") {
            Ordering::Less => println!("Hmm, maybe I will {} too.", action),
            Ordering::Greater => println!("I am not sure if you should {}.",
                                         action),
            Ordering::Equal => {
                println!("See you later!");
                break;
            }
        }
    }
}
