use std::io;

fn main() {
    loop {
        let mut buffer = String::new();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading from std in");

        if let Ok(age) = buffer.trim().parse::<u32>() {
            println!("Your Age in days is {}", age * 365);
            break;
        } else {
            println!("Not a valid age!");
        }
    }
}
