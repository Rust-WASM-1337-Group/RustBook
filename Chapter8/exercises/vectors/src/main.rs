use std::{collections::HashMap, io};

fn main() {
    println!("input the vector size:");
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Failed to read line");

    let size: usize = size.trim().parse().expect("Error parsing size");

    let mut vector = Vec::new();

    for i in 0..size {
        let mut temp = String::new();
        println!("input the vector member {}:", i+1);
        io::stdin().read_line(&mut temp).expect("Failed to read line");
        let temp: usize = temp.trim().parse()
            .expect("Error parsing temp: must be an integer");
        vector.push(temp);
    }

    println!("the vector is: {:?}", &vector);
    println!("the median is: {}", median(&vector));
    println!("the mode is: {}", mode(&vector));
}


// Returns the median from a vector
fn median(vector: &Vec<usize>)-> f32{
    let median:usize = vector.len() / 2;
    
    if vector.len() % 2 == 0 {
        (vector.get(median - 1).unwrap() + vector.get(median).unwrap()) as f32 
            / 2.0
    } else {
        *vector.get(median).unwrap() as f32
    }
}
// Returns the mode from a vector
fn mode(vector: &Vec<usize>)-> usize {
    let mut map = HashMap::new();
    let mut mode: usize = 0;

    // count the repetitions
    for i in vector {
        let count = map.entry(i).or_insert(0);
        *count+=1;
    }

    let mut temp: usize = 0;
    for (k, v) in &map {
        if *v >= temp {
            temp = *v;
            mode = **k;
        }
    }

    mode

}

