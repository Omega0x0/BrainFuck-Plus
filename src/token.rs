#[derive(Clone)]
pub enum Token {
    Increment,              // '+'
    Decrement,              // '-'
    MoveLeft,               // '<'
    MoveRight,              // '>'
    Print,                  // '.'
    Input,                  // ','
    BlockCode(Vec<Token>),  // [...]
    CreateLoop,             // '@'
    CreateFunction,         // '$'
    CallFunction,           // '*'
    Debug                   // '!'
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let mut res = String::new();

        match self {
            Token::Increment => res.push_str(":Increment:"),
            Token::Decrement => res.push_str(":Decrement:"),
            Token::MoveLeft => res.push_str(":MoveLeft:"),
            Token::MoveRight => res.push_str(":MoveRight:"),
            Token::Print => res.push_str(":Print:"),
            Token::Input => res.push_str(":Input:"),
            Token::BlockCode(tokens) => {
                for token in tokens.iter() {
                    res.push_str(token.to_string().as_str())
                }
            },
            Token::CreateLoop => res.push_str(":CreateLoop:"),
            Token::CreateFunction => res.push_str(":CreateFunction:"),
            Token::CallFunction => res.push_str(":CallFunction:"),
            Token::Debug => res.push_str(":Debug:")
        }

        res
    }
}