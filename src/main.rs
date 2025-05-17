#[derive(Debug)]
struct Student {
        name: String,
        reg_number: u32,
        age: u8,
        gender: String,
        is_graduate: bool,
        email: String,
    }

impl Student {

    // Normal method
    fn student1(name: String,
        reg_number: u32,
        age: u8,
        gender: String,
        is_graduate: bool,
        email: String) -> Student {
            Student{name, reg_number, age, gender, is_graduate, email}
    }

    // Associative Meethod
    fn print(&self) {
        print!("{} \n {}\n {}\n {}\n {}\n {}", self.name, self.reg_number, self.age, self.gender, self.is_graduate, self.email)
    }

}

fn main() {

    // Using Impl to pass a function 
    // Student.print();


    // Using Normal Struct to passing the values
    let student = Student {
    name: String::from("Akanimoh"),
    reg_number:1,
    age:16,
    gender: String::from("Male"),
    is_graduate: true,
    email:"akanimohjohnson2003@gmail.com".to_string(),
    };

    print!("Information of : {:#?}", student);



    // // Number Data Type
    // let x = 6;
    // println!("Hello world and {}", x);

    // let a = 6;
    // let b = 7;
    // println!("\n{}", a + b );

    // // Constant Data Type must be written in Capital from my understanding
    // const PI: f64 = 3.123;
    // println!("\n{}", PI );

    // // Boolean Data Type for True or False
    // let is_name = true;
    // print!("\n{}", is_name);

    // // Character uses one Quote
    // let letter = "A";
    // print!("\n\n{}", letter);

    // // array    
    // let array   = ["Hello", "Hi", "Great"];
    // print!("\n{:#?}", array[0]);

    // // Tuples: i used tuples to return all my other datas
    // let mixture = (PI, is_name, letter, array);
    // print!("\n\n{:#?}", mixture.3[2]);

    // // Control Flows
    // if array.len() == 3 {
    //     print!("\n Is successful")
    // }else {
    //     print!("\n Not Successful")
    // }

    // let name = "Christiana".to_string();
    // print!("\nv1: {}", name);

    // let mut new_name = name.as_str();
    // print!("\n\nv2: {}\n", new_name);

    // new_name = "Tali";
    // print!("\n New v2: {} \n\n", new_name);

    print!("\n\n");

    


}
