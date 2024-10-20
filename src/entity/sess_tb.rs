use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sess_tb")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub token: String,
    pub session_info: String,
    pub create_time: u64,
    pub update_time: u64,
    pub expire_time: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Session = Model;
