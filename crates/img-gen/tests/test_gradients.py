from typing import Mapping
from typing import cast
import hashlib
from pathlib import Path
import pytest
from img_gen import (
    Layer,
    Layout,
    Size,
    Offset,
    Background,
    Spread,
    ColorKind,
    Generator,
)

GRADIENT = [(0, "green"), (0.1, "red"), (0.5, "green"), (1, "blue")]


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "spread", [Spread.Pad, Spread.Reflect, Spread.Repeat], ids=lambda x: str(x)
)
async def test_linear_gradient(spread: Spread, tmp_path: Path):
    gradient_domains = [
        dict(start=Offset(x=50, y=50), end=Offset(x=200, y=200)),
        dict(start=Offset(x=200, y=200), end=Offset(x=50, y=50)),
        dict(start=Offset(x=125, y=50), end=Offset(x=125, y=200)),
        dict(start=Offset(x=50, y=125), end=Offset(x=200, y=125)),
    ]
    layout = Layout(size=Size(width=500, height=500), layers=[])
    for index, domain in enumerate(gradient_domains):
        layout.append_layer(
            Layer(
                size=Size(width=250, height=250),
                offset=Offset(x=250 * (index % 2), y=250 * int(index / 2)),
                background=Background(
                    color=ColorKind.linear_gradient(
                        start=domain["start"],
                        end=domain["end"],
                        colors=GRADIENT,
                        spread=spread,
                    )
                ),
            )
        )
    gen = Generator(layout)
    img = await gen.render()
    img.save(str(tmp_path / f"gradient_test_{str(spread)}.png"))


@pytest.mark.asyncio
async def test_conical_gradient(tmp_path: Path):
    gradient_domains = [
        dict(center=Offset(x=50, y=125), angle=0.0),
        dict(center=Offset(x=125, y=125), angle=-45.0),
        dict(center=Offset(x=200, y=125), angle=-180.0),
        dict(center=Offset(x=125, y=125), angle=90.0),
    ]
    layout = Layout(size=Size(width=500, height=500), layers=[])
    for index, domain in enumerate(gradient_domains):
        layout.append_layer(
            Layer(
                size=Size(width=250, height=250),
                offset=Offset(x=250 * (index % 2), y=250 * int(index / 2)),
                background=Background(
                    color=ColorKind.conical_gradient(
                        center=cast(Offset, domain["center"]),
                        angle=cast(float, domain["angle"]),
                        colors=GRADIENT,
                    )
                ),
            )
        )
    gen = Generator(layout)
    img = await gen.render()
    img_bytes = img.bytes
    assert isinstance(img_bytes, bytes)
    img_hash = hashlib.sha256(img_bytes).hexdigest()
    assert img_hash == img.sha256
    img.save(str(tmp_path / f"gradient_test-{img_hash[:16]}.png"))


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "spread", [Spread.Pad, Spread.Reflect, Spread.Repeat], ids=lambda x: str(x)
)
async def test_radial_gradient(spread: Spread, tmp_path: Path):
    gradient_domains = [
        dict(focal_point=None),
        dict(focal_point=Offset(x=75, y=75)),
        dict(focal_point=Offset(x=75, y=75), focal_radius=37.5),
        dict(focal_point=Offset(x=75, y=75), focal_radius=100),
    ]
    layout = Layout(size=Size(width=500, height=500), layers=[])
    for index, domain in enumerate(gradient_domains):
        layout.append_layer(
            Layer(
                size=Size(width=250, height=250),
                offset=Offset(x=250 * (index % 2), y=250 * int(index / 2)),
                background=Background(
                    color=ColorKind.radial_gradient(
                        center=Offset(x=125, y=125),
                        radius=125,
                        colors=GRADIENT,
                        spread=spread,
                        **cast(Mapping, domain),
                    )
                ),
            )
        )
    gen = Generator(layout)
    img = await gen.render()
    img.save(str(tmp_path / f"gradient_test_{str(spread)}.png"))
