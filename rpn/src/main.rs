use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error");
        return;
    }

    rpn(&args[1]);
}

fn rpn(expr: &str) {
    let mut stack: Vec<i64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" | "%" => {
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }

                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let result = match token {
                    "+" => a.checked_add(b),
                    "-" => a.checked_sub(b),
                    "*" => a.checked_mul(b),
                    "/" => if b != 0 { a.checked_div(b) } else { None },
                    "%" => if b != 0 { a.checked_rem(b) } else { None },
                    _ => None,
                };

                match result {
                    Some(val) => stack.push(val),
                    None => {
                        println!("Error");
                        return;
                    }
                }
            }
            _ => {
                match token.parse::<i64>() {
                    Ok(num) => stack.push(num),
                    Err(_) => {
                        println!("Error");
                        return;
                    }
                }
            }
        }
    }

    if stack.len() == 1 {
        println!("{}", stack[0]);
    } else {
        println!("Error");
    }
}
