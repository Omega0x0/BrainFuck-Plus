pub enum Token {
    CommentLine,            // ';'
    Increment,              // '+'
    Decrement,              // '-'
    MoveLeft,               // '<'
    MoveRight,              // '>'
    Print,                  // '.'
    Input,                  // ','
    BlockCode(Vec<Token>),  // {...}
    Loop(Vec<Token>),       // [...]
    CreateFunction,         // '$'
    CallFunction,           // '*'
    Debug                   // '!'
}