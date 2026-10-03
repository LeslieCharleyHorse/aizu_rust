fn main() {
    // get standard library read input functionality
    use std::io;
    use std::io::Stdin;


    // create variables to hold input, input reader object, hold result as a integer 
    let input_reader: Stdin = io::stdin();
    let mut input: String = String::new();
    let mut res: i64 = 0;


    // readline and store number(String) into input 
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // convert string into integer 
    res = input.trim().parse::<i64>().expect("Convert to int = failed");


    // print the resut cubed 
    println!("{}", res.pow(3));

}