fn main()
{
    // import input object + functionality 
    use std::io;
    use std::io::Stdin;

    // import iterator for splitwhitespace
    use std::str::SplitWhitespace;
    // lesliecharleyhorse

    // create input reader object 
    let input_reader: Stdin = io::stdin();

    // var for input
    let mut input: String = String::new();

    // iterator for numbers as string
    let mut nums: SplitWhitespace = "".split_whitespace();


    // vars to hold vals of nums
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;

    // control while loop
    let mut go: i32 = 0;



    while go == 0
    {

    // readline 
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split input into numbers
    nums = input.split_whitespace();


    // assign values 
    num1 = nums.next().expect("Access element = Failed").parse::<i32>().expect("Convert to int = Failed");
    num2 = nums.next().expect("Access element = Failed").parse::<i32>().expect("Convert to int = Failed");


    // conditionals 
    if num1 == 0 && num2 == 0
        {
            go = 1;
            break;
        }

    if num1 < num2
        {
            println!("{} {}", num1, num2);
        }

    else
        {
            println!("{} {}", num2, num1);
        }

    // clearing operations
    input.clear();
    num1 = 0;
    num2 = 0;
    nums = "".split_whitespace();
    

    }

}