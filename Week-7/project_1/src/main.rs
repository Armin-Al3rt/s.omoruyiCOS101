use std::io;


fn public_service() {
    let years_of_service = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let office_administrator = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];
    let academic = vec![
        "-",
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];
    let lawyer = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];
    let teacher = vec![
        "Placement",
        "Classroom Teacher",
        "Snr Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    let roles = vec![
        ("Office Administrator", office_administrator),
        ("Academic", academic),
        ("Lawyer", lawyer),
        ("Teacher", teacher),
    ];

    let mut input = String::new();

    println!("Are you a public servant (y or n)?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    if !input.trim().eq_ignore_ascii_case("y") {
        println!("You are not a public servant.");
        return;
    }

    println!("Select your years of service:");
    for (index, level) in years_of_service.iter().enumerate() {
        println!("{}. {}", index + 1, level);
    }

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let years_index: usize = input.trim().parse::<usize>().unwrap_or(0);
    if years_index == 0 || years_index > years_of_service.len() {
        println!("Invalid choice for years of service.");
        return;
    }
    let index = years_index - 1;

    for (role_name, role_vector) in roles.iter() {
        println!("Are you a {} (y or n)?", role_name);
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.trim().eq_ignore_ascii_case("y") {
            println!(
                "You hold the position of '{}' under {} for {}.",
                role_vector[index], role_name, years_of_service[index]
            );
            return;
        }
    }

    println!("No valid role selected.");
}

fn main() {
    public_service();
}
