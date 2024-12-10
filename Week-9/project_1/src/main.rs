use std::fs::File;
use std::io;
use std::io::Write;



fn main() {
    
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    
    let mut file = std::fs::File::create("drinks_categories.txt").unwrap();

    
    writeln!(file, "Lager,Stout,Non-Alcoholic").unwrap();

    
    let max_length = std::cmp::max(lager.len(), std::cmp::max(stout.len(), non_alcoholic.len()));

    
    for i in 0..max_length {
        let lager_item = lager.get(i).unwrap_or(&"");
        let stout_item = stout.get(i).unwrap_or(&"");
        let non_alcoholic_item = non_alcoholic.get(i).unwrap_or(&"");
        writeln!(file, "{},{},{}", lager_item, stout_item, non_alcoholic_item).unwrap();
    }

    println!("File 'drinks_categories.txt' created successfully.");
    
}
