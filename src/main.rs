mod utils;

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::io::{self, Write};
use crossterm::style::Color;

fn main() -> io::Result<()> {
    utils::print_utils::println_colored("欢迎使用 r-calc 计算器！", Color::Green)?;
    utils::print_utils::println_colored("请输入算式，支持 +、-、*、/、%、**（幂运算）和括号（兼容中文括号）", Color::Yellow)?;
    utils::print_utils::println_colored("输入 'q' 退出程序", Color::Yellow)?;

    loop {
        print!("> ");

        // 刷新输出缓冲区，确保所有缓冲的输出数据立即写入到控制台。
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input == "q" {
            utils::print_utils::println_colored("再见！", Color::Green)?;
            break;
        }

        // 计算表达式
        match evaluate_expression(input) {
            Ok(result) => {
                utils::print_utils::print_colored("结果: ", Color::Blue)?;
                utils::print_utils::println_colored(&result.to_string(), Color::White)?;
            }
            Err(e) => utils::print_utils::println_colored(&format!("错误: {}", e), Color::Red)?,
        }
    }

    Ok(())
}

// 计算表达式的值
fn evaluate_expression(expr: &str) -> Result<Decimal, String> {
    let tokens = tokenize(expr)?;
    let postfix = infix_to_postfix(tokens)?;
    evaluate_postfix(postfix)
}

// 将输入的算式转换为 Token 序列
fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut num_str = String::new();
    let mut chars = expr.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' | '.' => num_str.push(c),
            '+' | '-' | '*' | '/' | '%' | '(' | ')' | '（' | '）' => {
                if !num_str.is_empty() {
                    tokens.push(Token::Number(num_str.parse().map_err(|_| "无效的数字")?));
                    num_str.clear();
                }
                if c == '*' && chars.peek() == Some(&'*') {
                    chars.next(); // 消耗第二个 '*'
                    tokens.push(Token::Power);
                } else {
                    tokens.push(match c {
                        '+' => Token::Add,
                        '-' => Token::Subtract,
                        '*' => Token::Multiply,
                        '/' => Token::Divide,
                        '%' => Token::Modulo,
                        '(' | '（' => Token::LeftParen,
                        ')' | '）' => Token::RightParen,
                        _ => unreachable!(),
                    });
                }
            }
            ' ' | '\t' | '\n' | '\r' => {} // 忽略空白字符
            _ => return Err(format!("未知字符: [{}]", c)),
        }
    }

    if !num_str.is_empty() {
        tokens.push(Token::Number(num_str.parse().map_err(|_| "无效的数字")?));
    }

    Ok(tokens)
}

// 中缀表达式转后缀表达式
fn infix_to_postfix(infix: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();

    for token in infix {
        match token {
            Token::Number(_) => output.push(token),
            Token::LeftParen => operator_stack.push(token),
            Token::RightParen => {
                while let Some(op) = operator_stack.pop() {
                    if op == Token::LeftParen {
                        break;
                    }
                    output.push(op);
                }
            }
            op => {
                while let Some(top) = operator_stack.last() {
                    if *top == Token::LeftParen || op.precedence() > top.precedence() {
                        break;
                    }
                    output.push(operator_stack.pop().unwrap());
                }
                operator_stack.push(op);
            }
        }
    }

    while let Some(op) = operator_stack.pop() {
        if op == Token::LeftParen {
            return Err("不匹配的括号".to_string());
        }
        output.push(op);
    }

    Ok(output)
}

// 计算后缀表达式的值
fn evaluate_postfix(postfix: Vec<Token>) -> Result<Decimal, String> {
    let mut stack = Vec::new();

    for token in postfix {
        match token {
            Token::Number(num) => stack.push(num),
            op => {
                let b = stack.pop().ok_or("表达式无效")?;
                let a = stack.pop().ok_or("表达式无效")?;
                let result = match op {
                    Token::Add => a + b,
                    Token::Subtract => a - b,
                    Token::Multiply => a * b,
                    Token::Divide => {
                        if b == Decimal::ZERO {
                            return Err("除数不能为零".to_string());
                        }
                        a / b
                    }
                    Token::Modulo => {
                        if b == Decimal::ZERO {
                            return Err("模数不能为零".to_string());
                        }
                        a % b
                    }
                    Token::Power => {
                        let base = a.to_f64().ok_or("无法转换为浮点数")?;
                        let exponent = b.to_f64().ok_or("无法转换为浮点数")?;
                        let result = base.powf(exponent);
                        Decimal::try_from(result).map_err(|_| "幂运算结果超出范围")?
                    }
                    _ => unreachable!(),
                };
                stack.push(result);
            }
        }
    }

    stack.pop().ok_or("表达式无效".to_string())
}

// Token 类型
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(Decimal),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    LeftParen,
    RightParen,
}

impl Token {
    // 运算符优先级
    fn precedence(&self) -> u8 {
        match self {
            Token::Add | Token::Subtract => 1,
            Token::Multiply | Token::Divide | Token::Modulo => 2,
            Token::Power => 3,
            _ => 0,
        }
    }
}