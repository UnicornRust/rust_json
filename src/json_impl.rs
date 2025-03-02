/// 包含了所有 Json 中可能出现的 token 单元 
#[derive(Debug, Clone)]
pub enum JsonToken {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Comma,
    Colon,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
}
