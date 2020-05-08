trait NoisyAnimal {
    fn make_noise<'staic>(&self) -> &'staic str;
}
struct Cat{}
impl NoisyAnimal for Cat {
    fn make_noise<'staic>(&self) -> &'staic str {
        "meow"
    }
}

struct Dog{}
impl NoisyAnimal for Dog {
    fn make_noise<'staic>(&self) -> &'staic str {
        "woof"
    }
}

fn make_noises(animals: Vec<Box<dyn NoisyAnimal>>) {
                        //       ----- trait object
    for animal in animals.iter() {
        println!("{}", animal.make_noise());
    }
}

pub fn call_make_noises() {
    let animals: Vec<Box<dyn NoisyAnimal>> = vec![
        Box::new(Dog{}),
        Box::new(Cat{}),
    ];
    make_noises(animals);
}
