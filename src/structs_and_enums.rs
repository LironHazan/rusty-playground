// Basic type reminds the type / interface declaration in Typescript
struct Foo {
    bazz: String,
}

pub fn use_foo() {
    let _foo = Foo {
        bazz: String::from("Hey you!"),
    };
    println!("{}", _foo.bazz);
}

#[derive(PartialEq)]
enum Animal {
    Dog,
    Cat,
    Cow,
    Other(u32)
}

enum Baz {
    VarA(Foo), // interesting
}

fn make_sound() {
    let biss = Animal::Dog;
    let chuk = Animal::Cat;

    // This will fail: assert!(biss == chuk);

}
