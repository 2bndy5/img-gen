use crate::Font;
use fontsource_downloader::FontQuery;

pub fn to_font_query(font: &Font) -> FontQuery {
    FontQuery {
        family: font.family.clone(),
        style: font.style.clone(),
        weight: font.weight.into(),
        subset: font.subset.clone(),
    }
}
