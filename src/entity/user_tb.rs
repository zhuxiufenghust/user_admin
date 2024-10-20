//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_tb")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub passwd: String,
    pub create_time: u64,
    pub update_time: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
pub type User = Model;
