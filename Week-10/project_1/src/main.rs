struct Devices{
brand:String,
price:f32,
quantity:u32,
}

fn  main() {
    let dvc1 = Devices{
    brand:String::from("HP"),
    quantity:10,
    price:650000.0
    };

    let dvc2 = Devices{
    brand:String::from("IBM"),
    quantity:6,
    price:755000.0
    };

    let dvc3 = Devices{
    brand:String::from("Toshiba"),
    quantity:10,
    price:550000.0
    };

    let dvc4 = Devices{
    brand:String::from("Dell"),
    quantity:4,
    price:850000.0
    };

    let sum = dvc1.price * 3.0 + dvc2.price * 3.0 + dvc3.price * 3.0 + dvc4.price * 3.0; 

    println!("The total cost of your devices is: {}",sum);
}
