
// 不能使用fmt::Display
struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32) ;

#[derive(Debug)]
struct Stucture(i32);

#[derive(Debug)]
struct Deep(Stucture);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor = "actor's");
    println!("Now {:?} will print!", Stucture(3));
    println!("Now {:?} will print!", Deep(Stucture(5)));

    let name = "Peter";
    let age = 25;
    let peter = Person{name, age};
    println!("{:#?}", peter);
}
