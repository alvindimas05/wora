use sea_orm_migration::{prelude::*, schema::*};
use crate::m20240902_225607_create_table_albums::Albums;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Song::Table)
                    .if_not_exists()
                    .col(pk_auto(Song::Id))
                    .col(text(Song::FilePath))
                    .col(text(Song::Name))
                    .col(text(Song::Artist))
                    .col(integer(Song::Duration))
                    .col(integer(Song::AlbumId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Song::Table, Song::AlbumId)
                            .to(Albums::Table, Albums::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Song::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Song {
    Table,
    Id,
    FilePath,
    Name,
    Artist,
    Duration,
    AlbumId
}
