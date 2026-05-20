use chrono::{Local};

fn main() {
    /* RBE1 */
    let now = Local::now();
    println!("Hello, world! It's {} local time now", now);
    println!("Do you like {0} or {1}? I like {0}", "running", "swimming");

    println!{"Base 10: {}", 99}
    println!{"Base 2: {:b}", 99}
    println!{"Base 8: {:o}", 99}
    println!{"Base 16: {:x}", 99}

    #[allow(dead_code)] 
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));

    let precision = 2;
    // "1$" refers to the second arg, i.e. precision
    // {:.n} controls the number of digits after the decimal point
    println!{"Pi is roughly {:.1$}", std::f64::consts::PI, precision}
}
