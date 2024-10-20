use crate::entity::prelude::*;
use anyhow::{Error, Result};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub trait UserMgr {
    async fn get_user(&self, user_id: u64) -> Result<User>;
    async fn add_user(&self, user: &User) -> Result<()>;
    async fn get_user_with_name(&self, name: &str) -> Result<User>;
}
