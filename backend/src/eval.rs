
#[derive(Clone, Debug, PartialEq)]
pub enum ByteCode {
    App(u8), // arg < 64
    Push(u8), // arg < 64
    Pop(u8), // arg < 64
    Add(u8),
    Call,
    Ret,
}