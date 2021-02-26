
    extern crate rand;
    use rand::random;
    use std::io;

    // point to think of Result trait is similar to Either typeclass (haskell)
    pub fn get_guess() -> u8 {
        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)
                .expect("can't read from stdin");
            match guess.trim().parse::<u8>() {
                Ok(v) => return v,
                Err(e) => print!("Wrong! {}", e ),
            }
        }

    }

    pub fn handle_guess(guess: u8, correct: u8) -> bool {
        if guess < correct {
            println!("less");
            false

        } else if guess > correct {
            println!("more");
            false
        } else {
            println!("Boom!");
            true
        }
    }


