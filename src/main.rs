use core::fmt;

struct Server {
    name: String,
    cpu_cores: u32,
    is_active: bool,
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrafficLight::Green => write!(f, "GO"),
            TrafficLight::Yellow => write!(f, "SLOW"),
            TrafficLight::Red => write!(f, "STOP"),
        }
    }
}

fn main() {
    // Day 1 - Hello World & Toolchain
    // println!("Hello, world!");

    // Day 2 - Variables, Mutability, and Types
    // let name: &str = "Mark";
    // let age: u32 = 123;
    // let height_meters: f32 = 3.5;
    // let is_devops: bool = true;
    // println!("{name}, {age}, {height_meters}, {is_devops}");

    // Day 3 - FizzBuzz
    // let iter = 20;
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
    // for i in 1..=iter {
    //     match (i % 3, i % 5) {
    //         (0, 0) => println!("FizzBuzz"),
    //         (0, _) => println!("Fizz"),
    //         (_, 0) => println!("Buzz"),
    //         _ => println!("{i}"),
    //     }
    // }

    // Day 4 - Functions
    // for i in 2..=20 {
    //     if is_prime(i) {
    //         println!("{i}");
    //     }
    // }

    // Day 5 - Collections — Arrays, Vectors
    // let v: Vec<i32> = (1..=10).collect();
    // let sum: i32 = v.iter().sum();
    // let avg: f64 = sum as f64 / v.len() as f64;
    // let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    // println!("sum={sum}, avg={avg}, evens={evens:?}");

    // Day 6 - Ownership & Borrowing
    // let s1 = "testing testing";
    // let s2 = "testing";
    // println!("{}", longest(&s1, &s2))

    // Day 7 - Structs
    // My way
    // let servers: Vec<Server> = vec![
    //     Server {
    //         name: "web-01".into(),
    //         cpu_cores: 4,
    //         is_active: true,
    //     },
    //     Server {
    //         name: "web-02".into(),
    //         cpu_cores: 4,
    //         is_active: false,
    //     },
    //     Server {
    //         name: "backend-01".into(),
    //         cpu_cores: 16,
    //         is_active: true,
    //     },
    // ];
    // let mut num_active = 0;
    // for s in &servers {
    //     println!("{} has {} cores", s.name, s.cpu_cores);
    //     if s.is_active {
    //         num_active += 1;
    //     }
    // }
    // println!("{} servers active", num_active);
    // More rustic way
    // let active = servers.iter().filter(|s| s.is_active).count();
    // println!("active: {active}");

    // Day 8 - Enums
    // let lights: Vec<TrafficLight> =
    //     vec![TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];
    // for i in lights {
    //     let next = next_traffic_light(&i);
    //     println!("Current: {}; Next: {}", i, next)
    // }
}

// Day 4
fn is_prime(num: i32) -> bool {
    if num < 2 {
        return false;
    }

    for i in 2..num {
        if i * i > num {
            break;
        }
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Day 6
fn longest(s1: &str, s2: &str) -> usize {
    if s1.len() > s2.len() {
        s1.len()
    } else {
        s2.len()
    }
}

// Day 8
fn next_traffic_light(light: &TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Green => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Red => TrafficLight::Green,
    }
}
