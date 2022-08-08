pub enum Token {
    CommentLine,
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    BlockCode(Vec<Token>),
    Loop(Vec<Token>),
    CreateFunction,
    CallFunction,
    Debug
}