use std::io;
const VOWELS: &str = "aeiouAEIOU";
fn main() {
    println!("Welcome to the pig latin conversor:");
    
    loop {
        println!("input a word:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Error Reading line");
        println!("{}", pig_latin(input));
    }
}

fn pig_latin(input: String) -> String {
    let input = input.trim();
    let first = input.chars().into_iter().next().expect("No first Char");

    if VOWELS.contains(first){
        format!("{input}-hay")
    } else {
        let input = &input[1..];
        format!("{input}-{first}hay")
    }
}
