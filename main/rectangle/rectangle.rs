fn main() 
{
    // import standard library input reading functionality
    use std::io;
    use std::io::Stdin;


    // import standard library string iterator object of splitwhitespace
    use std::str::SplitWhitespace;


    //create input reader object
    let input_reader: Stdin = io::stdin();

    // create input variable to hold input 
    let mut input: String = String::new();

    // create iterator to hold values split by whitespace
    let mut nums: SplitWhitespace = "".split_whitespace();

    // create variables to hold length and width
    let mut len: i32 = 0;
    let mut wid: i32 = 0;


    // read line of input and store input in input variable
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split input by whitespace
    nums = input.split_whitespace();


    // assign values 
    len = nums.next().expect("Acces element = Failed").parse::<i32>().expect("Convert to number = Failed");

    wid = nums.next().expect("Acces elemt = Failed").parse::<i32>().expect("Convert to number = Failed");


    // print area and perimeter of rectangle
    println!("{} {}", len*wid, (len+len) + (wid+wid));


}