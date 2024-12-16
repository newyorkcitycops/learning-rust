#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    sex: char,
}

// Field init shorthand
fn create_male(name: &str, age: i32) -> Person {
    Person {
        name: name.to_string(),
        age,
        sex: 'M'
    }
}

// Field update operator
fn update_person(person: Person) -> Person {
    Person {
        ..person
    }
}

// Tuple structs
#[derive(Debug)]
struct Point(i32, i32);

// Unit struct
struct Mutable;

// Derive keyword
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct User {
    name: String,
    username: String,
    password: String,
}

impl User {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn new(name: &str, username: &str, password: &str) -> Self {
        Self {
            name: name.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

fn main() {
    // Struct definition
    let john = Person {
        name: "John".to_string(),
        age: 31,
        sex: 'M'
    };

    println!("{:?}", john);

    let mut lucas = create_male("Lucas", 25);

    println!("Created lucas: {:?}", lucas);

    lucas.age = 26;

    let updated_lucas = update_person(lucas);

    println!("Updated lucas: {:?}", updated_lucas);

    let coord = Point(10, -10);

    println!("Your coordinates: {:?}", coord);

    let mutable = Mutable;

    println!("{:?}", Rectangle { width: 32, height: 64 });

    let user = User::new("test", "test", "123");

    println!("User name: {:?}", user.get_name());
}
