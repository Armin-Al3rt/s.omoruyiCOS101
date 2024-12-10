use std::io::Read;
fn main(){
    let mut name_of_commisioners_file = std::fs::File::open("Name of commisioners.txt").unwrap();
    let mut name_of_commisioners_contents = String::new();
    name_of_commisioners_file.read_to_string(&mut name_of_commisioners_contents).unwrap();
    print!("{}", name_of_commisioners_contents);

    let mut geopolitical_zone_file = std::fs::File::open("Geopolitical Zone.txt").unwrap();
    let mut geopolitical_zone_contents = String::new();
    geopolitical_zone_file.read_to_string(&mut geopolitical_zone_contents).unwrap();
    print!("{}", geopolitical_zone_contents);

    let mut ministry_file = std::fs::File::open("Ministry.txt").unwrap();
    let mut ministry_contents = String::new();
    ministry_file.read_to_string(&mut ministry_contents).unwrap();
    print!("{}", ministry_contents);

    let name_of_commisioners_vec: Vec<&str> = name_of_commisioners_contents.split("\n").collect();
    let geopolitical_zone_vec: Vec<&str> = geopolitical_zone_contents.split("\n").collect();
    let ministry_vec: Vec<&str> =  ministry_contents.split("\n").collect();
    for i in 0..name_of_commisioners_vec.len() {
        println!("{} {} {}",name_of_commisioners_vec[i],geopolitical_zone_vec[i],ministry_vec[i]);
    }

}