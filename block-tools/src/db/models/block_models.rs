use super::{super::schema::blocks, NewProperty};
use crate::Error;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Block {
	pub id: i64,
	pub block_type: String,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	pub block_data: Option<String>,
	pub owner_id: i32,
}

impl Block {
	pub fn make_property(&self, name: &str, block_id: i64) -> NewProperty {
		NewProperty {
			property_name: name.to_string(),
			parent_id: self.id,
			value_id: block_id,
			annotation: None,
		}
	}

	pub fn by_id(block_id: i64, conn: &PgConnection) -> Result<Option<Self>, Error> {
		Ok(blocks::dsl::blocks
			.filter(blocks::id.eq(block_id))
			.limit(1)
			.get_result(conn)
			.optional()?)
	}

	pub fn update_data(&self, new_data: &str, conn: &PgConnection) -> Result<Block, Error> {
		Ok(
			diesel::update(blocks::dsl::blocks.filter(blocks::id.eq(self.id)))
				.set((
					blocks::block_data.eq(Some(new_data)),
					blocks::updated_at.eq(std::time::SystemTime::now()),
				))
				.get_result(conn)?,
		)
	}
}

#[derive(Insertable)]
#[table_name = "blocks"]
pub struct NewBlock {
	pub block_type: String,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
	pub block_data: Option<String>,
	pub owner_id: i32,
}

pub struct MinNewBlock<'a> {
	pub block_type: &'a str,
	pub owner_id: i32,
}

impl NewBlock {
	pub fn new(minimum: MinNewBlock) -> Self {
		NewBlock {
			block_type: minimum.block_type.to_string(),
			created_at: std::time::SystemTime::now(),
			updated_at: std::time::SystemTime::now(),
			block_data: None,
			owner_id: minimum.owner_id,
		}
	}

	pub fn data(self, data: &str) -> Self {
		NewBlock {
			block_data: Some(data.to_string()),
			..self
		}
	}

	pub fn insert(self, conn: &PgConnection) -> Result<Block, Error> {
		Ok(diesel::insert_into(blocks::table)
			.values(&self)
			.get_result(conn)?)
	}
}

impl MinNewBlock<'_> {
	pub fn into(self) -> NewBlock {
		NewBlock::new(self)
	}
	pub fn insert(self, conn: &PgConnection) -> Result<Block, Error> {
		self.into().insert(conn)
	}
}