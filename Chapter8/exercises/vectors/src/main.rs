use std::{collections::HashMap, io};

fn main() {
    println!("input the vector size:");
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Failed to read line");

    let size: usize = size.trim().parse().expect("Error parsing size");

    let mut vector = Vec::new();
    
    // read vector: items
    for i in 0..size {
        let mut temp = String::new();
        println!("inpuit the vector item {}", i+1);
        io::stdin().read_line(&mut temp).expect("Error reading line");
        let temp_item: usize = temp.trim().parse()
            .expect("Error parsing temp_item: must be an integer");
        vector.push(temp_item);
    }
    // Print results
    println!("the vector is: {:?}", &vector);
    println!("the median is: {}", median(&vector));
    println!("the mode is: {}", mode(&vector));

}

// function median
fn median(vector: &Vec<usize>) -> f32 {
    let median_index: usize = vector.len() / 2; 
    if vector.len() % 2 == 0 {
        (vector.get(median_index - 1).expect("Index not found") 
         + vector.get(median_index).expect("Index not found")) as f32 / 2.0
    } else {
        *vector.get(median_index).expect("Index not found") as f32
    }
}

// function mode
fn mode(vector: &Vec<usize>) -> usize {
    // declare hashmap
    let mut map = HashMap::new();
    // declare mode
    let mut mode: usize = 0;
    
    // count the repetitions
    for i in vector {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
    
    // get the mode (most repeated)
    let mut temp: usize = 0;
    for (k, v) in map {
        if v >= temp {
            temp = v;
            mode = *k;
        }
    } 

    mode 
}

// function mode p1r0
//fn mode(vector: &Vec<usize>)-> usize {
//    let mut map = HashMap::new();
//    let mut mode: usize = 0;
//
//    // count the repetitions
//    for i in vector {
//        let count = map.entry(i).or_insert(0);
//        *count+=1;
//    }
//
//    let mut temp: usize = 0;
//    for (k, v) in &map {
//        if *v >= temp {
//            temp = *v;
//            mode = **k;
//        }
//    } 
//
//    mode
//
//}


