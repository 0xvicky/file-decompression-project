use std::fs;
use std::io;


//There is a pattern in rust code, one function is main function and another function is used to cleanup the logic of main2 function
fn main(){
    std::process::exit(main2());//Cleanup the logic
}

fn main2() -> i32{
let args:Vec<_> = std::env::args().collect();//Here we're collecting the arguments

if args.len()<2 {
    println!("Usage: {} <filename>", args[0]);
    return 1;
}



}