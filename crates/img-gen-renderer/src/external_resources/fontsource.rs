use crate::Font;
use fontsource_downloader::{FontQuery, QueryBuilder, Weight};

pub fn to_font_query(font: &Font) -> FontQuery {
    QueryBuilder::new(font.family.as_str())
        .with_style(font.style.as_str())
        .with_weight(Weight::from(&font.weight))
        .with_subset(font.subset.as_str())
        .build()
}
