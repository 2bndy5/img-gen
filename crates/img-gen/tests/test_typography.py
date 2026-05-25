from pathlib import Path
import pytest
from img_gen import ColorKind
from img_gen import (
    Background,
    Layer,
    Offset,
    Layout,
    Line,
    Size,
    Typography,
    TypographyAlign,
    Border,
    Debug,
    Font,
    Generator,
    SolidColor,
    Weight,
)

CACHE_ROOT = Path(__file__).parent / "out" / "test_cache"
CACHE_ROOT.mkdir(parents=True, exist_ok=True)


# White-background layout with a single constrained typography layer.
def basic_layout(
    canvas_w: int,
    canvas_h: int,
    layer_w: int,
    layer_h: int,
    typography: Typography,
) -> Layout:
    layer_offset = Offset(
        x=int((canvas_w - layer_w) / 2),
        y=int((canvas_h - layer_h) / 2),
    )
    layout = Layout(
        size=Size(canvas_w, canvas_h),
        layers=[],
        debug=Debug(
            enable=True,
            color=SolidColor(85, 35, 0, 255),
            grid=False,
            grid_step=50,
        ),
    )
    bg_layer = Layer(
        background=Background(color=ColorKind.solid_color(255, 255, 255, 255))
    )
    layer = Layer(
        size=Size(layer_w, layer_h), offset=layer_offset, typography=typography
    )
    layout.layers = [bg_layer, layer]
    return layout


@pytest.mark.asyncio
async def test_shrink_fit(tmp_path: Path):
    """`overflow = true`: font is shrunk until all text fits within the layer"""
    layer_w = 200
    layer_h = 60
    canvas_w = 400
    canvas_h = 200

    typography = Typography(
        "All of this text must fit inside the small layer without being cut off.",
        align=TypographyAlign.StartTop,
        line=Line(3, 1.0),
        overflow=True,  # shrink font to fit
    )

    layout = basic_layout(canvas_w, canvas_h, layer_w, layer_h, typography)
    img = await Generator(layout).render(CACHE_ROOT)
    img.save(str(tmp_path / "test_typography_shrink_to_fit.png"))


@pytest.mark.asyncio
async def test_wrap_ellipsis(tmp_path: Path):
    """`overflow = false`: text wraps within the layer width; content that still
    doesn't fit vertically is replaced with a trailing ellipsis so that the
    rendered output stays within the layer height."""
    layer_w = 550
    layer_h = 350

    typography = Typography(
        "This sentence is intentionally very long so that it definitely overflows the small \
         layer height and must be truncated with an ellipsis at the end.",
        align=TypographyAlign.StartTop,
        line=Line(5, 1.2),
        overflow=False,  # wrap + ellipsis
        font=Font(
            # spell-checker: disable-next-line
            family="Playfair Display",
            weight=Weight.Regular,
            style="normal",
            subset="latin",
        ),
        border=Border(
            width=3,
            color=ColorKind.solid_color(255, 0, 0, 255),
        ),
    )

    layout = basic_layout(600, 400, layer_w, layer_h, typography)
    img = await Generator(layout).render(CACHE_ROOT)
    img.save(str(tmp_path / "test_typography_wrap_ellipsis.png"))


@pytest.mark.asyncio
async def test_center(tmp_path: Path):
    layer_w = 200
    layer_h = 100
    canvas_w = 400
    canvas_h = 200

    typography = Typography(
        "Center\nthis\ntext.",
        align=TypographyAlign.CenterCenter,
        line=Line(3, 1.0),
        overflow=True,  # shrink font to fit
    )

    layout = basic_layout(canvas_w, canvas_h, layer_w, layer_h, typography)
    img = await Generator(layout).render()

    img.save(str(tmp_path / "test_typography_center.png"))


@pytest.mark.asyncio
async def test_end_bottom(tmp_path: Path):
    layer_w = 200
    layer_h = 100
    canvas_w = 400
    canvas_h = 200

    typography = Typography(
        "This\nstarts at the `EndBottom`.",
        align=TypographyAlign.EndBottom,
        line=Line(3, 1.0),
        overflow=True,  # shrink font to fit
    )
    layout = basic_layout(canvas_w, canvas_h, layer_w, layer_h, typography)

    img = await Generator(layout).render(CACHE_ROOT)

    img.save(str(tmp_path / "test_typography_end_bottom.png"))


def test_font_legacy_style_parsing_in_constructor():
    font = Font(family="Roboto", style="Bold Italic")

    assert font.weight == Weight.Bold
    assert font.style == "italic"
