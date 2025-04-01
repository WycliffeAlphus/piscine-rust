use std::io

fn main() {

   let mut trials = 0; 

   let mut input = String::new();
   
   loop {
    trials += 1;
    io.stdin().read_line(&mut input).expect("Failed to read line");


   }

   println!("Number of trials: {}", trials);

}
