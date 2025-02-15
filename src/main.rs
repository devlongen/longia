use std::io;

fn main() {
    println!("Staring learn rust");

    println!("Text:"); let mut text = String::new();
    io::stdin()
        .read_line(&mut text);
    println!("You text:{}" ,text);
}