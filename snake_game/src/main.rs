
struct Person {
    name: String,
    last_name: String,
    age: u32,
}

impl Person {
    fn from(name: String, last_name:String, age:u32) -> Person {
        Person {
            name, 
            last_name,
            age,
        }
    }

    fn change_age(&mut self, new_age: u32) {
        self.age = new_age
    }

}

fn main() {
    


    let mut person = Person {
        name: "Gagan".to_string(),
        last_name: "Nayyar".to_string(),
        age: 33,
    };

    let mut person_1 = Person::from (
        String::from("Shahzia"),
        String::from("Kulsum"),
        10
    );

    person_1.change_age(30);

    println!("{} {} {}", person_1.name, person_1.last_name, person_1.age);
}