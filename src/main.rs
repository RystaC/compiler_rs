use std::env;
use std::collections::LinkedList;

#[derive(Debug)]
enum Token {
    RESERVED(&'static str),
    NUMBER(u32),
    EOF(),
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { panic!("invalid the number of args (expected: 1)"); }

    let s: &String = &args[1];

    let mut token: LinkedList<Token> = tokenize(s);

    //println!("{:?}", token);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("    mov rax, {}", consume_number(&mut token));

    loop {
        match consume(&mut token) {
            Token::RESERVED("+") => println!("    add rax, {}", consume_number(&mut token)),
            Token::RESERVED("-") => println!("    sub rax, {}", consume_number(&mut token)),
            Token::EOF() => break,
            _ => panic!("invalid input is found."),
        }
    }

    println!("    ret");
    println!("");
}

fn tokenize(s: &String) -> LinkedList<Token>
{
    let mut token: LinkedList<Token> = LinkedList::new();
    let mut tmp_str = String::new();

    for x in s.as_str().chars() {
        if x == ' ' { continue; }
        match x {
            '0' ..= '9' => tmp_str.push(x),
            '+' => {
                if !tmp_str.is_empty() {
                    token.push_back(Token::NUMBER(tmp_str.parse().unwrap()));
                    tmp_str.clear();
                }
                token.push_back(Token::RESERVED("+"));
            },
            '-' => {
                if !tmp_str.is_empty() {
                    token.push_back(Token::NUMBER(tmp_str.parse().unwrap()));
                    tmp_str.clear();
                }
                token.push_back(Token::RESERVED("-"));
            },
            _ => panic!("invalid input is found."),
        }
    }

    if !tmp_str.is_empty() {
        token.push_back(Token::NUMBER(tmp_str.parse().unwrap()));
        tmp_str.clear();
    }
    token.push_back(Token::EOF());

    token
}

fn consume_number(token: &mut LinkedList<Token>) -> u32
{
    match token.pop_front() {
        Some(Token::NUMBER(x)) => x,
        _ => panic!("invalid input is found."),
    }
}

fn consume(token: &mut LinkedList<Token>) -> Token
{
    match token.pop_front() {
        Some(t) => t,
        _ => panic!("invalid input is found."),
    }
}
