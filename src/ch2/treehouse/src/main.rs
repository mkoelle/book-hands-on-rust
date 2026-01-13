use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            // Could also use Visitor here, but would have to change if the struct name changes
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome back, {}!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome back, {}!", self.name);
                println!("Note for {}: {}", self.name, note);
            }
            VisitorAction::Refuse => println!("Do not let {} in!", self.name),
            VisitorAction::Probation => println!(
                "{} is on probation. Let them in, but keep an eye on them.",
                self.name
            ),
        }
        if self.age < 21 {
            println!("Note: {} is not old enough to drink alcohol.", self.name);
        }
    }
}

fn get_user_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Alice", VisitorAction::Accept, 30),
        Visitor::new(
            "Bob",
            VisitorAction::AcceptWithNote {
                note: "Diet soda is in the fridge.".to_string(),
            },
            25,
        ),
        Visitor::new("Fred", VisitorAction::Refuse, 28),
    ];

    loop {
        println!("Hello, What is your name? (leave blank to exit)");

        let name = get_user_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet(),
            None => {
                if name.is_empty() {
                    break;
                }
                println!("{} is not in the list, welcome new friend!", name);
                visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
