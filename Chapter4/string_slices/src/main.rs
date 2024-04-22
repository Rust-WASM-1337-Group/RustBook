fn main() {
    // String slice reference
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    // analogy start
    //let s = String::from("hello");

    let slice = &s[0..2];

    println!("{}",slice);
    let slice = &s[..2];

    println!("{}",slice);

    // analogy end
    let len = s.len();

    let slice = &s[3..len];
    println!("{}",slice);
    let slice = &s[3..];
    println!("{}",slice);
    
    // full analogy
    let slice = &s[0..len];
    println!("{}",slice);
    let slice = &s[..];
    println!("{}",slice);

    // rewrite first_word with &str
    let first = first_word(&s);
    println!("{}",first); 

    // Compile Error
    //let mut s = String::from("hello world");

    //let word = first_word(&s);

    //s.clear(); // error!

    //println!("the first word is: {}", word);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
