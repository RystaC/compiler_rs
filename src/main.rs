use std::env;
use std::collections::LinkedList;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { panic!("invalid the number of args (expected: 1)"); }

    let s: &String = &args[1];

    let mut token: LinkedList<Token> = tokenize(s);

    //println!("{:?}", token);

    let tree = expr(&mut token);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generate(&tree);

    println!("    pop rax");
    println!("    ret\n");
}

#[derive(Debug)]
enum Token {
    RESERVED(String),
    NUMBER(u32),
    EOF,
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

                '+' | '-' | '*' | '/' | '(' | ')' =>{
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
        _ => panic!("token is not a number."),
    }
}

fn consume(token: &mut LinkedList<Token>, expect: &str) -> bool
{
    if let Some(Token::RESERVED(s)) = token.front() {
        if s.as_str() == expect {
            token.pop_front();
            return true;
        }
    }

    false
}

fn consume_expect(token: &mut LinkedList<Token>, expect: &str) {
    if let Some(Token::RESERVED(s)) = token.front() {
        if s.as_str() == expect {
            token.pop_front();
        }
        else { panic!("token is not {}.", s); }
    }
    else { panic!("token is not Token::RESERVED(_)."); }
}

#[derive(Clone)]
enum Node {
    ADD,
    SUB,
    MUL,
    DIV,
    NUM(u32),
}

#[derive(Clone)]
struct Tree {
    value: Node,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

fn expr(token: &mut LinkedList<Token>) -> Tree {
    let mut tree = mul(token);

    loop {
        if consume(token, "+") {
            tree = Tree { value: Node::ADD, left: Some(Box::new(tree.clone())), right: Some(Box::new(mul(token))) };
        }
        else if consume(token, "-") {
            tree = Tree { value: Node::SUB, left: Some(Box::new(tree.clone())), right: Some(Box::new(mul(token))) };
        }
        else { break; }
    }

    tree
}

fn mul(token: &mut LinkedList<Token>) -> Tree {
    let mut tree = primary(token);
    
    loop {
        if consume(token, "*") {
            tree = Tree { value: Node::MUL, left: Some(Box::new(tree.clone())), right: Some(Box::new(primary(token))) };
        }
        else if consume(token, "/") {
            tree = Tree { value: Node::DIV, left: Some(Box::new(tree.clone())), right: Some(Box::new(primary(token))) };
        }
        else { break; }
    }

    tree
}

fn primary(token: &mut LinkedList<Token>) -> Tree {
    if consume(token, "(") {
        let tree = expr(token);
        consume_expect(token, ")");
        tree
    }
    else {
        Tree { value: Node::NUM(consume_number(token)), left: None, right: None }
    }
}

fn generate(tree: &Tree) {
    if let Node::NUM(num) = tree.value {
        println!("    push {}", num);
    }
    else {
        if let Some(ref left) = tree.left { generate(&left); }
        if let Some(ref right) = tree.right { generate(&right); }

        println!("    pop rdi");
        println!("    pop rax");

        match tree.value {
            Node::ADD => println!("    add rax, rdi"),
            Node::SUB => println!("    sub rax, rdi"),
            Node::MUL => println!("    imul rax, rdi"),
            Node::DIV => println!("    cqo\n    idiv rdi"),
            _ => panic!("some input is not a expression."),
        }

        println!("    push rax");
    }
}
