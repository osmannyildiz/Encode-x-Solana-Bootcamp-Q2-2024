fn main() {
    println!("> Welcome to the fizz buzz program.");
    println!("> Here we go!");

    let result = fizz_buzz();

    println!("> We've reached the end. Fizz buzz count: {}", result);
}

fn fizz_buzz() -> u32 {
    let mut fizz_buzz_count = 0u32;

    for i in 1..301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{} (Fizz buzz!)", i);
            fizz_buzz_count += 1;
        } else if i % 3 == 0 {
            println!("{} (Fizz!)", i);
        } else if i % 5 == 0 {
            println!("{} (Buzz!)", i);
        } else {
            println!("{}", i);
        }
    }

    fizz_buzz_count
}
