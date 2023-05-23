use std::env;

use crate::token::TokenKind;

pub mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("{}: invail number of arguments.", args[0]);
    }

    let mut tokens = token::tokenize(&args[1]);

    println!("  .global main");
    println!("main:");

    // 处理第一个操作数
    println!("  li a0, {}", token::consume_one_num(&mut tokens));

    // 处理后续token
    while !tokens.is_empty() {
        let tok = tokens.pop_front().unwrap();

        if let TokenKind::PUNCT { op } = tok.token_kind {
            match op {
                '+' => {
                    println!("  add a0, a0, {}", token::consume_one_num(&mut tokens));
                }
                '-' => {
                    println!("  add a0, a0, -{}", token::consume_one_num(&mut tokens));
                }
                _ => {
                    panic!("unknown operator {}.", op);
                }
            }
        } else {
            panic!("expected a operator.")
        }
    }

    println!("  ret");
}
