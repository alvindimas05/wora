use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Playlists::Table)
                    .if_not_exists()
                    .col(pk_auto(Playlists::Id))
                    .col(text(Playlists::Name))
                    .col(text(Playlists::Description))
                    .col(blob(Playlists::CoverArt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Playlists::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Playlists {
    Table,
    Id,
    Name,
    Description,
    CoverArt
}
