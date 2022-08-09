use crate::token::Token;

pub fn analyze(code: String) -> Vec<Token> {
    let mut is_comment = false;
    let mut block_stack: i16 = 0;
    let mut block_code: String = String::new();
    let mut tokens: Vec<Token> = vec![];

    for ch in code.chars() {
        if is_comment { continue; }

        if block_stack != 0 {
            if ch == '[' { block_stack += 1; }
            if ch == ']' { block_stack -= 1; }

            if block_stack == 0 {
                tokens.push(Token::BlockCode(analyze(block_code.clone())));
                block_code.clear();

                continue;
            }

            block_code.push(ch);
            continue;
        }

        match ch {
            '+' => tokens.push(Token::Increment),
            '-' => tokens.push(Token::Decrement),
            '<' => tokens.push(Token::MoveLeft),
            '>' => tokens.push(Token::MoveRight),
            '.' => tokens.push(Token::Print),
            ',' => tokens.push(Token::Input),
            '[' => block_stack += 1,
            ']' => block_stack -= 1,
            ';' => is_comment = true,
            '\n' => is_comment = false,
            '@' => tokens.push(Token::CreateLoop),
            '$' => tokens.push(Token::CreateFunction),
            '*' => tokens.push(Token::CallFunction),
            '!' => tokens.push(Token::Debug),
            _ => {}
        }
    }

    if block_stack != 0 { panic!("A non-closed bracket was detected."); }

    tokens
}