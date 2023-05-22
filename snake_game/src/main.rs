
struct Person {
    name: String,
    last_name:String,
    age: u32
}

impl Person {
    fn some_function() {
        println!("some_function")
    }

    fn change_age(&self) {
        println!("Current age: {}", self.age);
    }


}

fn main() {
    let person = Person {
        name: "gagan".to_string(),
        last_name: "nayyar".to_string(),
        age: 30,
    };

    person.change_age();

    println!("{}", person.name);

}