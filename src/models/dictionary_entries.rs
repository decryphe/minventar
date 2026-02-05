use std::collections::BTreeMap;

use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};

pub use super::_entities::dictionary_entries::{self, ActiveModel, Entity, Model};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DictionaryEntryGroup {
    pub category: String,
    pub entries: Vec<Model>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DictionaryEntryParams {
    pub category: String,
    pub value: String,
}

impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        params: &DictionaryEntryParams,
    ) -> ModelResult<Self> {
        params.validate()?;
        Self::ensure_unique(db, None, params).await?;

        let active_model = ActiveModel {
            category: Set(params.category.trim().to_string()),
            value: Set(params.value.trim().to_string()),
            ..Default::default()
        };

        active_model.insert(db).await.map_err(Into::into)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> ModelResult<()> {
        let item = Self::find_by_id(db, id).await?;
        item.delete(db).await.map_err(Into::into).map(|_| ())
    }

    async fn ensure_unique(
        db: &DatabaseConnection,
        current_id: Option<i32>,
        params: &DictionaryEntryParams,
    ) -> ModelResult<()> {
        let existing = Entity::find()
            .filter(dictionary_entries::Column::Category.eq(params.category.trim()))
            .filter(dictionary_entries::Column::Value.eq(params.value.trim()))
            .one(db)
            .await?;

        if existing
            .as_ref()
            .is_some_and(|entry| current_id != Some(entry.id))
        {
            return Err(ModelError::EntityAlreadyExists {});
        }

        Ok(())
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(ModelError::from)?
            .ok_or(ModelError::EntityNotFound)
    }

    pub async fn list_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        Entity::find()
            .order_by_asc(dictionary_entries::Column::Category)
            .order_by_asc(dictionary_entries::Column::Value)
            .all(db)
            .await
            .map_err(Into::into)
    }

    pub async fn list_grouped(db: &DatabaseConnection) -> ModelResult<Vec<DictionaryEntryGroup>> {
        let entries = Self::list_all(db).await?;
        let mut groups = BTreeMap::<String, Vec<Model>>::new();

        for entry in entries {
            groups
                .entry(entry.category.clone())
                .or_default()
                .push(entry);
        }

        Ok(groups
            .into_iter()
            .map(|(category, entries)| DictionaryEntryGroup { category, entries })
            .collect())
    }

    pub async fn update(
        self,
        db: &DatabaseConnection,
        params: &DictionaryEntryParams,
    ) -> ModelResult<Self> {
        params.validate()?;
        Self::ensure_unique(db, Some(self.id), params).await?;

        let mut active_model = self.into_active_model();
        active_model.category = Set(params.category.trim().to_string());
        active_model.value = Set(params.value.trim().to_string());

        active_model.update(db).await.map_err(Into::into)
    }
}

impl DictionaryEntryParams {
    fn validate(&self) -> ModelResult<()> {
        if self.category.trim().is_empty() {
            return Err(ModelError::msg("category cannot be blank"));
        }

        if self.value.trim().is_empty() {
            return Err(ModelError::msg("value cannot be blank"));
        }

        Ok(())
    }
}
