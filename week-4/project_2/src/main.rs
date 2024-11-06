//Rust program to determine the annual incentive
// of an employee based on experience and age

use std::io;

fn 
calcuate_incentive(experienced:
    bool,age: u32) -> u32 {
    if experienced {
        if age >= 40 {
            1_560_000
        } else if age >=30 && age < 40 {
            1_480_000
        } else if age < 28 {
            1_300_000 * 12
        } else {
            0
        }
    } else {
        100_000
    }
}

 fn main() {
    println!("Is the employee experienced? (yes/no)");
    let mut experience_input =String::new();
    
    io::stdin().read_line(&mut experience_input).expect("Failed to read input ");
    let experienced = experience_input.trim().eq_ignore_ascii_case("yes");
    

    println!("What is the age of the employee?");
    let mut age_input =  String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age:u32 = age_input.trim().parse().expect("Please enter a valid age ");

    

    let incentive=calcuate_incentive(experienced,age);
    println!("The annual incentive for the employee is {}", incentive);

    
}
    
    

    





