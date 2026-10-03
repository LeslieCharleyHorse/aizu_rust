fn main()
{
    // import input reading functionality
    use std::io;
    use std::io::Stdin;

    // import splitwhitespace object functionaltiy 
    use std::str::SplitWhitespace;


    // declare input reader object
    let input_reader: Stdin = io::stdin();

    // declare variable to hold input line
    let mut input: String = String::new();

    //  declare iterator to hold individual values
    let mut nums: SplitWhitespace = "".split_whitespace();

    // declare variables to hold values as integers
    let mut num_1: i8 = 0;
    let mut num_2: i8 = 0;
    let mut num_3: i8 = 0;


    // readline and hold value in input
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split input into individual values
    nums = input.split_whitespace();

    // LeslieCharleyHorse


    // assign values
    num_1 = nums.next().expect("Access element").parse::<i8>().expect("Convert to integer = Failed");
    num_2 = nums.next().expect("Access element").parse::<i8>().expect("Convert to integer = Failed");
    num_3 = nums.next().expect("Access element").parse::<i8>().expect("Convert to integer = Failed");


    // conditional chain to find anwer
    if num_1 < num_2
        {
            if num_2 < num_3
                {
                    println!("Yes");
                }
            else
                {
                    println!("No");
                }
        }
    else 
        {
            println!("No");
        }
    



    



}