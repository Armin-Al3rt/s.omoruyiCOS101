use std::io;

fn main() {
    loop {
        println!("\nChoose an option for calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let choice = get_input("Enter your choice (1-6):");
        match choice.trim().parse::<u32>() {
            Ok(1) => {
                let height = get_input("Enter the height:").trim().parse().expect("Invalid input");
                let base1 = get_input("Enter base1:").trim().parse().expect("Invalid input");

                let base2 = get_input("Enter base2:").trim().parse().expect("Invalid input");
                println!("Area of Trapezium: {}", area_of_trapezium(height, base1, base2));
            }
            Ok(2) => {
                let diagonal1 = get_input("Enter diagonal1:").trim().parse().expect("Invalid input");
                let diagonal2 = get_input("Enter diagonal2:").trim().parse().expect("Invalid input");
                println!("Area of Rhombus: {}", area_of_rhombus(diagonal1, diagonal2));
            }
            Ok(3) => {
                let base = get_input("Enter the base:").trim().parse().expect("Invalid input");
                let altitude = get_input("Enter the altitude:").trim().parse().expect("Invalid input");
                println!("Area of Parallelogram: {}", area_of_parallelogram(base, altitude));
            }
            Ok(4) => {
                let side = get_input("Enter the length of the side:").trim().parse().expect("Invalid input");
                println!("Area of Cube: {}", area_of_cube(side));
            }
            Ok(5) => {
                let radius = get_input("Enter the radius:").trim().parse().expect("Invalid input");
                let height = get_input("Enter the height:").trim().parse().expect("Invalid input");
                println!("Volume of Cylinder: {}", volume_of_cylinder(radius, height));
            }
            Ok(6) => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2) * height
}
