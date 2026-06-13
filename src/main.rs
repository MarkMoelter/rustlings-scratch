fn main() {
    // Day 2 - Variables, Mutability, and Types
    // let name: &str = "Mark";
    // let age: u32 = 123;
    // let height_meters: f32 = 3.5;
    // let is_devops: bool = true;

    // println!("{name}, {age}, {height_meters}, {is_devops}");

    // Day 3 - FizzBuzz
    let iter = 20;

    // for i in 1..=iter {
    //     let mut output = i.to_string();

    //     if i % 3 == 0 {
    //         output = String::from("Fizz");
    //     }
    //     if i % 5 == 0 {
    //         output = String::from("Buzz");
    //     }
    //     if i % 3 == 0 && i % 5 == 0 {
    //         output = String::from("FizzBuzz");
    //     }

    //     println!("{output}")
    // }

    for i in 1..=iter {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{i}"),
        }
    }
}
