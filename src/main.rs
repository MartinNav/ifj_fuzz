use std::{
    char,
    fs::{File, OpenOptions},
    io::Write,
    process::Command,
    usize,
};

use rand::{rngs::ThreadRng, Rng, RngCore};

enum Mode {
    Ascii,
    Bytes,
    Tokens,
}
fn generate_random_str_of_rn_len(rng: &mut ThreadRng) -> Vec<u8> {
    const MAX_LEN: usize = 6000;
    let len: usize = (rng.next_u32() as usize) % MAX_LEN;
    let mut res = Vec::with_capacity(len);
    for _ in 0..len {
        res.push(rng.next_u32() as u8);
    }
    res
}
fn generate_random_tokens(rng: &mut ThreadRng) -> Vec<u8> {
    const MAX_TOKS: usize = 1000;
    let len: usize = (rng.next_u32() as usize) % MAX_TOKS;
    // Must add more tokens
    let tokens = ["\"", "@", "'", "f64", "i32","'","return","void"," ",";",")","(",":",".","ifj"," ","var","=","pub","fn","if","else","{","}","const","+","-","\n"];
    //There will be 50% chance of having valid prolog
    let mut prep_str:String = {
        match rng.gen_bool(0.5) {
            true=>
            "const ifj = @import(\"ifj24.zig\");\n".to_string(),
            false=>
        String::new()
        }
    };
    for _ in 0..len {
        prep_str.push_str(tokens[rng.next_u32() as usize % tokens.len()]);
    }
    prep_str.chars().map(|x| (x as u8)).collect::<Vec<u8>>()
}
fn generate_valid_ascii_bytes(rng: &mut ThreadRng) -> Vec<u8> {
    generate_random_str_of_rn_len(rng)
        .iter()
        .map(|x| x & 127)
        .collect()
}
/// This function will return mode in whitch we want our code to be run
/// Compiler binary location
/// Directory for testing files
/// number of testcases to be generated
fn parse_arguments(args: &Vec<String>) -> Option<(Mode, String, String, usize)> {
    if args.len() != 5 {
        return None;
    }
    let mode = match args[1].as_str() {
        "ascii" => Mode::Ascii,
        "bytes" => Mode::Bytes,
        "tokens" => Mode::Tokens,
        _ => Mode::Bytes,
    };
    let compiler_location = args[2].clone();
    let files_directory = args[3].clone();
    if let Ok(v) = args[4].as_str().parse::<usize>() {
        return Some((mode, compiler_location, files_directory, v));
    }
    None
}
static FILE_NAME: &str = "code.zig";
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (mode, executable, dir_name, mut num_of_cycles) =
        parse_arguments(&args).expect("Invalid arguments");
    let generate = match mode {
        Mode::Ascii => generate_valid_ascii_bytes,
        Mode::Tokens => generate_random_tokens,
        _ => generate_random_str_of_rn_len,
    };
    let mut rng = rand::thread_rng();
    File::create(format!("{dir_name}{FILE_NAME}")).expect("file code.zig can not be created");
    loop {
        let random_chars = generate(&mut rng);
        //let mut file = File::open(FILE_NAME).expect("can not open file");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(format!("{dir_name}{FILE_NAME}"))
            .expect("can not open file");
        file.write_all(&random_chars[..])
            .expect("file can not be written");

        let status = Command::new(executable.clone())
            .arg(format!("{dir_name}{FILE_NAME}"))
            .status()
            .expect("ERROR:failed to execute the program");
        if let Some(code) = status.code() {
            if code == 99 {
                let mut failure_case_file =
                    File::create(format!("{dir_name}internal_error_99_{num_of_cycles}.zig"))
                        .unwrap(); //if I fail here so be it.
                failure_case_file.write_all(&random_chars[..]).unwrap();
            }
        } else {
            //here we should have SEGFAULT or error like that
            println!("{num_of_cycles}:signal failure(really bad)");
            let mut failure_case_file =
                File::create(format!("{dir_name}signal_failure_{num_of_cycles}.zig")).unwrap();
            failure_case_file.write_all(&random_chars[..]).unwrap();
        }

        if num_of_cycles == 0 {
            break;
        }
        num_of_cycles -= 1;
    }
}
