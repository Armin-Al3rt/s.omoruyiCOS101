// Rust program for the student council voter system
use std::io;
fn main(){
	println!("\nStudent Council voters system ");
	//input name
	println!("/nPlease enter your name");
	let mut name = String::new();
    io::stdin();
    read_line(&mut name)
    .expect("Failed to read input");
        let name = name.trim().parse().expect("Failed to load input");
    println!("Your name is:{}",name );

    //input email
    println!("/nPlease enter your email");
    let mut email = String::new();
    io::stdin();read_line(&mut email).expect("Failed to read line");
    let email = email.trim().parse().expect("Failed to load input");
    println!("Email entered: {}",email );

    //input department
    println!("/nPlease enter your department");
    let mut department = String::new();
    io::stdin();
    read_line(&mut department)
    .expect("Failed to read line");
    let department = department.trim().parse().expect("Failed to load input");
    println!("Your department is:{}",department );

    //input state of origin
    println!("/nPlease enter your state of origin");
    io::stdin();
       let mut state_of_origin: = String::new();
       read_line(&mut state_of_origin)
       .expect("Failed to read line");
       let state_of_origin = state_of_origin.trim().parse().expect("Failed to load input");
       println!("Your state of origin is:{}",state_of_origin);

      // input cgpa
      println!("Enter your cgpa: ");
      io::stdin().read_line(&mut cpga).expect("Not a valid string");
     let cgpa:i32 = cgpa.trim().parse().expect("Not a valid number");
     

     if cgpa >=4.0 {
        println!("You can vote{} {} {} {} !", name,email,department,state_of_origin);
    } else {


        println!("Oops, you are not eligible to vote",);
     }






}
//No. 2
use std::io;
fn  main() {
	println!("What is the staff name? ");
	let mut name = String::new();
	io::stdin().read_line(&mut name).expect("Failed to read input");

	println!("Number of papers printed by the staff");
	let mut input1= String::new();
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let z:i32=z.trim().parse().expect("Not a valid number");

	if z >5 && z>3
	println!("The incentive is 500_000");


	if z >5 && z <10
	println!("The incentive is 800_000");

	if z < 3
	println!("The incentive is 100_000");
	
	if z > 10
	println!("The incentive is 1_000_000");
	




  }