
use std::env;
use std::fs;
use std::task::Context;

fn day1(part_chosen: &String, content:String) -> u32 {
    let n:Vec<u32> = content
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .collect();

    if part_chosen == "1" {
        for num in &n {
            for num2 in &n {
                if num + num2 == 2020 { return num * num2 }
            }
        }
    }else {

    for num in &n{
        for num2 in &n{
            for num3 in &n{
                if num + num2 + num3 == 2020{ return num * num2 * num3 }
            }
        }
    }}
    0

}

/*fn day2(part_chosen: &String, content:String)->u32{
    
}
*/
fn main() {
    let args: Vec<String> = env::args().collect(); //Note that std::env::args will panic if any argument contains invalid Unicode. If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead.
    // That function returns an iterator that produces OsString values instead of String values.
    // We’ve chosen to use std::env::args here for simplicity because OsString values differ per platform and are more complex to work with than String values.
    //dbg!(args);

    let day = &args[1];
    let part = &args[2];
    let file_path = &args[3];

    let contents = fs::read_to_string(file_path)
        .expect("Remember to add your own input files");

    if day == "1"{
        println!("{}",day1(part,contents));
    }

    //cargo run -- 1 2 src/inputs/day1_input.txt
}
