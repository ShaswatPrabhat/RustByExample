#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("Hello, world!");
    println!("Now I am here {:?}", Structure(3));
    println!("And here {:?}", Deep(Structure(4)));
    println!(
        "Pretty printing {:#?}",
        Person {
            name: "Zuzu",
            age: 32
        }
    );
}
