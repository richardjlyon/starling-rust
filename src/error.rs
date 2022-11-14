#[derive(Debug)]
pub enum AppError {
    ConfigLoad,
    Authorisation,
    NotFound,
    NetworkError,
    ParseError,
    ReadError,
    Other,
}
