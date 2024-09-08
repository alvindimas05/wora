pub use sea_orm_migration::prelude::*;

mod m20240902_005535_create_settings_table;
mod m20240902_225607_create_table_albums;
mod m20240902_230458_create_table_songs;
mod m20240902_231319_create_table_playlists;
mod m20240902_231641_create_table_playlist_songs;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240902_005535_create_settings_table::Migration),
            Box::new(m20240902_225607_create_table_albums::Migration),
            Box::new(m20240902_230458_create_table_songs::Migration),
            Box::new(m20240902_231319_create_table_playlists::Migration),
            Box::new(m20240902_231641_create_table_playlist_songs::Migration),
        ]
    }
}
