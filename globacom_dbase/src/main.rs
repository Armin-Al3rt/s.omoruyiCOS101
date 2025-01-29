use std::io::Read;
use std::io;

fn main(){
    println!("Welcome,Enter your role:
1 - Administrator
2 - Manager
3 - Customer
4 - Employee
5 - Vendor");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let user_role: u32 = input.trim().parse().unwrap();
 if user_role == 1{
    globacom_dbase();
 } else if user_role == 2{
    project_table();
 } else if user_role == 3{
    customer_table();
 } else if user_role == 4{
    staff_table();
 } else if user_role== 5{
    dataplan_table();
 }
}

fn globacom_dbase(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn staff_table(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn project_table(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn customer_table(){
    let mut file = std::fs::File::open("customer_table_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn dataplan_table(){
    let mut file = std::fs::File::open("dataplan_table_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
