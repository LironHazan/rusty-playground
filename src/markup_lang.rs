
#[derive(Copy, Clone)]
enum State {
    Normal,
    Comment,
    Upper,
    Lower
}
// state machine using patterns matching for transitions
fn machine_cycle(state: State, c:char) -> (Option<char>, State) {
    use self::State::*;
    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
        (Comment, '#') => (None, Normal),
        (Comment, other) => (Some(other), Comment),
        (Upper, '^') => (None, Comment),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_ascii_lowercase()), Lower),
    }
}

pub fn start() {
    let mut state = State::Normal;
    let mut processed_str = String::new();
    let input = "hey hey ^you #foo";

    for c in input.chars() {
        let (output, new_state) = machine_cycle(state, c);
        if let Some(c) = output {
            processed_str.push(c);
        }
        state = new_state;
    }
    println!("{}", processed_str);
}
