use std::env;
use std::collections::LinkedList;

#[derive(Debug)]
enum Token {
    RESERVED(String),
    NUMBER(u32),
    EOF,
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
        if let Some(s) = consume(&mut token) { match s.as_str() {
                "+" => println!("    add rax, {}", consume_number(&mut token)),
                "-" => println!("    sub rax, {}", consume_number(&mut token)),
                _ => panic!("invalid input is found."),
            }
        }
        else { break; } 
    }

    println!("    ret");
    println!("");
}

fn tokenize(s: &String) -> LinkedList<Token>
{
    let mut token: LinkedList<Token> = LinkedList::new();
    let mut tmp_str = String::new();

    let mut str_iter = s.chars().peekable();

    loop {
        if let Some(next) = str_iter.next() {
            match next {
                ' ' => continue,

                '0' ..= '9' =>{
                    tmp_str.push(next);
                    match str_iter.peek() {
                        Some('0' ..= '9') => {},
                        _ => {
                            token.push_back(Token::NUMBER(tmp_str.parse().unwrap()));
                            tmp_str.clear();
                        }
                    }
                },

                '+' | '-' =>{
                    tmp_str.push(next);
                    token.push_back(Token::RESERVED(tmp_str.clone()));
                    tmp_str.clear();
                },
                _ => panic!("invalid input is found."),
            }
        }
        else { break; }
        
    }

    token.push_back(Token::EOF);

    token
}

fn consume_number(token: &mut LinkedList<Token>) -> u32
{
    match token.pop_front() {
        Some(Token::NUMBER(x)) => x,
        _ => panic!("invalid input is found."),
    }
}

fn consume(token: &mut LinkedList<Token>) -> Option<String>
{
    match token.pop_front() {
        Some(Token::RESERVED(s)) => Some(s),
        Some(Token::EOF) => None,
        _ => panic!("invalid input is found."),
    }
}
