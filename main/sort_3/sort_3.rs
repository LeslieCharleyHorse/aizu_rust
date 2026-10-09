fn main()
{
    // import input reader object and functionality
    use std::io;
    use std::io::Stdin;


    // import Splitwhitepsace object and functionality
    use std::str::SplitWhitespace;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();


    // create Splitwhitespace object to hold nums
    let mut nums_str: SplitWhitespace = "".split_whitespace();

    // vector to hold + sort nums
    let mut nums_int: Vec<i32> = vec![];



    // readline and assign value to input
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // lesliecharleyhorse



    // split input by spaces
    nums_str = input.split_whitespace();


    // go thru nums and add int values to vector
    for num in nums_str
        {
            nums_int.push(num.parse::<i32>().expect("Convert to int = Failed"));
        }


    // sort numbers from desc to insc
    nums_int.sort();



    // var to hold result
    let mut res: String = String::new();


    // add sorted vector to result
    for num in nums_int
        {
            
            res.push_str(&num.to_string());
            res.push_str(" ");
        }

    // remove final space
    res.pop();


    // result

    println!("{}", res);


    



}