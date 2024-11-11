use std::{char, fs::File, io::Write, process::Command};

use rand::{rngs::ThreadRng, RngCore};

fn generate_random_str_of_rn_len(rng:&mut ThreadRng)->Vec<u8>{
    const MAX_LEN:usize=6000;
    let len:usize = (rng.next_u32()as usize)%MAX_LEN;
    let mut res = Vec::with_capacity(len);
    for _ in 0..len {
        res.push(rng.next_u32()as u8);
    }
    res
}
static FILE_NAME:&str="code.zig";
static EXECUTABLE:&str="execut";
fn main() {
    let mut num_of_cycles=6669;
    let mut rng = rand::thread_rng();
    File::create(FILE_NAME).expect("file code.zig can not be created");
    loop {
        let random_chars =generate_random_str_of_rn_len(&mut rng);
        let mut file = File::open(FILE_NAME).expect("can not open file");
        file.write_all(&random_chars[..]).expect("file can not be written");
        
        let status = Command::new(EXECUTABLE).arg(FILE_NAME).status().expect("ERROR:failed to execute the program");
        if let Some(code)=status.code() {
            if code==99 {
               let mut failure_case_file = File::create(format!("internal_error_99_{num_of_cycles}.zig")).unwrap();//if I fail here so be it.
                failure_case_file.write_all(&random_chars[..]).unwrap();
            }
        }else {
            //here we should have SEGFAULT or error like that
            println!("{num_of_cycles}:signal failure(really bad)");
            let mut failure_case_file = File::create(format!("signal_failure{num_of_cycles}.zig")).unwrap();
            failure_case_file.write_all(&random_chars[..]).unwrap();

        }

        
        if num_of_cycles==0 {
            break;
        }
        num_of_cycles-=1;
    }
}
