
use std::env;
use std::fs;

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

fn day2(part_chosen: &String, content:String)->u32{
    let entries:Vec<(u32,u32,char,&str)> = content
        .lines()
        .filter_map(|line|{
            let parts: Vec<&str> = line.split_whitespace().collect();
            let range: Vec<&str> = parts[0].split('-').collect();

            let start = range[0].parse::<u32>().ok()?;
            let end = range[1].parse::<u32>().ok()?;
            let letter = parts[1].chars().next()?;
            let password = parts[2];

            Some((start,end,letter,password))

        })
        .collect();


    if part_chosen == "1"{
        let mut valid:u32 = 0;
        for entry in entries{
            let mut count:u32 = 0;
            for char in entry.3.chars(){
                if char == entry.2{ count += 1 }
            }
            if entry.0<= count && count<= entry.1  { valid +=1 }
        }
        valid

    }else {
        let mut valid:u32 = 0;
        for entry in entries{
            let chars: Vec<char> = entry.3.chars().collect();
            if (chars.get((entry.0 as usize)-1) == Some(&entry.2)) ^ (chars.get((entry.1 as usize)-1) == Some(&entry.2)) {valid +=1}
        }
        valid
    }

}

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
    }else if day == "2" {
        println!("{}", day2(part,contents));
    }

    //cargo run -- 2 2 src/inputs/day2_input.txt
}
