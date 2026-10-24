fn main()
{
    // import input + output crate 
    use std::io;
    use std::io::Stdin;


    // var to hold res
    let mut res: u32 = 0;

    // var to hold input
    let mut input: String = String::new();

    // create input reader object
    let mut input_reader: Stdin = io::stdin();


    // var to control while loop
    //lesliecharleyhorse
    let mut go: bool = true;


    // while looop
    while go == true
        {
            // read input and assign val to input var
            input_reader.read_line(&mut input).expect("Readline = Failed");


            // asssign 0 to res
            res = 0;

            // clean up string 
            input = input.trim().to_string();


            // loop thru the digits of a cleaned input number
            for dig in input.chars()
                {
                    // add digit to res
                    res += dig.to_digit(10).expect("Convert to digit = Failed");
                }

    
            // break loop 
            if res == 0
                {
                    go = false;
                }
            else
                {
                    // print result and empty input
                    println!("{res}");
                    input.clear();
                }

        }
        

}


