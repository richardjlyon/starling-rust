#[derive(Debug)]
pub enum AppError {
    ConfigLoad,
    Authorisation,
    Other,
    NetworkError,
    ReadError,
}

