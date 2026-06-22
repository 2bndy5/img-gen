use super::{ConcreteSize, Renderer, fonts::to_font_query};
use crate::{
    Border, ColorKind, Font, ImgGenRendererError, Layer, LayerOffset, Line, Result, TypographyAlign,
};
use image::{RgbaImage, imageops::overlay};
use parley::{
    Alignment, AlignmentOptions, GenericFamily, GlyphRun, Layout, LineHeight, OverflowWrap,
    PositionedLayoutItem, StyleProperty, TextWrapMode, fontique::Blob,
};
use resvg::tiny_skia::Color;
use std::{collections::VecDeque, fs, path::Path, sync::Arc};
use swash::{
    scale::{Render, ScaleContext, Source, StrikeWith},
    zeno::{Format, Vector},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct TextBrush {
    pub color: Color,
}

impl Default for TextBrush {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
        }
    }
}

const WBR: char = '\u{200B}';

#[derive(Clone, Copy)]
pub(super) struct TextMeasureParams<'a> {
    pub(super) max_width: Option<f32>,
    pub(super) font: Option<&'a Font>,
    pub(super) line: Option<&'a Line>,
    pub(super) alignment: Alignment,
    pub(super) wrap_mode: TextWrapMode,
    pub(super) border_width: u32,
}

#[derive(Clone, Copy)]
pub(super) struct RenderTextParams<'a> {
    pub(super) color: &'a ColorKind,
    pub(super) font_size: f32,
    pub(super) max_width: f32,
    pub(super) max_height: u32,
    pub(super) layer_offset: &'a LayerOffset,
    pub(super) font: &'a Font,
    pub(super) line: &'a Line,
    pub(super) alignment: Alignment,
    pub(super) wrap_mode: TextWrapMode,
    pub(super) border: Option<&'a Border>,
}

#[derive(Clone, Copy)]
struct OverflowLayoutParams<'a> {
    text: &'a str,
    max_width: f32,
    max_lines: usize,
    font: &'a Font,
    line: &'a Line,
    alignment: Alignment,
    border_width: u32,
}

impl<'a> OverflowLayoutParams<'a> {
    fn as_measure_params(&self) -> TextMeasureParams<'a> {
        TextMeasureParams {
            max_width: Some(self.max_width),
            font: Some(self.font),
            line: Some(self.line),
            alignment: self.alignment,
            wrap_mode: TextWrapMode::Wrap,
            border_width: self.border_width,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct TextMetrics {
    width: u32,
    height: u32,
    lines: usize,
}

#[derive(Clone, Copy)]
struct TextLayoutParams<'a> {
    max_width: Option<f32>,
    font: Option<&'a Font>,
    line: Option<&'a Line>,
    alignment: Alignment,
    wrap_mode: TextWrapMode,
    overflow_wrap: OverflowWrap,
    brush: TextBrush,
    font_size: f32,
}

impl Renderer<'_> {
    pub async fn render_typography(
        &mut self,
        layer: &Layer,
        size: ConcreteSize,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        if let Some(l) = layer.typography.as_ref() {
            let border_width = l.border.as_ref().map(|b| b.width.get()).unwrap_or_default();
            let max_width = size.width;
            let max_height = size.height;
            let layout_max_width = max_width.saturating_sub(border_width).max(1);
            let alignment = Self::horizontal_alignment(&l.align);
            let text = Self::inject_word_breaks(&l.content);
            let params = OverflowLayoutParams {
                text: &text,
                max_width: layout_max_width as f32,
                max_lines: l.line.amount.get() as usize,
                font: &l.font,
                line: &l.line,
                alignment,
                border_width,
            };
            let initial_font_size = l.line.get_font_size(max_height, Some(border_width))? as f32;
            let line_box_font_size = self
                .fit_font_size_to_line_box(
                    &l.font,
                    &l.line,
                    max_height,
                    border_width,
                    initial_font_size,
                )
                .await?;

            let measure = params.as_measure_params();

            let (render_text, render_font_size) = if l.overflow {
                (
                    text.clone(),
                    self.fit_font_size_to_text_bounds(params, line_box_font_size)
                        .await?,
                )
            } else {
                let metrics = self
                    .measure_text_metrics(&text, line_box_font_size, measure)
                    .await?;
                if metrics.lines <= l.line.amount.get() as usize {
                    (text.clone(), line_box_font_size)
                } else {
                    (
                        self.truncate_with_ellipsis(params, line_box_font_size)
                            .await?,
                        line_box_font_size,
                    )
                }
            };

            let text_metrics = self
                .measure_text_metrics(&render_text, render_font_size, measure)
                .await?;
            // When multiple lines will be distributed to fill the layer height,
            // treat the effective text height as max_height so vertical alignment
            // offsets are computed correctly (text already spans the full height).
            let effective_text_height =
                if text_metrics.lines > 1 && text_metrics.height < max_height {
                    max_height
                } else {
                    text_metrics.height
                };
            let vertical_offset =
                Self::vertical_alignment_offset(&l.align, max_height, effective_text_height);
            let text_offset = LayerOffset {
                x: layer.offset.x,
                y: layer.offset.y + vertical_offset as i32,
            };
            let render = RenderTextParams {
                color: &l.color,
                font_size: render_font_size,
                max_width: layout_max_width as f32,
                max_height,
                layer_offset: &text_offset,
                font: &l.font,
                line: &l.line,
                alignment,
                wrap_mode: TextWrapMode::Wrap,
                border: l.border.as_ref(),
            };
            self.render_text(canvas, &render_text, render).await?;
        }
        Ok(())
    }

    pub(super) async fn render_text(
        &mut self,
        canvas: &mut RgbaImage,
        text: &str,
        params: RenderTextParams<'_>,
    ) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }

        self.ensure_font_available(params.font).await?;

        let layout_params = TextLayoutParams {
            max_width: Some(params.max_width),
            font: Some(params.font),
            line: Some(params.line),
            alignment: params.alignment,
            wrap_mode: params.wrap_mode,
            overflow_wrap: OverflowWrap::Anywhere,
            brush: TextBrush::default(),
            font_size: params.font_size,
        };
        let layout = self.build_text_layout(text, layout_params);
        let border_width = params.border.map_or(0, |b| b.width.get());
        // Use max_width as the image width when available: after alignment (center/end),
        // glyph run offsets are already relative to the full max_advance container, so
        // layout.width() (the natural content width) would be too narrow to hold them.
        let width = params.max_width.ceil().max(1.0) as u32 + border_width;
        // Use block_max_coord of the last line so descenders are not clipped.
        let num_lines = layout.lines().len();
        let true_height = Self::layout_true_height(&layout);
        // Distribute any remaining vertical space evenly between lines so that
        // multi-line text fills the full layer height (like CSS justify for vertical).
        let gap_per_line = if params.line.height.get() > 1.0 && num_lines > 1 {
            let available = params.max_height.saturating_sub(border_width) as f32;
            (available - true_height) / (num_lines - 1) as f32
        } else {
            0.0
        };
        let height = if gap_per_line > 0.0 {
            params.max_height + border_width
        } else {
            true_height.ceil().max(1.0) as u32 + border_width
        };
        let mut text_img = RgbaImage::new(width, height);
        let mut fill_mask = vec![0u8; (width * height) as usize];

        for (line_idx, line) in layout.lines().enumerate() {
            let y_offset = (line_idx as f32 * gap_per_line).round() as i32;
            for item in line.items() {
                if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                    Self::render_glyph_run(
                        &glyph_run,
                        &mut self.scale_cx,
                        &mut text_img,
                        &mut fill_mask,
                        border_width / 2,
                        y_offset,
                    )?;
                }
            }
        }

        // Compose the fill into `text_img` by sampling the color source per-pixel.
        // Reuse the renderer colorize helper which handles optional masks.
        Self::colorize_masked(
            params.color,
            &mut text_img,
            Some(fill_mask.as_slice()),
            false,
        );

        if let Some(border) = params.border {
            let outer_radius = border.width.get().div_ceil(2);
            let inner_radius = border.width.get() / 2;
            let dilated = Self::dilate_alpha_mask(&fill_mask, width, height, outer_radius);
            let eroded = if inner_radius == 0 {
                None
            } else {
                Some(Self::erode_alpha_mask(
                    &fill_mask,
                    width,
                    height,
                    inner_radius,
                ))
            };

            let mut border_img = RgbaImage::new(width, height);
            // Build a ring mask then use the same colorize helper to paint it.
            let mut ring_mask = vec![0u8; (width * height) as usize];
            for (idx, ring_alpha) in ring_mask.iter_mut().enumerate() {
                let base = eroded.as_ref().map_or(fill_mask[idx], |mask| mask[idx]);
                *ring_alpha = dilated[idx].saturating_sub(base);
            }
            super::Renderer::colorize_masked(
                &border.color,
                &mut border_img,
                Some(ring_mask.as_slice()),
                false,
            );
            overlay(
                canvas,
                &border_img,
                params.layer_offset.x.into(),
                params.layer_offset.y.into(),
            );
        }

        overlay(
            canvas,
            &text_img,
            params.layer_offset.x.into(),
            params.layer_offset.y.into(),
        );
        Ok(())
    }

    pub(super) async fn measure_text(
        &mut self,
        text: &str,
        font_size: f32,
        params: TextMeasureParams<'_>,
    ) -> Result<ConcreteSize> {
        let metrics = self.measure_text_metrics(text, font_size, params).await?;
        Ok(ConcreteSize {
            width: metrics.width,
            height: metrics.height,
        })
    }

    async fn measure_text_metrics(
        &mut self,
        text: &str,
        font_size: f32,
        params: TextMeasureParams<'_>,
    ) -> Result<TextMetrics> {
        if text.is_empty() {
            return Ok(TextMetrics::default());
        }

        if let Some(font) = params.font {
            self.ensure_font_available(font).await?;
        }
        let layout_params = TextLayoutParams {
            max_width: params.max_width,
            font: params.font,
            line: params.line,
            alignment: params.alignment,
            wrap_mode: params.wrap_mode,
            overflow_wrap: OverflowWrap::Normal,
            brush: TextBrush::default(),
            font_size,
        };
        let layout = self.build_text_layout(text, layout_params);

        // Use block_max_coord of the last line as the true height so that
        // descenders of the final line (e.g. "g", "p", "y") are included.
        // layout.height() only sums line_height values and can under-count when
        // the natural ascent+descent exceeds the configured line height.
        let true_height = Self::layout_true_height(&layout);

        Ok(TextMetrics {
            width: layout.width().ceil().max(1.0) as u32 + params.border_width,
            height: true_height.ceil().max(1.0) as u32 + params.border_width,
            lines: layout.lines().len(),
        })
    }

    fn build_text_layout(&mut self, text: &str, params: TextLayoutParams<'_>) -> Layout<TextBrush> {
        let mut builder = self
            .layout_cx
            .ranged_builder(&mut self.font_cx, text, 1.0, true);
        builder.push_default(StyleProperty::Brush(params.brush));
        if let Some(font) = params.font {
            builder.push_default(StyleProperty::FontFamily(font.font_family()));
            builder.push_default(StyleProperty::FontStyle(font.font_style()));
            builder.push_default(StyleProperty::FontWeight(font.font_weight()));
        } else {
            builder.push_default(GenericFamily::SystemUi);
        }
        builder.push_default(StyleProperty::FontSize(params.font_size.max(1.0)));
        if let Some(line) = params.line {
            let lh = line.height.get();
            let amt = line.amount.get().max(1) as f32;
            // Convert the spec's per-line height ratio to the parley baseline-to-baseline
            // multiplier.  Derivation mirrors fit_font_size_to_line_box:
            //   target_height = theoretical * [1 + (lh-1) * (amt-1)/amt]
            //   parley_lh     = theoretical / target_height = 1 / that factor
            // lh < 1 → gap between lines; lh = 1 → lines touch; lh > 1 → lines overlap.
            let denom = (1.0 + (lh - 1.0) * (amt - 1.0) / amt).max(f32::EPSILON);
            let parley_lh = (1.0 / denom).max(0.1);
            builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(
                parley_lh,
            )));
        }
        builder.push_default(StyleProperty::TextWrapMode(params.wrap_mode));
        builder.push_default(StyleProperty::OverflowWrap(params.overflow_wrap));

        let mut layout: Layout<TextBrush> = builder.build(text);
        layout.break_all_lines(params.max_width);
        layout.align(params.alignment, AlignmentOptions::default());
        layout
    }

    async fn ensure_font_available(&mut self, font: &Font) -> Result<()> {
        if let Some(path) = font.path_buf()
            && let Some(resolved) = self.find_ext_resource_path(&path)
        {
            self.register_font_path(&resolved)?;
            return Ok(());
        }

        if self
            .font_cx
            .collection
            .family_by_name(&font.family)
            .is_some()
        {
            return Ok(());
        }
        let query = to_font_query(font);
        let downloaded_paths = self.fontsource_client.download_font(&query).await?;
        for p in &downloaded_paths {
            self.register_font_path(p)?;
        }
        Ok(())
    }

    pub(super) fn register_font_path(&mut self, path: &Path) -> Result<()> {
        let canonical_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        if self.loaded_font_paths.contains(&canonical_path) {
            return Ok(());
        }

        let font_data = fs::read(&canonical_path).map_err(|source| {
            ImgGenRendererError::ReadFontFileFailed {
                path: canonical_path.display().to_string(),
                source,
            }
        })?;
        self.font_cx
            .collection
            .register_fonts(Blob::from(font_data.clone()), None);
        Arc::make_mut(&mut self.svg_options.fontdb).load_font_data(font_data);
        self.loaded_font_paths.insert(canonical_path);
        Ok(())
    }

    /// Inject zero-width spaces (`\u{200B}`) at probable word-break points so the
    /// layout engine can wrap long API-style identifiers (e.g. `camelCase`,
    /// `snake_case`, `module.Class`) more naturally.
    fn inject_word_breaks(text: &str) -> String {
        let mut result = String::with_capacity(text.len() + 16);
        let mut iter = text.chars().peekable();
        while let Some(c) = iter.next() {
            result.push(c);
            let next = iter.peek().copied();
            // After a run of punctuation (. : _ -), insert a break opportunity.
            if matches!(c, '.' | ':' | '_' | '-') && !matches!(next, Some('.' | ':' | '_' | '-')) {
                result.push(WBR);
            }
            // Before opening brackets or slashes, insert a break opportunity.
            if matches!(next, Some('(' | '[' | '{' | '/')) {
                result.push(WBR);
            }
            // Between camelCase transitions: lowercase followed by uppercase.
            if c.is_ascii_lowercase() && matches!(next, Some(n) if n.is_ascii_uppercase()) {
                result.push(WBR);
            }
        }
        result
    }

    /// Fit the font size to the layer's line box using a representative glyph.
    /// This mirrors the Qt-style approach: start near the theoretical size,
    /// then adjust in larger and smaller steps until the measured height fits.
    async fn fit_font_size_to_line_box(
        &mut self,
        font: &Font,
        line: &Line,
        max_height: u32,
        border_width: u32,
        initial: f32,
    ) -> Result<f32> {
        self.ensure_font_available(font).await?;

        let available_height = max_height.saturating_sub(border_width) as f32;
        let line_amount = line.amount.get() as f32;
        let theoretical_height = available_height / line_amount;
        let lh = line.height.get();
        let factor = 1.0 + (lh - 1.0) * (line_amount - 1.0) / line_amount;
        let target_height = (theoretical_height * factor).max(1.0);

        let measure = TextMeasureParams {
            max_width: None,
            font: Some(font),
            line: Some(line),
            alignment: Alignment::Start,
            wrap_mode: TextWrapMode::NoWrap,
            border_width,
        };

        let mut font_size = initial.max(2.0).floor();
        font_size -= font_size % 2.0;
        font_size = font_size.max(2.0);

        let mut metrics = self.measure_text_metrics("Q", font_size, measure).await?;
        while metrics.height as f32 > target_height && font_size > 2.0 {
            font_size -= 2.0;
            metrics = self.measure_text_metrics("Q", font_size, measure).await?;
        }

        while (metrics.height as f32) < target_height - 1.0 {
            font_size += 1.0;
            metrics = self.measure_text_metrics("Q", font_size, measure).await?;
        }

        Ok(font_size.max(1.0))
    }

    /// Binary-search the largest font size ≤ `initial` for the actual content
    /// while only checking width and line count. Height is already handled by
    /// `fit_font_size_to_line_box`.
    async fn fit_font_size_to_text_bounds(
        &mut self,
        params: OverflowLayoutParams<'_>,
        initial: f32,
    ) -> Result<f32> {
        let measure = params.as_measure_params();
        let metrics = self
            .measure_text_metrics(params.text, initial, measure)
            .await?;
        if metrics.width <= params.max_width.ceil() as u32 && metrics.lines <= params.max_lines {
            return Ok(initial);
        }
        let mut lo = 1.0f32;
        let mut hi = initial;
        while hi - lo > 0.5 {
            let mid = (lo + hi) / 2.0;
            let metrics = self.measure_text_metrics(params.text, mid, measure).await?;
            if metrics.width <= params.max_width.ceil() as u32 && metrics.lines <= params.max_lines
            {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Ok(lo.max(1.0))
    }

    /// Return the largest prefix of `text` (plus a trailing `…`) that still
    /// fits within `max_height` when wrapped to `max_width`, or the original
    /// text if it already fits.
    async fn truncate_with_ellipsis(
        &mut self,
        params: OverflowLayoutParams<'_>,
        font_size: f32,
    ) -> Result<String> {
        let measure = params.as_measure_params();
        let metrics = self
            .measure_text_metrics(params.text, font_size, measure)
            .await?;
        if metrics.lines <= params.max_lines {
            return Ok(params.text.to_string());
        }
        const ELLIPSIS: &str = "\u{2026}";
        let chars: Vec<char> = params.text.chars().collect();
        let mut lo = 0usize;
        let mut hi = chars.len();
        while lo < hi {
            let mid = (lo + hi).div_ceil(2);
            let candidate: String = chars[..mid].iter().collect::<String>() + ELLIPSIS;
            let metrics = self
                .measure_text_metrics(&candidate, font_size, measure)
                .await?;
            if metrics.lines <= params.max_lines {
                lo = mid;
            } else {
                if mid == 0 {
                    break;
                }
                hi = mid - 1;
            }
        }
        Ok(chars[..lo].iter().collect::<String>() + ELLIPSIS)
    }

    fn layout_true_height(layout: &Layout<TextBrush>) -> f32 {
        layout.lines().last().map_or(layout.height(), |l| {
            l.metrics().block_max_coord.max(layout.height())
        })
    }

    fn horizontal_alignment(alignment: &TypographyAlign) -> Alignment {
        match alignment {
            TypographyAlign::StartTop
            | TypographyAlign::StartCenter
            | TypographyAlign::StartBottom => Alignment::Start,
            TypographyAlign::CenterTop
            | TypographyAlign::Center
            | TypographyAlign::CenterCenter
            | TypographyAlign::CenterBottom => Alignment::Center,
            TypographyAlign::EndTop | TypographyAlign::EndCenter | TypographyAlign::EndBottom => {
                Alignment::End
            }
        }
    }

    fn vertical_alignment_offset(
        alignment: &TypographyAlign,
        bounds_height: u32,
        text_height: u32,
    ) -> i64 {
        let spare_height = bounds_height.saturating_sub(text_height) as i64;
        match alignment {
            TypographyAlign::StartTop | TypographyAlign::CenterTop | TypographyAlign::EndTop => 0,
            TypographyAlign::StartCenter
            | TypographyAlign::Center
            | TypographyAlign::CenterCenter
            | TypographyAlign::EndCenter => spare_height / 2,
            TypographyAlign::StartBottom
            | TypographyAlign::CenterBottom
            | TypographyAlign::EndBottom => spare_height,
        }
    }

    fn render_glyph_run(
        glyph_run: &GlyphRun<'_, TextBrush>,
        scale_cx: &mut ScaleContext,
        img: &mut RgbaImage,
        fill_mask: &mut [u8],
        stroke_padding: u32,
        y_offset: i32,
    ) -> Result<()> {
        let mut run_x = glyph_run.offset();
        let run_y = glyph_run.baseline();
        let run = glyph_run.run();
        let font = run.font();
        let font_size = run.font_size();
        let normalized_coords = run.normalized_coords();

        let font_ref = swash::FontRef::from_index(font.data.as_ref(), font.index as usize)
            .ok_or(ImgGenRendererError::InvalidGlyphRunFontReference)?;

        let mut scaler = scale_cx
            .builder(font_ref)
            .size(font_size)
            .hint(true)
            .normalized_coords(normalized_coords)
            .build();
        let img_w = img.width();
        let img_h = img.height();

        for glyph in glyph_run.glyphs() {
            let glyph_x = run_x + glyph.x;
            let glyph_y = run_y + glyph.y;
            run_x += glyph.advance;

            let offset = Vector::new(glyph_x.fract(), glyph_y.fract());
            let Some(rendered) = Render::new(&[
                Source::ColorOutline(0),
                Source::ColorBitmap(StrikeWith::BestFit),
                Source::Outline,
            ])
            .format(Format::Alpha)
            .offset(offset)
            .render(&mut scaler, glyph.id as u16) else {
                continue;
            };

            let gx = (stroke_padding as i32 + glyph_x.floor() as i32 + rendered.placement.left)
                .max(0) as u32;
            let gy = (stroke_padding as i32 + y_offset + glyph_y.floor() as i32
                - rendered.placement.top)
                .max(0) as u32;
            let glyph_w = rendered.placement.width;
            let glyph_h = rendered.placement.height;

            if gx >= img_w || gy >= img_h {
                continue;
            }

            let copy_w = (glyph_w).min(img_w - gx);
            let copy_h = (glyph_h).min(img_h - gy);
            if copy_w == 0 || copy_h == 0 {
                continue;
            }

            for py in 0..copy_h {
                let row_start = (py * glyph_w) as usize;
                for px in 0..copy_w {
                    let x = gx + px;
                    let y = gy + py;
                    let alpha_raw = rendered.data[row_start + px as usize];
                    let idx = (y * img_w + x) as usize;
                    fill_mask[idx] = fill_mask[idx].max(alpha_raw);
                }
            }
        }
        Ok(())
    }

    /// Separable O(W·H) sliding-window max-filter (`is_max=true`, dilation) or
    /// min-filter (`is_max=false`, erosion). Two passes: horizontal then vertical.
    /// Erosion zeros pixels within `radius` of any image edge.
    fn sliding_window_filter(
        src: &[u8],
        width: u32,
        height: u32,
        radius: u32,
        is_max: bool,
    ) -> Vec<u8> {
        if radius == 0 {
            return src.to_vec();
        }
        let (w, h) = (width as usize, height as usize);
        let r = radius as usize;
        let len = w * h;

        // Pass 1: horizontal sliding-window.
        let mut tmp = vec![0u8; len];
        for y in 0..h {
            let row_off = y * w;
            let mut deque: VecDeque<usize> = VecDeque::new();
            let mut right: usize = 0;
            for x in 0..w {
                let new_right = (x + r).min(w - 1);
                while right < new_right {
                    right += 1;
                    while deque.back().is_some_and(|&b| {
                        let (bv, rv) = (src[row_off + b], src[row_off + right]);
                        if is_max { bv <= rv } else { bv >= rv }
                    }) {
                        deque.pop_back();
                    }
                    deque.push_back(right);
                }
                let left_bound = x.saturating_sub(r);
                while deque.front().is_some_and(|&f| f < left_bound) {
                    deque.pop_front();
                }
                tmp[row_off + x] = if !is_max && (x < r || x + r >= w) {
                    0
                } else {
                    deque.front().map_or(0, |&f| src[row_off + f])
                };
            }
        }

        // Pass 2: vertical sliding-window.
        let mut out = vec![0u8; len];
        for x in 0..w {
            let mut deque: VecDeque<usize> = VecDeque::new();
            let mut bottom: usize = 0;
            for y in 0..h {
                let new_bottom = (y + r).min(h - 1);
                while bottom < new_bottom {
                    bottom += 1;
                    while deque.back().is_some_and(|&b| {
                        let (bv, bv2) = (tmp[b * w + x], tmp[bottom * w + x]);
                        if is_max { bv <= bv2 } else { bv >= bv2 }
                    }) {
                        deque.pop_back();
                    }
                    deque.push_back(bottom);
                }
                let top_bound = y.saturating_sub(r);
                while deque.front().is_some_and(|&f| f < top_bound) {
                    deque.pop_front();
                }
                out[y * w + x] = if !is_max && (y < r || y + r >= h) {
                    0
                } else {
                    deque.front().map_or(0, |&f| tmp[f * w + x])
                };
            }
        }
        out
    }

    fn dilate_alpha_mask(src: &[u8], width: u32, height: u32, radius: u32) -> Vec<u8> {
        Self::sliding_window_filter(src, width, height, radius, true)
    }

    fn erode_alpha_mask(src: &[u8], width: u32, height: u32, radius: u32) -> Vec<u8> {
        Self::sliding_window_filter(src, width, height, radius, false)
    }
}
