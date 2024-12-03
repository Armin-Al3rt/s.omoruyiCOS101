use std::cmp::Ordering;

#[derive(Debug)]
struct Developer {
    name: String,
    experience: u32, // years of experience
}

fn find_most_experienced(developers: Vec<Developer>) -> Option<Developer> {
    developers.into_iter().max_by(|a, b| a.experience.cmp(&b.experience))
}

fn main() {
    let developers = vec![
        Developer {
            name: String::from("Samantha"),
            experience: 5,
        },
        Developer {
            name: String::from("Chidera"),
            experience: 8,
        },
        Developer {
            name: String::from("Chimamanda"),
            experience: 3,
        },
    ];

    match find_most_experienced(developers) {
        Some(dev) => println!(
            "The most experienced developer is {} with {} years of experience.",
            dev.name, dev.experience
        ),
        None => println!("No developers in the list."),
    }
}
