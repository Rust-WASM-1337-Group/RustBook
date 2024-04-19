use std::io;
const VOWELS: &str  = "aeiouAEIOU";
fn main() {
    println!("welcome to the pig latin conversor:");

    loop {
        println!("input a word:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Error reading line");
        let input = input.trim();
        println!("{}", pig_latin(input));
    }
}

fn pig_latin(input: &str) -> String {
    let first = input.chars().into_iter().next()
        .expect("first char not found");

    if VOWELS.contains(first){
        format!("{input}-hay")
    } else {
        let input = &input[1..];
        format!("{input}-{first}ay")
    }
} 
