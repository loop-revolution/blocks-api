use crate::{BlockError, EmailConfirmError, InternalError, LoopError, UserError};
use std::time::SystemTimeError;

impl From<SystemTimeError> for LoopError {
	fn from(e: SystemTimeError) -> Self {
		sentry::capture_error(&e);
		LoopError::GenericError
	}
}

impl From<InternalError> for LoopError {
	fn from(e: InternalError) -> Self {
		LoopError::InternalError(e)
	}
}

impl From<UserError> for LoopError {
	fn from(e: UserError) -> Self {
		LoopError::UserError(e)
	}
}

impl From<BlockError> for LoopError {
	fn from(e: BlockError) -> Self {
		LoopError::BlockError(e)
	}
}

impl From<EmailConfirmError> for LoopError {
	fn from(e: EmailConfirmError) -> Self {
		UserError::EmailConfirmError(e).into()
	}
}

impl From<diesel::result::Error> for LoopError {
	fn from(e: diesel::result::Error) -> Self {
		sentry::capture_error(&e);
		LoopError::GenericError
	}
}

impl From<r2d2::Error> for LoopError {
	fn from(e: r2d2::Error) -> Self {
		sentry::capture_error(&e);
		InternalError::DatabaseTimeout.into()
	}
}

impl From<std::num::ParseIntError> for UserError {
	fn from(_: std::num::ParseIntError) -> Self {
		UserError::JwtGeneric
	}
}

impl From<serde_json::Error> for BlockError {
	fn from(_: serde_json::Error) -> Self {
		BlockError::InputParse
	}
}

impl From<jsonwebtoken::errors::Error> for UserError {
	fn from(_: jsonwebtoken::errors::Error) -> Self {
		UserError::JwtGeneric
	}
}
