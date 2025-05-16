fn main() {
    // Number Data Type
    let x = 6;
    println!("Hello world and {}", x);

    let a = 6;
    let b = 7;
    println!("\n{}", a + b );

    // Constant Data Type must be written in Capital from my understanding
    const PI: f64 = 3.123;
    println!("\n{}", PI );

    // Boolean Data Type for True or False
    let is_name = true;
    print!("\n{}", is_name);

    // Character uses one Quote
    let letter = "A";
    print!("\n\n{}", letter);

    // array    
    let array   = ["Hello", "Hi", "Great"];
    print!("\n{:#?}", array[0]);

    // Tuples: i used tuples to return all my other datas
    let mixture = (PI, is_name, letter, array);
    print!("\n\n{:#?}", mixture.3[2]);

    // Control Flows
    if array.len() == 3 {
        print!("\n Is successful")
    }else {
        print!("\n Not Successful")
    }

    let name = "Christiana".to_string();
    print!("\nv1: {}", name);

    let mut new_name = name.as_str();
    print!("\n\nv2: {}\n", new_name);

    new_name = "Tali";
    print!("\n New v2: {} \n\n", new_name);

    print!("\n name {} \n\n", name);


}
