// Day 14 - Modules
pub fn fizz_buzz(i: i32) -> String {
    let mut output = i.to_string();
    if i % 3 == 0 {
        output = String::from("Fizz");
    }
    if i % 5 == 0 {
        output = String::from("Buzz");
    }
    if i % 3 == 0 && i % 5 == 0 {
        output = String::from("FizzBuzz");
    }
    output
}
