from pathlib import Path
import pytest
from img_gen import (
    Border,
    Polygon,
    PolygonSides,
    Layout,
    Layer,
    ColorKind,
    Size,
    Offset,
    Generator,
)


@pytest.mark.asyncio
@pytest.mark.parametrize("border_width", [0, 20], ids=lambda x: f"{x}px-border")
async def test_polygon(tmp_path: Path, border_width: int):
    polygon_sides = [
        PolygonSides.regular(3),
        PolygonSides.regular(5),
        PolygonSides.regular(6),
        PolygonSides.irregular(
            [
                Offset(x=40, y=20),
                Offset(x=190, y=40),
                Offset(x=170, y=180),
                Offset(x=20, y=150),
            ]
        ),
    ]
    layout = Layout(Size(400, 400), layers=[])
    for index, sides in enumerate(polygon_sides):
        r, g, b = (
            255 * (index in [0, 3]),
            255 * (index in [1, 3]),
            255 * (index in [2, 3]),
        )
        layout.append_layer(
            Layer(
                size=Size(200, 200),
                offset=Offset(200 * (index % 2), 200 * int(index / 2)),
                polygon=Polygon(
                    color=ColorKind.solid_color(g, b, r, 127),
                    sides=sides,
                    rotation=90 * (index in [0, 3]),
                    border=None
                    if border_width == 0
                    else Border(
                        ColorKind.solid_color(r, g, b, 63),
                        width=border_width,
                    ),
                ),
            )
        )
    gen = Generator()
    img = await gen.render(layout)
    img.save(str(tmp_path / "test_polygon.png"))
