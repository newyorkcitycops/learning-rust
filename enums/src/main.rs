#[derive(Debug)]
enum TaskState {
    Todo,
    InProgress,
    Done(String),
    Blocked(String),
}

#[derive(Debug)]
struct Task {
    name: String,
    description: String,
    state: TaskState,
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    borrower: Option<String>,
}

enum Number {
    Integer(i32),
    Float(f64),
}

impl Task {
    fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            state: TaskState::Todo,
        }
    }

    fn update_state(&mut self, state: TaskState) {
        self.state = state;
    }
}

impl Book {
    fn new(title: String, author: String) -> Self {
        Self {
            title,
            author,
            borrower: None,
        }
    }

    fn borrow(&mut self, borrower: String) -> Result<(), String> {
        match self.borrower {
            Some(_) => Err(String::from("The book is already borrowed.")),
            None => {
                self.borrower = Some(borrower);
                Ok(())
            }
        }
    }

    fn return_book(&mut self) -> Result<(), String> {
        match self.borrower {
            Some(_) => {
                self.borrower = None;
                Ok(())
            }
            None => Err(String::from("The book is not borrowed.")),
        }
    }
}

fn main() {
    // Struct with enum
    let mut task = Task::new(String::from("task"), String::from("a task"));

    task.update_state(TaskState::Done(String::from("done")));

    println!("{:?}", task);

    // Option enum and match statement
    let mut book = Book::new(String::from("book"), String::from("a book"));

    match book.borrow(String::from("John")) {
        Ok(()) => println!("John borrowed book"),
        Err(err) => println!("{}", err),
    }

    match book.borrow(String::from("Daniel")) {
        Ok(()) => println!("Daniel borrowed book"),
        Err(err) => println!("{}", err),
    }

    match book.return_book() {
        Ok(()) => println!("Book returned"),
        Err(err) => println!("{}", err),
    }

    match book.borrow(String::from("Daniel")) {
        Ok(()) => println!("Daniel borrowed book"),
        Err(err) => println!("{}", err),
    }

    println!("{:?}", book);

    // if let
    let integer: Option<Number> = Some(Number::Integer(42));
    
    let float: Option<Number> = Some(Number::Float(3.1415));

    if let Some(Number::Integer(value)) = integer {
        println!("Number is an integer");
    } else {
        println!("Number isn't an integer");
    }

    if let Some(Number::Float(value)) = float {
        println!("Number is a float-pointing number");
    } else {
        println!("Number isn't a float-pointing number");
    }
}
