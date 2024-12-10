use std::io::Write;
fn main () {
    let student_name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
    let matric_number = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let department = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = vec![300,100,200,200,100];
    
    let mut file = std::fs::File::create("Student_Management_Information_System.txt").expect("create failed");
        
    for i in 0..5 {
        println!("{} {} {} {}",student_name[i],matric_number[i],department[i],level[i]);
        file.write_all(format!("{} {} {} {}",student_name[i],matric_number[i],department[i],level[i]).as_bytes()).expect("write failed");
    }
}
