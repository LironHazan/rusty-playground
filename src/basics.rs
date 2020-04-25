
pub mod fib {
    use std::collections::HashMap;

    // 3 slash will make a rustdoc of the following comment!
    /// memo fib is private
    /// ` fib_memo() `
    fn fib_memo(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
        // pattern matching
        match n {
            0 | 1 => 1,
            n => {
                if map.contains_key(&n) {
                    // unwrap related to error handling
                    *map.get(&n).unwrap()
                } else {
                    let val = fib_memo(n-1, map) + fib_memo(n-2, map);
                    map.insert(n, val);
                    val
                }
            }
        }
    }

    pub fn run_fib_memo() {
        let mut map = HashMap::new();
        for i in 1..20 {
            println!("{}:{}", i, fib_memo(i, &mut map))
        }
    }
}

pub mod guessgame {
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
}

