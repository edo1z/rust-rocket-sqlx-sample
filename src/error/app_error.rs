use crate::repositories::error::DbRepoError;
use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::Request;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error")]
    DbError(#[from] DbRepoError),
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("{message}")]
    CustomError { status_code: u16, message: String },
}

impl AppError {
    pub fn new(status_code: u16, message: &str) -> Self {
        AppError::CustomError {
            status_code,
            message: message.to_string(),
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            AppError::DbError(_) => 500,
            AppError::BadRequest => 400,
            AppError::Unauthorized => 401,
            AppError::Forbidden => 403,
            AppError::NotFound => 404,
            AppError::InternalServerError => 500,
            AppError::CustomError { status_code, .. } => *status_code,
        }
    }
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status_code = self.status_code();
        let status = Status::from_code(status_code).unwrap_or(Status::InternalServerError);
        let error_message = self.to_string();
        Response::build_from(error_message.respond_to(req)?)
            .status(status)
            .ok()
    }
}

pub trait AppErr<T, E> {
    fn app_err(self, status_code: u16, message: &str) -> Result<T, AppError>;
}

impl<T, E> AppErr<T, E> for Result<T, E>
where
    E: core::fmt::Display,
{
    fn app_err(self, status_code: u16, message: &str) -> Result<T, AppError> {
        self.map_err(|_| AppError::new(status_code, message))
    }
}

impl<T> AppErr<T, ()> for Option<T> {
    fn app_err(self, status_code: u16, message: &str) -> Result<T, AppError> {
        self.map_or_else(|| Err(AppError::new(status_code, message)), Ok)
    }
}

// app_err!マクロ(簡単にAppErrorを作れるようにする)
#[macro_export]
macro_rules! app_err {
    ($status_code:expr, $message:expr) => {
        Err($crate::error::app_error::AppError::new(
            $status_code,
            $message,
        ))
    };
}

// app_err_bail!マクロ（return Err(app_err!(...))の略）
#[macro_export]
macro_rules! app_err_bail {
    ($status_code:expr, $message:expr) => {
        return Err($crate::error::app_error::AppError::new(
            $status_code,
            $message,
        ));
    };
}

// app_err_ensure!マクロ（if !... { return Err(app_err!(...)) }の略）
#[macro_export]
macro_rules! app_err_ensure {
    ($condition:expr, $status_code:expr, $message:expr) => {
        if !$condition {
            return Err($crate::error::app_error::AppError::new(
                $status_code,
                $message,
            ));
        }
    };
}
