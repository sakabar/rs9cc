use std::env;
use std::process;

use rs9cc::tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut tokenizer = tokenizer::Tokenizer::new(&args[1]);
    let fst_num_op = tokenizer.expect_number();
    if fst_num_op.is_none() {
        eprintln!(
            "{}",
            tokenizer.error_at_cur("最初のトークンが数字ではありません")
        );

        process::exit(1)
    }

    let fst_num = fst_num_op.unwrap();
    println!("  mov rax, {}", fst_num);

    while !tokenizer.expect_eof() {
        if tokenizer.expect_op("+") {
            let num_op = tokenizer.expect_number();
            if num_op.is_none() {
                eprintln!("{}", tokenizer.error_at_cur("予期しない文字列です"));
                process::exit(1);
            }

            let num = num_op.unwrap();
            println!("  add rax, {}", num);
            continue;
        } else if tokenizer.expect_op("-") {
            let num_op = tokenizer.expect_number();
            if num_op.is_none() {
                eprintln!("{}", tokenizer.error_at_cur("予期しない文字列です"));
                process::exit(1);
            }

            let num = num_op.unwrap();
            println!("  sub rax, {}", num);
            continue;
        } else {
            eprintln!("{}", tokenizer.error_at_cur("予期しない文字列です"));
            process::exit(1);
        }
    }
    println!("  ret");
    process::exit(0);
}
