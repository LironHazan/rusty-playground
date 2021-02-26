mod lang_stuff;
mod parse;

use std::collections::LinkedList;
use rand::random;

use lang_stuff::borrows::borrows::imperative_sum;
use lang_stuff::io_parse::get_guess;
use lang_stuff::io_parse::handle_guess;
use parse::markup_lang::start;



fn main() {

    imperative_sum();

    start();

    let correct = random::<u8>();
    println!("{}", correct);

    loop {
        let guess = get_guess();
        if handle_guess(guess, correct) {
            break;
        }
    }
}
