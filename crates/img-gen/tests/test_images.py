from pathlib import Path
import pytest
from img_gen import (
    Background,
    Icon,
    PreserveAspect,
    Layout,
    Layer,
    ColorKind,
    Size,
    Offset,
    Generator,
)

CUR_PATH = Path(__file__).parent
SVG_ASSET = CUR_PATH / "message.svg"
PNG_ASSET = CUR_PATH / "message.png"

OFFSETS = [Offset(0, 0), Offset(100, 0), Offset(0, 100), Offset(150, 0)]
SIZES = [Size(100, 50), Size(50, 100), Size(150, 150), Size(200, 250)]
ASPECTS = [
    PreserveAspect.Height,
    PreserveAspect.Width,
    PreserveAspect.Off,
    PreserveAspect.On,
]
OVERALL_SIZE = Size(350, 250)


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "image",
    [SVG_ASSET, PNG_ASSET],
    ids=["svg", "png"],
)
async def test_background(image: Path, tmp_path: Path):
    layout = Layout(size=OVERALL_SIZE, layers=[])
    for index, (size, offset, aspect) in enumerate(zip(SIZES, OFFSETS, ASPECTS)):
        layout.append_layer(
            Layer(
                size=size,
                offset=offset,
                background=Background(
                    image=str(image),
                    color=ColorKind.solid_color(
                        255 * (index in [0, 3]),
                        255 * (index in [1, 3]),
                        255 * (index in [2, 3]),
                        100,
                    ),
                    preserve_aspect=aspect,
                ),
            )
        )
    gen = Generator()
    img = await gen.render(layout)
    img.save(str(tmp_path / f"img_test_{image.suffix.lstrip('.')}.png"))


@pytest.mark.asyncio
@pytest.mark.parametrize("image", [SVG_ASSET, PNG_ASSET], ids=["svg", "png"])
async def test_icon(image: Path, tmp_path: Path):
    layout = Layout(size=OVERALL_SIZE, layers=[])
    for index, (size, offset, aspect) in enumerate(zip(SIZES, OFFSETS, ASPECTS)):
        layout.append_layer(
            Layer(
                size=size,
                offset=offset,
                icon=Icon(
                    image=str(image),
                    color=ColorKind.solid_color(
                        255 * (index in [0, 3]),
                        255 * (index in [1, 3]),
                        255 * (index in [2, 3]),
                        127,
                    ),
                    preserve_aspect=aspect,
                ),
            )
        )
    gen = Generator()
    img = await gen.render(layout)
    img.save(str(tmp_path / f"img_test_{image.suffix.lstrip('.')}.png"))
