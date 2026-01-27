#[derive(Debug)]
pub enum NetworkError {
    Connection,
    Timeout,
    Write,
    Read,
    WrongRPC,
    Send,
}
