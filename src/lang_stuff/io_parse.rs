 use std::io;

    // point to think of Result trait is similar to Either typeclass (haskell)
    pub fn ask_name() -> String {
        println!("Enter your name");
        loop {
            let mut name = String::new();
            io::stdin().read_line(&mut name)
                .expect("can't read from stdin");
            match name.trim().parse::<String>() {
                Ok(v) => return v,
                Err(e) => print!("Wrong! {}", e ),
            }
        }

    }

    pub fn greet(name: String)  {
            println!("Nice to meet you {} !!", name);
    }


