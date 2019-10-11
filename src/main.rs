use std::env;
use std::process;

use rs9cc::libc;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let result_fst_num = libc::strtol(&args[1], 10);
    if result_fst_num.is_err() {
        eprintln!("{}", result_fst_num.err().unwrap());
        process::exit(1)
    }

    let mut rest_s: String;
    let (num, tmp_s) = result_fst_num.ok().unwrap();
    rest_s = tmp_s;
    println!("  mov rax, {}", num);

    while !rest_s.is_empty() {
        if &rest_s[0..1] == "+" {
            let result = libc::strtol(&String::from(&rest_s[1..]), 10);
            if result.is_err() {
                eprintln!("{}", result.err().unwrap());
                process::exit(1)
            }

            let (num, s) = result.ok().unwrap();
            rest_s = s;
            println!("  add rax, {}", num);

            continue;
        }

        if &rest_s[0..1] == "-" {
            let result = libc::strtol(&String::from(&rest_s[1..]), 10);
            if result.is_err() {
                eprintln!("{}", result.err().unwrap());
                process::exit(1)
            }

            let (num, s) = result.ok().unwrap();
            rest_s = s;
            println!("  sub rax, {}", num);

            continue;
        }

        eprintln!("予期しない文字列です: [{}]", rest_s);
        process::exit(1)
    }

    println!("  ret");
    process::exit(0);
}
