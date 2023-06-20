struct Person {}
struct Ghost {}

struct NotImplementingAnything {}

trait SomethingNotUseful {
    fn write(&self) -> String {
        println!("not useful");
        "NOT USEFUL".to_string()
    }
}

impl SomethingNotUseful for NotImplementingAnything {}

fn just_some_function<T>(value: &T) -> String
where
    T: SomethingNotUseful,
{
    value.write()
}

trait Movement {
    fn walk(&self) {
        println!("Walking")
    }
}

trait Action {
    fn eat(&self) {
        println!("Eating")
    }
}

trait Human: Action + Movement {
    fn speak(&self) {
        println!("Speak!")
    }
}

impl Human for Person {}
impl Movement for Person {}
impl Action for Person {}
impl Human for Ghost {
    fn speak(&self) {
        println!("Ghost is speaking")
    }
}
impl Movement for Ghost {
    fn walk(&self) {
        println!("Ghost is walking")
    }
}
impl Action for Ghost {}

fn print_action(human: &impl Human) {
    human.speak();
    human.eat();
    human.walk();
}

fn main() {
    let x = Person {};
    print_action(&x);
    let y = Ghost {};
    print_action(&y);
    let z = NotImplementingAnything {};
    let resp = just_some_function(&z);
    println!("{}", resp);
}
