fn main() 
{
    // import input read functionality
    use std::io;
    use std::io::Stdin;

    // import splitwhitespace string object
    use std::str::SplitWhitespace;


    // declare input variable
    let mut input: String = String::new();
    
    // declare input reader object
    let input_reader: Stdin = io::stdin();

    // declare variables to hold values of 2 numbers
    let mut a: i16 = 0;
    let mut b: i16 = 0;

    // declare iterator to hold values removing whitespace
    let mut nums: SplitWhitespace = "".split_whitespace();


    // readline and hold alue in input
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split input into iterator of values removing whitepsace
    nums = input.split_whitespace();


    // assign values
    a = nums.next().expect("Access element = Failed").parse::<i16>().expect("Convert to int = Failed");
    b = nums.next().expect("Access element = Failed").parse::<i16>().expect("Convert to int = Failed");


    // conditional chain to find answer
    if a < b
        {
            println!("a < b");
        }
    else if a > b
        {
            println!("a > b");
        }
    else
        {
            println!("a == b");
        }
    
    

}