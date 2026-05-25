from pathlib import Path
import pytest
from img_gen import (
    Border,
    Ellipse,
    Layout,
    Layer,
    ColorKind,
    Size,
    Offset,
    Generator,
)

SIZES = [Size(200, 100), Size(100, 200), Size(200, 200)]
OFFSETS = [Offset(100, 0), Offset(0, 100), Offset(100, 100)]


@pytest.mark.asyncio
@pytest.mark.parametrize("border_width", [0, 20], ids=lambda x: f"{x}px-border")
async def test_ellipse(tmp_path: Path, border_width: int):
    layout = Layout(Size(300, 300), layers=[])
    for index, (size, offset) in enumerate(zip(SIZES, OFFSETS)):
        r, g, b = (
            255 * (index in [0, 3]),
            255 * (index in [1, 3]),
            255 * (index in [2, 3]),
        )
        layout.append_layer(
            Layer(
                size=size,
                offset=offset,
                ellipse=Ellipse(
                    color=ColorKind.solid_color(g, b, r, 127),
                    border=None
                    if border_width == 0
                    else Border(
                        ColorKind.solid_color(r, g, b, 63),
                        width=border_width,
                    ),
                ),
            )
        )
    gen = Generator(layout)
    img = await gen.render()
    img.save(str(tmp_path / "test_ellipse.png"))
