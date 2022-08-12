use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};


pub type ErrorsType = HashMap<String, Vec<String>>;
use actix_web::{error as actix_error, http::StatusCode, HttpResponse};
use serde_json::json;

#[derive(Debug)]
pub enum CustomError {
    Authorization(String), // 認証エラー
    NotFound(String),      // 404・ドキュメントが存在しない場合
    Conversion(String),    // コンバージョンを伴う異常値の場合
    ExistAccount(String),  // アカウントが既に存在している場合
    Messages(ErrorsType),  // ユーザーに何かしらのメッセージを伝える
    Validate(ErrorsType),  // バリデーションエラー
    Database(String),      // DB周りのエラー
    Elastic(String),       // Elastic周りのエラー
    System(String),        // システムエラー
    Json(),                // Jsonパースエラー
    Other(String),         // 通知はしないが何かしらのエラー
}

impl CustomError {
    #[allow(dead_code)]
    pub fn user_errors(&self) -> Option<ErrorsType> {
        let value = self.to_owned();
        match value {
            CustomError::Authorization(e) => Some(new_error("authorization", e)),
            CustomError::Messages(errors) => Some(errors.clone()),
            CustomError::Validate(errors) => Some(errors.clone()),
            CustomError::Conversion(e) => Some(new_error("conversion", e)),
            CustomError::NotFound(e) => Some(new_error("notfound", e)),
            CustomError::ExistAccount(e) => Some(new_error("account", e)),
            CustomError::Database(_) => Some(new_error("database", "server error")),
            CustomError::Elastic(_) => Some(new_error("elastic", "server error")),
            CustomError::System(_) => Some(new_error("system", "server error")),
            CustomError::Other(_) => Some(new_error("other", "server error")),
            CustomError::Json() => Some(new_error("json", "error json format")),
            // _ => None,
        }
    }
    pub fn system_errors(&self) -> Option<ErrorsType> {
        let value = self.to_owned();
        match value {
            CustomError::Authorization(e) => Some(new_error("authorization", e)),
            CustomError::Messages(errors) => Some(errors.clone()),
            CustomError::Validate(errors) => Some(errors.clone()),
            CustomError::Conversion(e) => Some(new_error("conversion", e)),
            CustomError::NotFound(e) => Some(new_error("notfound", e)),
            CustomError::ExistAccount(e) => Some(new_error("account", e)),
            CustomError::Database(errors) => Some(new_error("database", errors.as_str())),
            CustomError::Elastic(errors) => Some(new_error("elastic", errors.as_str())),
            CustomError::System(errors) => Some(new_error("system", errors.as_str())),
            CustomError::Other(errors) => Some(new_error("other", errors.as_str())),
            CustomError::Json() => Some(new_error("json", "error json format")),
            // _ => None,
        }
    }
}
impl actix_error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::Authorization(_) => StatusCode::UNAUTHORIZED,
            CustomError::Messages(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CustomError::Validate(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CustomError::Conversion(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CustomError::NotFound(_) => StatusCode::NOT_FOUND,
            CustomError::ExistAccount(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CustomError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::Elastic(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::System(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CustomError::Other(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CustomError::Json() => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "errors":self.system_errors()
            // "errors":self.user_errors()
        }))
    }
}
impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let errors = self.system_errors();
        let mut messages = String::from("");
        for (key, value) in errors.unwrap() {
            for message in value {
                messages.push_str(key.as_str());
                messages.push_str(": ");
                messages.push_str(message.as_str());
                messages.push('\n');
            }
        }
        write!(f, "{}", messages)
    }
}

impl Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SumValue", 0)?;

        let errors = self.user_errors();

        if errors.is_some() {
            state.serialize_field("errors", &errors.unwrap())?;
        }
        state.end()
    }
}
pub fn new_error(key: &str, value: &str) -> ErrorsType {
    let mut errors = HashMap::new();
    errors.insert(key.to_string(), vec![value.to_string()]);
    errors
}
