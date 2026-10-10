fn main()
{
    // import input reader object and functionality 
    use std::io;
    use std::io::Stdin;


    // import spliwhitespace object and functionality
    use std::str::SplitWhitespace;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var for input
    let mut input: String = String::new();


    // var to hold split values
    let mut nums: SplitWhitespace = "".split_whitespace();

    // vector to hold values
    let mut nums_int: Vec<i32> = vec![0, 0, 0];


    // var to hold result
    let mut res: i32 = 0;


    // readline
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split input
    nums = input.split_whitespace();


    // assign values
    for num in 0..3
        {
            nums_int[num] = nums.next().expect("Access element = Failed").parse::<i32>().expect("Convert to int = Failed");
        }

    
    // find answer
    for num in nums_int[0]..=nums_int[1]
        {
            if nums_int[2] % num == 0
                {
                    res += 1;
                }
        }

    
    // print answer
    // lesliecharleyhorse
    println!("{}", res);
    
}