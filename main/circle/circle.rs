fn main()
{
    // import input reader object and functionality
    use std::io;
    use std::io::Stdin;


    // import pi
    use std::f64::consts::PI;



    
    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();

    // var to hold float val of input
    let mut input_int: f64 = 0.0;



    // read number and assign val to input var
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // convert string to int
    input_int = input.trim().parse::<f64>().expect("Convert to int = Failed");


    // print result
    println!("{} {}", PI * input_int*input_int, 2.0 * PI * input_int);
}




