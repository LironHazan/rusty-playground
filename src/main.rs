mod lang_stuff;
use lang_stuff::borrows::borrows::imperative_sum;
use lang_stuff::io_parse::ask_name;
use lang_stuff::io_parse::greet;

fn main() {
    greet(ask_name())
}
