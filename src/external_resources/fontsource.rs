use crate::{Font, Weight};
use fontsource_downloader::{FontQuery, Weight as FsWeight};

impl From<Weight> for FsWeight {
    fn from(value: Weight) -> Self {
        match value {
            Weight::Thin => FsWeight::Thin,
            Weight::Light => FsWeight::Light,
            Weight::Regular => FsWeight::Normal,
            Weight::Medium => FsWeight::Medium,
            Weight::Bold => FsWeight::Bold,
            Weight::Black => FsWeight::Black,
        }
    }
}

impl From<&Font> for FontQuery {
    fn from(font: &Font) -> Self {
        FontQuery {
            family: font.family.clone(),
            style: font.style.clone(),
            weight: font.weight.into(),
            subset: font.subset.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_weight() {
        assert_eq!(FsWeight::from(Weight::Thin), FsWeight::Thin);
        assert_eq!(FsWeight::from(Weight::Light), FsWeight::Light);
        assert_eq!(FsWeight::from(Weight::Regular), FsWeight::Normal);
        assert_eq!(FsWeight::from(Weight::Medium), FsWeight::Medium);
        assert_eq!(FsWeight::from(Weight::Bold), FsWeight::Bold);
        assert_eq!(FsWeight::from(Weight::Black), FsWeight::Black);
    }
}
