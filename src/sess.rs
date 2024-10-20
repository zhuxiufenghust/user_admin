use crate::entity::sess_tb::Session;
use anyhow::{Error, Result};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub trait SessMgr {
    async fn get_session(&self, id: &str) -> Result<Session>;
    async fn add_session(&self, sess: &Session) -> Result<()>;
}
