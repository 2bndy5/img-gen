from pathlib import Path
import pytest
from img_gen import (
    Background,
    ColorKind,
    Layer,
    Offset,
    Layout,
    Mask,
    Rectangle,
    Size,
    Corners,
    Generator,
)


def mk_mask(inverted: bool) -> Mask:
    return Mask(
        size=Size(40, 40),
        offset=Offset(30, 30),
        invert=inverted,
        rectangle=Rectangle(
            color=ColorKind.solid_color(255, 255, 255, 255),
            radius=15.0,
            corners=[Corners.TopRight, Corners.BottomLeft],
            border=None,
        ),
    )


def mk_layout(inverted: bool) -> Layout:
    layer = Layer(
        size=Size(100, 100),
        offset=Offset(0, 0),
        background=Background(color=ColorKind.solid_color(r=0, g=0, b=255, a=255)),
        mask=mk_mask(inverted),
    )
    layout = Layout(
        size=Size(100, 100),
        layers=[layer],
    )
    return layout


@pytest.mark.asyncio
@pytest.mark.parametrize("inverted", [False, True], ids=lambda x: f"inverted={x}")
async def test_mask(tmp_path: Path, inverted: bool):
    layout = mk_layout(inverted)

    img = await Generator(layout).render()
    img.save(str(tmp_path / "test_mask.png"))
