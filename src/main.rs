use std::collections::LinkedList;
mod basics; // filename
use basics::fib::run_fib_memo;
use basics::guessgame::get_guess;
use basics::guessgame::handle_guess;
use rand::random;
mod borrows;
use borrows::borrows::imperative_sum;
mod structs_and_enums;
use structs_and_enums::use_foo;
mod markup_lang;
use markup_lang::start;

fn main() {

    let mut ll = LinkedList::new();
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(3);

    for foo in ll { // LinkedList implements iterator
        // println!("{}", foo)
    }

    run_fib_memo();

    imperative_sum();

    use_foo();

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
