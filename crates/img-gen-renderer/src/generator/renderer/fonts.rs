use std::collections::HashSet;

use fontsource_downloader::{FontQuery, QueryBuilder, Weight};
use resvg::usvg::roxmltree;

pub(super) fn extract_fonts_from_svg(
    svg_data: &str,
) -> Result<(roxmltree::Document<'_>, HashSet<String>), roxmltree::Error> {
    let doc = roxmltree::Document::parse(svg_data)?;
    let mut families = HashSet::new();

    for node in doc.descendants().filter(|node| node.is_element()) {
        if let Some(value) = node.attribute("font-family") {
            add_font_families_from_value(value, &mut families);
        }
        if let Some(style) = node.attribute("style") {
            add_font_families_from_inline_style(style, &mut families);
        }
        if node.tag_name().name() == "style"
            && let Some(css) = node.text()
        {
            add_font_families_from_inline_style(css, &mut families);
        }
    }

    Ok((doc, families))
}

fn add_font_families_from_inline_style(style: &str, families: &mut HashSet<String>) {
    let lower = style.to_ascii_lowercase();
    let lower_bytes = lower.as_bytes();
    let style_bytes = style.as_bytes();
    let mut search_start = 0usize;

    while let Some(found) = lower[search_start..].find("font-family") {
        let mut cursor = search_start + found + "font-family".len();

        while let Some(byte) = lower_bytes.get(cursor) {
            if !byte.is_ascii_whitespace() {
                break;
            }
            cursor += 1;
        }

        if !matches!(lower_bytes.get(cursor), Some(&b':')) {
            search_start = cursor.saturating_add(1);
            continue;
        }
        cursor += 1;

        let value_start = cursor;
        while let Some(byte) = style_bytes.get(cursor) {
            if matches!(*byte, b';' | b'}') {
                break;
            }
            cursor += 1;
        }

        let value = style.get(value_start..cursor).unwrap_or_default();
        add_font_families_from_value(value, families);
        search_start = cursor.saturating_add(1);
    }
}

fn add_font_families_from_value(value: &str, families: &mut HashSet<String>) {
    for family in value
        .split(',')
        .map(|family| family.trim().trim_matches(['"', '\'']).trim())
    {
        if !family.is_empty() && !is_generic_font_family(family) {
            families.insert(family.to_string());
        }
    }
}

fn is_generic_font_family(family: &str) -> bool {
    matches!(
        family.trim().to_ascii_lowercase().as_str(),
        "serif"
            | "sans-serif"
            | "monospace"
            | "cursive"
            | "fantasy"
            | "system-ui"
            | "ui-serif"
            | "ui-sans-serif"
            | "ui-monospace"
            | "ui-rounded"
            | "emoji"
            | "math"
            | "fangsong" // spell-checker: disable-line
    )
}

pub(super) fn to_font_query(font: &crate::Font) -> FontQuery {
    QueryBuilder::new(&font.family)
        .with_style(&font.style)
        .with_weight(Weight::from(&font.weight))
        .with_subset(&font.subset)
        .build()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn svg_fonts_from_attrs_inline_style() {
        let svg = r#"
        <svg xmlns='http://www.w3.org/2000/svg'>
            <text font-family='Inter, serif'>hello</text>
            <text style='fill:#333; font-family: "Playfair Display", sans-serif;'>world</text>
        </svg>
        "#;
        let (_, families) = extract_fonts_from_svg(svg).unwrap();
        assert!(families.contains("Inter"));
        assert!(families.contains("Playfair Display"));
        assert!(!families.contains("serif"));
        assert!(!families.contains("sans-serif"));
    }

    #[test]
    fn svg_fonts_from_style_nodes() {
        let svg = r#"
        <svg xmlns='http://www.w3.org/2000/svg'>
            <style>
                .title { font-family: "Fira Sans", cursive; }
            </style>
            <text class='title'>hello</text>
        </svg>
        "#;
        let (_, families) = extract_fonts_from_svg(svg).unwrap();
        assert!(families.contains("Fira Sans"));
        assert!(!families.contains("cursive"));
    }
}
