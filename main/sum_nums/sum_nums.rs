fn main()
{
    // import input + output library 
    use std::io;


    // var to input
    let mut input: String = String::new();

    // var control while loop
    let mut go: i8 = 1;


    // lesliecharleyhorse




// wile loop until 0
    while go > 0
        {

            input.clear();

            // read  number in and assign val to input
            io::stdin().read_line(&mut input).expect("Readline = Failed");



            // make sure not end of file
            if input.trim().to_string() != "0"
                {
                    // var to hold res
                    let mut res: i32 = 0;

                    // loop thru each digit and add to res
                    for digit in input.trim().chars()
                        {
                            
                            // check if character is numeric to catch errors
                            if digit.is_numeric()
                            {
                            
                            // add converted value to res
                            res += digit.to_digit(10).unwrap() as i32;

                            }

                        }

                    
                    // print the res
                    println!("{res}");



                }


            // break while loop
            else
                {
                    go = 0;
                    break;
                }

        }
}


