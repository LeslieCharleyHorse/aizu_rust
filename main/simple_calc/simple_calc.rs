fn main()
{
    // import input object + functionslity
    use std::io;
    use std::io::Stdin;


    // lesliecharleyhorse
    //import splithwhitespace object
    use std::str::SplitWhitespace;


    // create input reader object
    let input_reader: Stdin = io::stdin();



    // var to control while loop
    let mut go: i8 = 1;

    // while loop
    while go > 0
        {
            // var to hold input 
            let mut input: String = String::new();

            // var to hold split input
            let mut input_split: SplitWhitespace = "".split_whitespace();

            // var to hold result
            let mut res: i32 = 0;

            // var to hold operation 
            let mut op: String = String::new();

            // var to hold value of 2nd number
            let mut num_2: i32 = 0;





            // read input
            input_reader.read_line(&mut input).expect("Readline = Failed");


            // split input by space and assign val to input_split var
            input_split = input.split_whitespace();


            // assign first number value to res
            res = input_split.next().expect("Access element = Failed").parse::<i32>().expect("Convert to i32 = Failed");


            // assign val of operation to op var
            op = String::from(input_split.next().expect("Access index = Failed"));


            // assign val of 2nd number to num_2 var
            num_2 = input_split.next().expect("Access element = Failed").parse::<i32>().expect("Convert to i32 = Failed");



            // conditionals check operation then find answer
            if op.eq("+")
                {
                    println!("{}", res + num_2);       
                }

            else if op.eq("-")
                {
                    println!("{}", res - num_2);       
                }

            else if op.eq("*")
                {
                    println!("{}", res * num_2);       
                }

            else if op.eq("/")
                {
                    println!("{}", res / num_2);       
                }
            
            else
                {
                    go = 0;
                    break
                }         
        }
}


