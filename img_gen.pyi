from pathlib import Path
from typing import Callable
from enum import Enum, IntEnum

class ColorKind:
    SolidColor: Callable[[SolidColor], ColorKind] = ...
    LinearGradient: Callable[[LinearGradient], ColorKind] = ...
    RadialGradient: Callable[[RadialGradient], ColorKind] = ...
    ConicalGradient: Callable[[ConicalGradient], ColorKind] = ...

    @staticmethod
    def solid_color(r: int, g: int, b: int, a: int) -> ColorKind: ...
    @staticmethod
    def solid_color_from_str(val: str) -> ColorKind: ...
    @staticmethod
    def linear_gradient(
        start: Offset,
        end: Offset,
        colors: list[tuple[float, str]] | None = None,
        preset: Presets | None = None,
        spread: Spread | None = None,
    ) -> ColorKind: ...
    @staticmethod
    def radial_gradient(
        center: Offset,
        radius: float,
        colors: list[tuple[float, str]] | None = None,
        preset: Presets | None = None,
        spread: Spread | None = None,
        focal_point: Offset | None = None,
        focal_radius: float | None = None,
    ) -> ColorKind: ...
    @staticmethod
    def conical_gradient(
        center: Offset,
        angle: float | None = None,
        colors: list[tuple[float, str]] | None = None,
        preset: Presets | None = None,
    ) -> ColorKind: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> ColorKind: ...
    @staticmethod
    def from_json_str(json_str: str) -> ColorKind: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class SolidColor:
    """A solid color."""
    def __init__(self, r: int = 0, g: int = 0, b: int = 0, a: int = 0) -> None: ...
    def to_tuple(self) -> tuple[int, int, int, int]: ...
    @staticmethod
    def from_string(val: str) -> SolidColor: ...
    r: int
    g: int
    b: int
    a: int
    def get_foreground_color(self) -> SolidColor: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> SolidColor: ...
    @staticmethod
    def from_json_str(json_str: str) -> SolidColor: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Spread:
    Pad: Spread = ...
    Reflect: Spread = ...
    Repeat: Spread = ...

class Presets(IntEnum):
    MonoChrome = 0
    WarmLight = 1
    NightFade = 2
    SpringWarmth = 3
    JuicyPeach = 4
    YoungPassion = 5
    LadyLips = 6
    SunnyMorning = 7
    RainyAshville = 8
    FrozenDreams = 9
    WinterNeva = 10
    DustyGrass = 11
    TemptingAzure = 12
    HeavyRain = 13
    AmyCrisp = 14
    MeanFruit = 15
    DeepBlue = 16
    RipeMalinka = 17
    CloudyKnoxville = 18
    MalibuBeach = 19
    NewLife = 20
    TrueSunset = 21
    MorpheusDen = 22
    RareWind = 23
    NearMoon = 24
    WildApple = 25
    SaintPetersburg = 26
    AriellesSmile = 27
    PlumPlate = 28
    EverlastingSky = 29
    HappyFisher = 30
    Blessing = 31
    SharpeyeEagle = 32
    LadogaBottom = 33
    LemonGate = 34
    ItmeoBranding = 35
    ZeusMiracle = 36
    OldHat = 37
    StarWine = 38
    DeepBlue2 = 39
    HappyAcid = 40
    AwesomePine = 41
    NewYork = 42
    ShyRainbow = 43
    MixedHopes = 44
    FlyHigh = 45
    StrongBliss = 46
    FreshMilk = 47
    SnowAgain = 48
    FebruaryInk = 49
    KindSteel = 50
    SoftGrass = 51
    GrownEarly = 52
    SharpBlues = 53
    ShadyWater = 54
    DirtyBeauty = 55
    GreatWhale = 56
    TeenNotebook = 57
    PoliteRumors = 58
    SweetPeriod = 59
    WideMatrix = 60
    SoftCherish = 61
    RedSalvation = 62
    BurningSpring = 63
    NightParty = 64
    SkyGlider = 65
    HeavenPeach = 66
    PurpleDivision = 67
    AquaSplash = 68
    AboveClouds = 69
    SpikyNaga = 70
    LoveKiss = 71
    SharpGrass = 72
    CleanMirror = 73
    PremiumDark = 74
    ColdEvening = 75
    CochitiLake = 76
    SummerGames = 77
    PassionateBed = 78
    MountainRock = 79
    DesertHump = 80
    JungleDay = 81
    PhoenixStart = 82
    OctoberSilence = 83
    FarawayRiver = 84
    AlchemistLab = 85
    OverSun = 86
    PremiumWhite = 87
    MarsParty = 88
    EternalConstance = 89
    JapanBlush = 90
    SmilingRain = 91
    CloudyApple = 92
    BigMango = 93
    HealthyWater = 94
    AmourAmour = 95
    RiskyConcrete = 96
    StrongStick = 97
    ViciousStance = 98
    PaloAlto = 99
    HappyMemories = 100
    MidnightBloom = 101
    Crystalline = 102
    RaccoonBack = 103
    PartyBliss = 104
    ConfidentCloud = 105
    LeCocktail = 106
    RiverCity = 107
    FrozenBerry = 108
    Elegance = 109
    ChildCare = 110
    FlyingLemon = 111
    NewRetrowave = 112
    HiddenJaguar = 113
    AboveTheSky = 114
    Nega = 115
    DenseWater = 116
    Seashore = 117
    MarbleWall = 118
    CheerfulCaramel = 119
    NightSky = 120
    MagicLake = 121
    YoungGrass = 122
    ColorfulPeach = 123
    GentleCare = 124
    PlumBath = 125
    HappyUnicorn = 126
    FullMetal = 127
    AfricanField = 128
    SolidStone = 129
    OrangeJuice = 130
    GlassWater = 131
    SlickCarbon = 132
    NorthMiracle = 133
    FruitBlend = 134
    MillenniumPine = 135
    HighFlight = 136
    MoleHall = 137
    SpaceShift = 138
    ForestInei = 139
    RoyalGarden = 140
    RichMetal = 141
    JuicyCake = 142
    SmartIndigo = 143
    SandStrike = 144
    NorseBeauty = 145
    AquaGuidance = 146
    SunVeggie = 147
    SeaLord = 148
    BlackSea = 149
    GrassShampoo = 150
    LandingAircraft = 151
    WitchDance = 152
    SleeplessNight = 153
    AngelCare = 154
    CrystalRiver = 155
    SoftLipstick = 156
    SaltMountain = 157
    PerfectWhite = 158
    FreshOasis = 159
    StrictNovember = 160
    MorningSalad = 161
    DeepRelief = 162
    SeaStrike = 163
    NightCall = 164
    SupremeSky = 165
    LightBlue = 166
    MindCrawl = 167
    LilyMeadow = 168
    SugarLollipop = 169
    SweetDessert = 170
    MagicRay = 171
    TeenParty = 172
    FrozenHeat = 173
    GagarinView = 174
    FabledSunset = 175
    PerfectBlue = 176

    @staticmethod
    def from_str(val: str) -> Presets: ...
    @staticmethod
    def from_index(i: int) -> Presets: ...

class ColorGradient:
    def __init__(
        self,
        gradient_spec: list[tuple[float, str]] | None = None,
        preset: Presets | None = Presets.MonoChrome,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> ColorGradient: ...
    @staticmethod
    def from_json_str(json_str: str) -> ColorGradient: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class LinearGradient:
    @property
    def start(self) -> Offset: ...
    @start.setter
    def start(self, val: Offset): ...
    @property
    def end(self) -> Offset: ...
    @end.setter
    def end(self, val: Offset): ...
    @property
    def spread(self) -> Spread: ...
    @spread.setter
    def spread(self, val: Spread): ...
    def __init__(
        self,
        colors: ColorGradient,
        start: Offset,
        end: Offset,
        spread: Spread = Spread.Pad,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> LinearGradient: ...
    @staticmethod
    def from_json_str(json_str: str) -> LinearGradient: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class RadialGradient:
    @property
    def center(self) -> Offset: ...
    @center.setter
    def center(self, val: Offset): ...
    @property
    def radius(self) -> float: ...
    @radius.setter
    def radius(self, val: float): ...
    @property
    def focal_point(self) -> Offset: ...
    @focal_point.setter
    def focal_point(self, val: Offset): ...
    @property
    def focal_radius(self) -> float: ...
    @focal_radius.setter
    def focal_radius(self, val: float): ...
    @property
    def spread(self) -> Spread: ...
    @spread.setter
    def spread(self, val: Spread): ...
    def __init__(
        self,
        colors: ColorGradient,
        center: Offset,
        radius: float,
        spread: Spread = Spread.Pad,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> RadialGradient: ...
    @staticmethod
    def from_json_str(json_str: str) -> RadialGradient: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class ConicalGradient:
    @property
    def center(self) -> Offset: ...
    @center.setter
    def center(self, val: Offset): ...
    @property
    def angle(self) -> float: ...
    @angle.setter
    def angle(self, val: float): ...
    def __init__(
        self, colors: ColorGradient, center: Offset, angle: float = 0.0
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> ConicalGradient: ...
    @staticmethod
    def from_json_str(json_str: str) -> ConicalGradient: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Size:
    def __init__(self, width: int | None = None, height: int | None = None) -> None: ...
    @property
    def width(self) -> int | None: ...
    @width.setter
    def width(self, val: int | None): ...
    @property
    def height(self) -> int | None: ...
    @height.setter
    def height(self, val: int | None): ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Size: ...
    @staticmethod
    def from_json_str(json_str: str) -> Size: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Offset:
    def __init__(self, x: int | None = None, y: int | None = None) -> None: ...
    x: int
    y: int
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Offset: ...
    @staticmethod
    def from_json_str(json_str: str) -> Offset: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class PreserveAspect(Enum):
    On = ...
    Off = ...
    Width = ...
    Height = ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> PreserveAspect: ...
    @staticmethod
    def from_json_str(json_str: str) -> PreserveAspect: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Background:
    image: str | None
    color: ColorKind | None
    preserve_aspect: PreserveAspect
    def __init__(
        self,
        image: str | None = None,
        color: ColorKind | None = None,
        preserve_aspect: PreserveAspect = PreserveAspect.On,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Background: ...
    @staticmethod
    def from_json_str(json_str: str) -> Background: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Icon:
    image: str
    color: ColorKind | None
    preserve_aspect: PreserveAspect
    def __init__(
        self,
        image: str,
        color: ColorKind | None = None,
        preserve_aspect: PreserveAspect = PreserveAspect.On,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Icon: ...
    @staticmethod
    def from_json_str(json_str: str) -> Icon: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Corners(Enum):
    TopLeft = ...
    TopRight = ...
    BottomLeft = ...
    BottomRight = ...
    @staticmethod
    def all() -> list[Corners]: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Corners: ...
    @staticmethod
    def from_json_str(json_str: str) -> Corners: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Rectangle:
    border: Border | None
    color: ColorKind
    radius: float
    corners: list[Corners]
    def __init__(
        self,
        color: ColorKind,
        radius: float = 0.0,
        corners: list[Corners] | None = [
            Corners.TopLeft,
            Corners.TopRight,
            Corners.BottomLeft,
            Corners.BottomRight,
        ],
        border: Border | None = None,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Rectangle: ...
    @staticmethod
    def from_json_str(json_str: str) -> Rectangle: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class IrregularPolygonSides:
    def __init__(self, offsets: list[Offset]) -> None: ...
    def get(self) -> list[Offset]: ...

class RegularPolygonSides:
    def __init__(self, sides: int = 3) -> None: ...
    def get(self) -> int: ...

class PolygonSides(Enum):
    Regular = ...
    Irregular = ...

    @staticmethod
    def regular(sides: int) -> PolygonSides: ...
    @staticmethod
    def irregular(offsets: list[Offset]) -> PolygonSides: ...

class Polygon:
    border: Border | None
    color: ColorKind
    rotation: float
    @property
    def sides(self) -> PolygonSides: ...
    @sides.setter
    def sides(self, val: PolygonSides): ...
    def __init__(
        self,
        color: ColorKind,
        border: Border | None = None,
        sides: PolygonSides | None = None,
        rotation: float | None = 0.0,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Polygon: ...
    @staticmethod
    def from_json_str(json_str: str) -> Polygon: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Arc:
    start: float
    end: float
    def __init__(
        self,
        start: float,
        end: float,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Arc: ...
    @staticmethod
    def from_json_str(json_str: str) -> Arc: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Ellipse:
    border: Border | None
    color: ColorKind
    arc: Arc | None
    border_to_origin: bool
    def __init__(
        self,
        color: ColorKind,
        border: Border | None = None,
        arc: Arc | None = None,
        border_to_origin: bool = False,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Ellipse: ...
    @staticmethod
    def from_json_str(json_str: str) -> Ellipse: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Weight(Enum):
    Thin = 100
    Light = 300
    Regular = 400
    Medium = 500
    Bold = 700
    Black = 900

class Font:
    family: str
    style: str
    subset: str
    weight: Weight
    path: str | None
    def __init__(
        self,
        family: str = "Roboto",
        style: str = "normal",
        subset: str = "latin",
        weight: Weight = Weight.Regular,
        path: str | None = None,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Font: ...
    @staticmethod
    def from_json_str(json_str: str) -> Font: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class TypographyAlign(Enum):
    StartTop = ...
    StartCenter = ...
    StartBottom = ...
    CenterTop = ...
    Center = ...
    CenterCenter = ...
    CenterBottom = ...
    EndTop = ...
    EndCenter = ...
    EndBottom = ...

class Line:
    def __init__(self, amount: int, height: float): ...
    @property
    def amount(self) -> int: ...
    @amount.setter
    def amount(self, val: int): ...
    @property
    def height(self) -> float: ...
    @height.setter
    def height(self, val: float): ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Line: ...
    @staticmethod
    def from_json_str(json_str: str) -> Line: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Typography:
    content: str
    align: TypographyAlign
    color: ColorKind
    line: Line
    overflow: bool
    font: Font
    border: Border | None
    def __init__(
        self,
        content: str,
        align: TypographyAlign | None = TypographyAlign.StartTop,
        color: ColorKind | None = ColorKind.SolidColor(SolidColor(0, 0, 0, 255)),
        line: Line | None = Line(amount=1, height=1.0),
        overflow: bool = False,
        font: Font | None = Font(
            family="Roboto",
            style="normal",
            weight=Weight.Regular,
            subset="latin",
        ),
        border: Border | None = None,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Typography: ...
    @staticmethod
    def from_json_str(json_str: str) -> Typography: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Debug:
    @property
    def color(self) -> SolidColor: ...
    @color.setter
    def color(self, val: SolidColor): ...
    @property
    def enable(self) -> bool: ...
    @enable.setter
    def enable(self, val: bool): ...
    @property
    def grid(self) -> bool: ...
    @grid.setter
    def grid(self, val: bool): ...
    @property
    def grid_step(self) -> int: ...
    @grid_step.setter
    def grid_step(self, val: int): ...
    def __init__(
        self,
        enable: bool = False,
        grid: bool = True,
        grid_step: int = 16,
        color: SolidColor | None = None,
    ) -> None: ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Debug: ...
    @staticmethod
    def from_json_str(json_str: str) -> Debug: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Layout:
    def __init__(
        self,
        size: Size | None = None,
        layers: list[Layer] = [],
        debug: Debug | None = None,
    ) -> None: ...
    size: Size
    layers: list[Layer]
    debug: Debug

    @staticmethod
    def from_yaml_str(yaml_str: str) -> Layout: ...
    @staticmethod
    def from_json_str(json_str: str) -> Layout: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...
    def append_layer(self, layer: Layer) -> None: ...
    def extend_layers(self, layers: list[Layer]) -> None: ...

class Layer:
    def __init__(
        self,
        size: Size | None = None,
        offset: Offset | None = None,
        background: Background | None = None,
        ellipse: Ellipse | None = None,
        rectangle: Rectangle | None = None,
        polygon: Polygon | None = None,
        icon: Icon | None = None,
        typography: Typography | None = None,
        mask: Mask | None = None,
    ): ...
    size: Size | None
    offset: Offset | None
    background: Background | None
    icon: Icon | None
    rectangle: Rectangle | None
    polygon: Polygon | None
    ellipse: Ellipse | None
    typography: Typography | None
    mask: Mask | None
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Layer: ...
    @staticmethod
    def from_json_str(json_str: str) -> Layer: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Mask:
    size: Size | None
    offset: Offset
    invert: bool
    background: Background | None
    icon: Icon | None
    rectangle: Rectangle | None
    polygon: Polygon | None
    ellipse: Ellipse | None
    typography: Typography | None
    mask: Mask | None

    def __init__(
        self,
        size: Size | None = None,
        offset: Offset | None = None,
        invert: bool = False,
        background: Background | None = None,
        ellipse: Ellipse | None = None,
        rectangle: Rectangle | None = None,
        polygon: Polygon | None = None,
        icon: Icon | None = None,
        typography: Typography | None = None,
    ): ...
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Mask: ...
    @staticmethod
    def from_json_str(json_str: str) -> Mask: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Border:
    def __init__(
        self,
        color: ColorKind,
        width: int = 1,
    ): ...
    @property
    def width(self) -> int: ...
    @width.setter
    def width(self, val: int): ...
    color: ColorKind
    @staticmethod
    def from_yaml_str(yaml_str: str) -> Border: ...
    @staticmethod
    def from_json_str(json_str: str) -> Border: ...
    def as_yaml_str(self) -> str: ...
    def as_json_str(self) -> str: ...

class Image:
    @property
    def bytes(self) -> bytes: ...
    @property
    def sha256(self) -> str: ...
    def save(self, name: Path | str) -> None: ...

class Generator:
    def __init__(
        self,
        external_resource_paths: list[Path | str] | None = None,
        cache_root: Path | str | None = None,
    ) -> None: ...
    async def render(self, layout: Layout) -> Image: ...
