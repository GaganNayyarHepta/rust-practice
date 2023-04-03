

fn main() {
    let from = String::from("hello");
    let mut message = from;
    let name = "Filip";

    message.push_str(" World");

    println!("{}", message);
    println!("{}", name);

}