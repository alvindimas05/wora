use sea_orm_migration::{prelude::*};
use sea_orm_migration::schema::integer;
use crate::m20240902_230458_create_table_songs::Song;
use crate::m20240902_231319_create_table_playlists::Playlists;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlaylistSongs::Table)
                    .if_not_exists()
                    .col(integer(PlaylistSongs::PlaylistId))
                    .col(integer(PlaylistSongs::SongId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PlaylistSongs::Table, PlaylistSongs::PlaylistId)
                            .to(Playlists::Table, Playlists::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PlaylistSongs::Table, PlaylistSongs::SongId)
                            .to(Song::Table, Song::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlaylistSongs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PlaylistSongs {
    Table,
    PlaylistId,
    SongId
}
