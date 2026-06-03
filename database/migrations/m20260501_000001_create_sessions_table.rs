use rustbasic_core::{Schema, SchemaManager, MigrationTrait, DbErr};
use rustbasic_core::async_trait;

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    fn name(&self) -> &str {
        "m20260501_000001_create_sessions_table"
    }

    async fn up<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        Schema::create(manager, "sessions", |table| {
            table.no_id();
            table.string("id").not_null().primary_key();
            table.string("user_id").nullable();
            table.string("ip_address").nullable();
            table.text("user_agent").nullable();
            table.long_text("payload").not_null();
            table.integer("last_activity").not_null();
        }).await
    }

    async fn down<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        Schema::drop(manager, "sessions").await
    }
}
