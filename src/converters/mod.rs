pub mod lucide;
pub use lucide::LucideConverter;

pub trait IconConverter {
    fn generate_templates(icons: &[String], output: &std::path::Path) -> std::io::Result<()>;
}
