//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::handler::{GroupId, Uuid};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub group_id: GroupId,
    pub display_name: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub uuid: Uuid,
}

impl From<Model> for crate::domain::handler::Group {
    fn from(group: Model) -> Self {
        Self {
            id: group.group_id,
            display_name: group.display_name,
            creation_date: group.creation_date,
            uuid: group.uuid,
            users: vec![],
        }
    }
}

impl From<Model> for crate::domain::handler::GroupDetails {
    fn from(group: Model) -> Self {
        Self {
            group_id: group.group_id,
            display_name: group.display_name,
            creation_date: group.creation_date,
            uuid: group.uuid,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::memberships::Entity")]
    Memberships,
}

impl Related<super::memberships::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Memberships.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}