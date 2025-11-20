use std::{collections::HashMap, fs::File, io};

use super::IconConverter;
use convert_case::ccase;
use reqwest::blocking::get;
use std::io::Write;

const LUCIDE_BASE_URL: &str = "https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/";

/// Lucide-specific converter
pub struct LucideConverter;

fn wrap_svg_in_templ(icon_name: &str, svg_content: &str) -> String {
    // Convert kebab-case icon name to PascalCase
    let templ_name = ccase!(kebab -> pascal, icon_name);

    // Replace the width, height, stroke-width attributes with placeholders
    let svg_content = svg_content
        .replace(r#"width="24""#, r#"width={size}"#)
        .replace(r#"height="24""#, r#"height={size}"#)
        .replace(r#"stroke-width="2""#, r#"stroke-width={thickness}"#);

    format!(
        "templ {templ_name}(size int, thickness int) {{\n{svg_content}}}\n",
        templ_name = templ_name,
        svg_content = svg_content
    )
}

impl IconConverter for LucideConverter {
    fn generate_templates(icons: &[String], output: &std::path::Path) -> io::Result<()> {
        let mut svg_map: HashMap<String, String> = HashMap::new();
        for icon in icons {
            let url = format!("{}{}.svg", LUCIDE_BASE_URL, icon);
            let response = get(&url).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            if !response.status().is_success() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Icon not found: {}", icon),
                ));
            }
            let svg_content = response
                .text()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            svg_map.insert(icon.clone(), svg_content);
        }

        // 2. Ensure output directory exists
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 3. Open output file
        let mut file = File::create(output)?;

        // 4. Generate Templ templates using fetched SVGs
        for icon in icons {
            if let Some(svg_content) = svg_map.get(icon) {
                let template = wrap_svg_in_templ(&icon, &svg_content);
                writeln!(file, "{}", template)?;
            }
        }

        Ok(())
    }
}
