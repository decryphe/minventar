use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum DictionaryEntries {
    Table,
    Id,
    Category,
    Value,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(DictionaryEntries::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(DictionaryEntries::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(DictionaryEntries::Category)
                        .string()
                        .not_null(),
                )
                .col(ColumnDef::new(DictionaryEntries::Value).string().not_null())
                .index(
                    Index::create()
                        .name("idx-dictionary-entries-category-value")
                        .col(DictionaryEntries::Category)
                        .col(DictionaryEntries::Value)
                        .unique(),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(DictionaryEntries::Table).to_owned())
            .await
    }
}
