
extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;

fn ticketMaker9000(x: i32, y: usize) -> Vec<i32> { // Function to generate a vector of size y, where all values are from 1..=x
    let mut bingoBallz: Vec<i32> = (1..=x).collect();
    let mut RNJesus = rand::thread_rng();
    
    bingoBallz.shuffle(&mut RNJesus);
    bingoBallz.truncate(y); // Limiting the output to size y
    bingoBallz // return
}

fn main() {
    let mut x: i32 = 8; // x for use with fn ticketMaker9000
    let mut y: usize = 3; // y for use with fn ticketMaker9000
    let mut awesomeSauce: i32 = 5; // Number of tickets
    
    let listOrder = ticketMaker9000(10, 8); // Generating balls
    
    println!("Yuor balls: {:?}", listOrder);
    
    while (awesomeSauce > 0) {
        let mut ticket = ticketMaker9000(x, y); // Generating ticket // Vec<Vec<i32>>
        let mut indexLocations: Vec<i32> = Vec::new();  // A vector to save indexAddress values
        let mut j = 0;
        awesomeSauce -= 1;
        
        println!("Yuor Ticket: {:?}", ticket);
        
        while (j < y) {
            let winningNumber: i32 = ticket[j]; // The "winning number" (the value you wish to find the index address of)
            
            let indexAddress = match listOrder.iter().position(|&i| i == winningNumber) { // Searching the vector for winningNumber
                Some(i) => { 
                indexLocations.push(i as i32);} // It exists in the vector 
                , 
                None => {} // It doesn't exist in the vector
            };
            j += 1;
            
            if indexLocations.len() == y {
                indexLocations.sort();
                println!("{:?}", indexLocations);
                 }
    
            
    
        }
        
        indexLocations.clear();
        
    }
    




}

// Code can be run at https://play.rust-lang.org/?version=stable&mode=debug&edition=2021
