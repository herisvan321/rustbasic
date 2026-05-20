use rustbasic_core::rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
pub struct EmbeddedPublic;

#[derive(RustEmbed)]
#[folder = "src/resources/views/"]
pub struct EmbeddedTemplates;

// Storage configuration paths
pub const STORAGE_TARGET: &str = "public/storage";
pub const STORAGE_SOURCE: &str = "storage/app/public";
