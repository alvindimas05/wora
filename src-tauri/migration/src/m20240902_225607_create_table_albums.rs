use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Albums::Table)
                    .if_not_exists()
                    .col(pk_auto(Albums::Id))
                    .col(text(Albums::Name))
                    .col(text(Albums::Artist))
                    .col(integer(Albums::Year))
                    .col(blob(Albums::CoverArt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Albums::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Albums {
    Table,
    Id,
    Name,
    Artist,
    Year,
    CoverArt
}
