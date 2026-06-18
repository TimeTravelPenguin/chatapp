use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    // Generic protocol errors
    MalformedFrame,
    InvalidRequest,
    UnsupportedFrame,
    Internal,
    NotImplemented,

    // Authentication / account
    Unauthenticated,
    InvalidCredentials,
    TokenExpired,
    TokenInvalid,
    UsernameTaken,
    EmailTaken,

    // Authorisation
    Forbidden,

    // Room / message domain
    RoomNotFound,
    NotRoomMember,
    MessageNotFound,
    EditWindowExpired,

    // Reliability / limits
    DuplicateRequest,
    RateLimited,
    FrameTooLarge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientFrame {
    Login {
        request_id: Uuid,
        email: String,
        password: String,
    },

    Signup {
        request_id: Uuid,
        username: String,
        email: String,
        password: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerFrame {
    LoginOk {
        request_id: Uuid,
        user_id: Uuid,
        access_token: String,
        refresh_token: String,
    },

    SignupOk {
        request_id: Uuid,
        user_id: Uuid,
    },

    Error {
        request_id: Option<uuid::Uuid>,
        code: ErrorCode,
        message: String,
        details: Option<serde_json::Value>,
    },
}
