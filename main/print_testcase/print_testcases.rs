fn main()
{
    // import input object and functionality 
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();
    //lesliecharleyhorse


    // var to hold input
    let mut input: String = String::new();


    // var to control while loop flow 
    let mut num: i8 = 1;

    // var to keep track of testcases
    let mut test: i32 = 1;


    // while loop to find answer
    while num > 0
        {
            // readline
            input_reader.read_line(&mut input).expect("Readline = Failed");

            // check if to stop program 
            if input.trim() == "0".to_string()
                {
                    num = 0;
                    break;
                }

            else
                {
                    // print result 
                    println!("Case {}: {}", test, input.trim());

                    // clear input to read next  number
                    input.clear();

                    // increase test case by 1
                    test += 1;
                }    

        }

}