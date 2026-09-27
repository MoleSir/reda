use reda_unit::{num, Number, Suffix};

fn main() {
    let _ = Number::new(1.5, Suffix::Kilo); 
    let a = num!(1.5 k);
    let b = num!(500);

    println!("a = {}", a);
    println!("b = {}", b);

    let sum = a + b;
    println!("Sum in base unit: {}", sum);

    println!("a with suffix = {}", a);
}
