use rustbasic_core::{Schema, SchemaManager, MigrationTrait, DbErr};
use rustbasic_core::async_trait;

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    fn name(&self) -> &str {
        "m20260501_000002_create_users_table"
    }

    async fn up<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        Schema::create(manager, "users", |table| {
            table.id();
            table.string("name").not_null();
            table.string("email").not_null().unique();
            table.string("email_verified_at").nullable();
            table.string("password").not_null();
            table.string("remember_token").nullable();
        }).await
    }

    async fn down<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        Schema::drop(manager, "users").await
    }
}
