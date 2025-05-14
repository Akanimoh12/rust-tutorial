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

    
    let array   = ["Hello", "Hi", "Great"];
    print!("\n{:#?}", array[0]);

    let mixture = (PI, is_name, letter, array);
    print!("\n\n{:#?}", mixture.3[2]);
}
