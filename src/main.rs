trait Song {
    fn play(&self) -> String;
}

pub trait Solo {
    fn go(&self) -> String;
}

struct Lineup {
    pub songs: Vec<Box<Song>>,
}

impl Lineup {
    pub fn run(&self) {
    for song in self.songs.iter() {
        song.play();
    } }
}

struct Guitarist {
    pub guitar: String,
    pub notes: String,
}

struct Drummer {
    pub sticks: String,
    pub drum_set: String,
    // pub reathem: String,
}

impl Song for Guitarist {
    fn play(&self) -> String {
        format!("[Guitarist Info: My guitar brand is: {}]", self.guitar)
    }
}

impl Song for Drummer {
    fn play(&self) -> String {
        format!("[Drummer Info: I use {} sticks, My set brand is: {}] ", self.sticks, self.drum_set)
    }
}

impl Solo for Guitarist {
    fn go(&self) -> String {
        format!("[ Distortion is on that melody! ]")
    }
}

impl Solo for Drummer {
    fn go(&self) -> String {
        format!("[ Beat Groove Beat ]")
    }
}

pub fn play_solo<T: Solo>(_instrument: T) -> String {
    String::from("Rocking Solo!")
}

fn main() {
    let drummer = Drummer { sticks:  String::from("wood"), drum_set: String::from("Yamaha")};
    let guitarist = Guitarist { guitar: String::from("Fender USA Tele"), notes:  String::from("notes!")};

    let drummer_in = drummer.play();
    let guitarist_in = guitarist.play();
    let guitar_solo = play_solo(guitarist);

    println!("{} {} {}",
             drummer_in,
             guitarist_in,
             guitar_solo
    );

}
