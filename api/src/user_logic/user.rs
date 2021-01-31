use crate::{block_logic::block::BlockObject, graphql::ContextData};
use async_graphql::*;
use block_tools::{
	auth::{require_token, validate_token},
	dsl::prelude::*,
	models::{Block, User},
	schema::{blocks, users},
};
pub struct QLUser {
	/// Auto-incrementing unique ID for a user
	pub id: i32,
	/// Unique alphanumeric username for easy identification
	pub username: String,
}

#[Object]
impl QLUser {
	/// How many users there are in the database
	async fn credits(&self, context: &Context<'_>) -> Result<Option<i32>, Error> {
		let context = &context.data::<ContextData>()?;
		let conn = &context.pool.get()?;
		let token = require_token(&context.other())?;

		if self.id != validate_token(token)? {
			return Ok(None);
		}

		Ok(Some(
			users::dsl::users
				.filter(users::id.eq(&self.id))
				.select(users::credits)
				.first(conn)?,
		))
	}

	async fn id(&self) -> i32 {
		self.id
	}

	async fn username(&self) -> String {
		self.username.clone()
	}

	async fn blocks(&self, context: &Context<'_>) -> Result<Vec<BlockObject>, Error> {
		let conn = &context.data::<ContextData>()?.pool.get()?;

		let blocks: Vec<BlockObject> = blocks::dsl::blocks
			.filter(blocks::dsl::owner_id.eq(self.id))
			.load::<Block>(conn)?
			.iter()
			.map(BlockObject::from)
			.collect();

		Ok(blocks)
	}
}

impl From<User> for QLUser {
	fn from(userd: User) -> Self {
		QLUser {
			id: userd.id,
			username: userd.username,
		}
	}
}

pub async fn user_by_id(context: &ContextData, id: i32) -> Result<Option<QLUser>, Error> {
	let conn = &context.pool.get()?;

	let usr: Option<User> = users::dsl::users
		.filter(users::id.eq(id))
		.limit(1)
		.get_result(conn)
		.optional()?;

	match usr {
		None => Ok(None),
		Some(usr) => Ok(Some(QLUser::from(usr))),
	}
}

#[derive(Default)]
pub struct UserQueries;

#[Object]
impl UserQueries {
	/// How many users there are in the database
	async fn user_count(&self, context: &Context<'_>) -> FieldResult<i32> {
		let conn = &context.data::<ContextData>()?.pool.get()?;

		let num: i64 = users::dsl::users.count().get_result(conn)?;
		Ok(num as i32)
	}

	/// Tries to find a user with a matching ID. Will be null if a user is not found.
	async fn user_by_id(&self, context: &Context<'_>, id: i32) -> Result<Option<QLUser>, Error> {
		let context = &context.data::<ContextData>()?;
		user_by_id(context, id).await
	}

	/// Returns a `User` object corresponding with the authorization token.
	async fn whoami(&self, context: &Context<'_>) -> Result<Option<QLUser>, Error> {
		let context = &context.data::<ContextData>()?;
		let token = require_token(&context.other())?;
		let user_id = validate_token(token)?;

		user_by_id(context, user_id).await
	}
}