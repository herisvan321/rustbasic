use rustbasic_core::sea_orm_migration::prelude::*;
use rustbasic_core::async_trait;

#[derive(Iden)]
enum Sessions {
    Table,
    Id,
    UserId,
    IpAddress,
    UserAgent,
    Payload,
    LastActivity,
}

#[derive(Iden)]
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260501_000001_create_sessions_table"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Sessions::Id).string().primary_key())
                    .col(ColumnDef::new(Sessions::UserId).string().null())
                    .col(ColumnDef::new(Sessions::IpAddress).string().null())
                    .col(ColumnDef::new(Sessions::UserAgent).text().null())
                    .col(ColumnDef::new(Sessions::Payload).text().not_null())
                    .col(ColumnDef::new(Sessions::LastActivity).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("sessions_last_activity_index")
                    .table(Sessions::Table)
                    .col(Sessions::LastActivity)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}
