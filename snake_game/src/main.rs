

fn main() {
 
    let message = String::from("Hello");
    print_message(message);

}

fn print_message(a: String) {
    println!("{}",a);
    let _c = &a;
    println!("{}", _c)
}