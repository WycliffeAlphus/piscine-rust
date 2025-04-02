use std::io;



fn main(){

let mut tries = 0;
let answer = 'e';   
let mut input = String::new();

loop {

input.clear();
println!("{}", "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");    

tries += 1;

io::stdin().read_line(&mut input).expect("Error reading input");

if matching(&input, answer){
    break;
}

}

println!("Number of trials: {}", tries)

}


fn matching(input:&str, answer:char) -> bool {
    input.trim().eq_ignore_ascii_case(&answer.to_string()) || input.trim().eq_ignore_ascii_case(&format!("The letter {}", answer))
}