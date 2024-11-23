use std::io;

fn main() {
    //Displaying the menu
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - $3_200");
    println!("F = Fried Rice & Chicken - $3_000");
    println!("A = Amala & Ewedu Soup - $2_500");
    println!("E = Eba & Egusi Soup - $2_000");
    println!("W = White Rice & Stew - $2_500");

    //Getting input from the user for food type and quantity
    let mut food_choice = String::new();
    println!("Enter the type of food(P,F,A,E,W):");
    io::stdin().read_line(&mut food_choice).expect("Failed to read input");
    let food_choice = food_choice.trim().to_uppercase();

    let mut quantity_input = String::new();
    println!("Enter the quantity:");
    io::stdin().read_line(&mut quantity_input).expect("Failed to read input");

    let quantity:u32 = quantity_input.trim().parse().expect("Invalid quantity");

    //Defining the prices
    let price = match food_choice.as_str(){
        "P"=> 3200,
        "F"=> 3000,
        "A"=> 2500,
        "E"=> 2000,
        "W"=> 2500,
        _=>{
            println!("Invalid food choice");
            return;
        }
    };

    //Calculating total cost
    let mut total_cost = price * quantity;

    //Applying discount if the total cost is greater than $10_000
    if total_cost > 10_000{
        total_cost = (total_cost as f64 * 0.95) as u32;//Applying 5% discount
    }

    //Displaying the total charges
    println!("The total charges for your order are ${}",total_cost );

    
}
