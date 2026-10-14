fn main()
{
    // import input reader library
    use std::io;
    use std::io::Stdin;


    // import splitwhitespace library
    use std::str::SplitWhitespace;




    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();

    // var to hold total numbers to loop thru
    let mut total_nums: i32 = 0;

    // array to hold min, max, sum
    let mut res: [i64; 3] = [0, 1000000000000, 0];

    // bool to set array on first read
    let mut go: bool = true;

    // var to hold split string values
    let mut nums_string: SplitWhitespace = "".split_whitespace();




    // read total numbers to loop thru and assign val to input var
    //lesliecharleyhorse
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // convert input value to integer and assign val to nums
    total_nums = input.trim().parse::<i32>().expect("Convert to i32 = Failed");


    // clear input string 
    input.clear();


    // read line of numbers and assign value to input vars
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split string and assign val to nums
    nums_string = input.split_whitespace();




    // for loop to loop thru potential values; 
    for num in 0..total_nums
        {
            // temp var to hold int value of string
            let mut temp_num: i64 = 0;

            // assign value of converted string to temp_num
            temp_num = nums_string.next().expect("Access index = Failed").parse::<i64>().expect("Convert to i32 = Failed");

            // add number to sum
            res[2] += temp_num;


            // check if first number being read if so assign min and max to that number
            if go == true && total_nums == 1
                {
                    res[0] = temp_num;
                    res[1] = temp_num;
                    
                }

            else
                {    
                    // check if temp_num is new max or the new min and reassign 
                    if temp_num > res[0]
                        {res[0] = temp_num;}
                    
                    if temp_num < res[1]
                        {res[1] = temp_num;}
                }

            
        }


    // print result
    println!("{} {} {}", res[1], res[0], res[2]);




}