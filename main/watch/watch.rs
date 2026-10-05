fn main() {
    // import input read functionality + object
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();

    // var to hold input as int 
    let mut num: i32 = 0;


    // var to hold result
    let mut res: String = String::new();


    // read input
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // convert input to number 
    num = input.trim().parse::<i32>().expect("Convert to int = Failed");
    


    // find answer 
    res.push_str(&(num / 3600).to_string());
    num -= 3600 * (num / 3600);
    res.push_str(":");
    res.push_str(&(num / 60).to_string());
    num -= 60 * (num / 60);
    res.push_str(":");
    res.push_str(&num.to_string());

    println!("{}", res);

    
}
