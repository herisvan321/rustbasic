use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
    EmailVerifiedAt,
    Password,
    RememberToken,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260501_000002_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::EmailVerifiedAt).date_time().null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::RememberToken).string().null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("users_email_index")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}
