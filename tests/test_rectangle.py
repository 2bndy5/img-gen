from pathlib import Path
import pytest
from img_gen import (
    Border,
    Rectangle,
    Layout,
    Layer,
    ColorKind,
    SolidColor,
    Corners,
    Size,
    Offset,
    Generator,
)


@pytest.mark.asyncio
@pytest.mark.parametrize("border_width", [0, 20], ids=lambda x: f"{x}px-border")
async def test_rectangle(tmp_path: Path, border_width: int):
    rounded_corners = [
        [],
        [Corners.TopRight, Corners.BottomLeft],
        [Corners.TopLeft, Corners.BottomRight],
        None,
    ]
    radiuses = [0, 100, 250, 50]
    layout = Layout(Size(400, 400), layers=[])
    for index, (corners, radius) in enumerate(zip(rounded_corners, radiuses)):
        r, g, b = (
            255 * (index in [0, 3]),
            255 * (index in [1, 3]),
            255 * (index in [2, 3]),
        )
        layer = Layer(
            size=Size(200, 200),
            offset=Offset(200 * index % 2, 200 * int(index / 2)),
            rectangle=Rectangle(
                color=ColorKind.SolidColor(SolidColor(g, b, r, 127)),
                corners=corners,
                radius=radius,
                border=None
                if border_width == 0
                else Border(
                    ColorKind.SolidColor(SolidColor(r, g, b, 63)),
                    width=border_width,
                ),
            ),
        )
        layout.layers.append(layer)
    gen = Generator(layout)
    img = await gen.render()
    img.save(str(tmp_path / "test_rectangle.png"))
