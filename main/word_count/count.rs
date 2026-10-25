fn main()
{
    // import input + output librarry
    use std::io;
    use std::io::Stdin;

    // import SplitWhitespace library
    use std::str::SplitWhitespace;


    // input reader object
    let mut input_read: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();

    // var to hold result tuple (key word, result count)
    let mut res: (String, i64) = (String::new(), 0);

    // read in keyword and assign val to res
    input_read.read_line(&mut res.0).expect("Readline = Failed");

    res.0 = String::from(res.0.trim());


    // var to control loop
    let mut go: bool = true;
    while go == true
        {

            // clear input
            input.clear();

            // read input and assign val to input var
            input_read.read_line(&mut input).expect("Readline = Failed");


            // var to hold split input
            let mut input_split: SplitWhitespace = input.split_whitespace();

            // loop thru words and check for match
            for word in input_split
                {
                    //lesliecharleyhorse
                    if res.0.to_lowercase() == word.trim().to_lowercase()
                        {
                            res.1 += 1;
                        }

                    else if word.trim() == String::from("END_OF_TEXT")
                        {
                            go = false;
                            break;
                        }

                }

            
        }

     // result   

    println!("{}", res.1);
 
}

