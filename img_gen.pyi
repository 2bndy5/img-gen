from pathlib import Path
from typing import Callable
from enum import Enum

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

class Spread:
    Pad: Spread = ...
    Reflect: Spread = ...
    Repeat: Spread = ...

class Presets(Enum):
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
    CoupDeGrace = 40
    HappyAcid = 41
    AwesomePine = 42
    NewYork = 43
    ShyRainbow = 44
    LoonCrest = 45
    MixedHopes = 46
    FlyHigh = 47
    StrongBliss = 48
    FreshMilk = 49
    SnowAgain = 50
    FebruaryInk = 51
    KindSteel = 52
    SoftGrass = 53
    GrownEarly = 54
    SharpBlues = 55
    ShadyWater = 56
    DirtyBeauty = 57
    GreatWhale = 58
    TeenNotebook = 59
    PoliteRumors = 60
    SweetPeriod = 61
    WideMatrix = 62
    SoftCherish = 63
    RedSalvation = 64
    BurningSpring = 65
    NightParty = 66
    SkyGlider = 67
    HeavenPeach = 68
    PurpleDivision = 69
    AquaSplash = 70
    AboveClouds = 71
    SpikyNaga = 72
    LoveKiss = 73
    SharpGrass = 74
    CleanMirror = 75
    PremiumDark = 76
    ColdEvening = 77
    CochitiLake = 78
    SummerGames = 79
    PassionateBed = 80
    MountainRock = 81
    DesertHump = 82
    JungleDay = 83
    PhoenixStart = 84
    OctoberSilence = 85
    FarawayRiver = 86
    AlchemistLab = 87
    OverSun = 88
    PremiumWhite = 89
    MarsParty = 90
    EternalConstance = 91
    JapanBlush = 92
    SmilingRain = 93
    CloudyApple = 94
    BigMango = 95
    HealthyWater = 96
    AmourAmour = 97
    RiskyConcrete = 98
    StrongStick = 99
    ViciousStance = 100
    PaloAlto = 101
    HappyMemories = 102
    MidnightBloom = 103
    Crystalline = 104
    RaccoonBack = 105
    PartyBliss = 106
    ConfidentCloud = 107
    LeCocktail = 108
    RiverCity = 109
    FrozenBerry = 110
    Elegance = 111
    ChildCare = 112
    FlyingLemon = 113
    NewRetrowave = 114
    HiddenJaguar = 115
    AboveTheSky = 116
    Nega = 117
    DenseWater = 118
    ChemicAqua = 119
    Seashore = 120
    MarbleWall = 121
    CheerfulCaramel = 122
    NightSky = 123
    MagicLake = 124
    YoungGrass = 125
    ColorfulPeach = 126
    GentleCare = 127
    PlumBath = 128
    HappyUnicorn = 129
    FullMetal = 130
    AfricanField = 131
    SolidStone = 132
    OrangeJuice = 133
    GlassWater = 134
    SlickCarbon = 135
    NorthMiracle = 136
    FruitBlend = 137
    MillenniumPine = 138
    HighFlight = 139
    MoleHall = 140
    EarlGray = 141
    SpaceShift = 142
    ForestInei = 143
    RoyalGarden = 144
    RichMetal = 145
    JuicyCake = 146
    SmartIndigo = 147
    SandStrike = 148
    NorseBeauty = 149
    AquaGuidance = 150
    SunVeggie = 151
    SeaLord = 152
    BlackSea = 153
    GrassShampoo = 154
    LandingAircraft = 155
    WitchDance = 156
    SleeplessNight = 157
    AngelCare = 158
    CrystalRiver = 159
    SoftLipstick = 160
    SaltMountain = 161
    PerfectWhite = 162
    FreshOasis = 163
    StrictNovember = 164
    MorningSalad = 165
    DeepRelief = 166
    SeaStrike = 167
    NightCall = 168
    SupremeSky = 169
    LightBlue = 170
    MindCrawl = 171
    LilyMeadow = 172
    SugarLollipop = 173
    SweetDessert = 174
    MagicRay = 175
    TeenParty = 176
    FrozenHeat = 177
    GagarinView = 178
    FabledSunset = 179
    PerfectBlue = 180

class ColorGradient:
    def __init__(
        self,
        gradient_spec: list[tuple[float, str]] | None = None,
        preset: Presets | None = Presets.MonoChrome,
    ) -> None: ...

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

class Offset:
    def __init__(self, x: int | None = None, y: int | None = None) -> None: ...
    x: int
    y: int

class PreserveAspect(Enum):
    On = ...
    Off = ...
    Width = ...
    Height = ...

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

class Corners(Enum):
    TopLeft = ...
    TopRight = ...
    BottomLeft = ...
    BottomRight = ...
    @staticmethod
    def all() -> list[Corners]: ...

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

class Arc:
    start: float
    end: float
    def __init__(
        self,
        start: float,
        end: float,
    ) -> None: ...

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

class Layout:
    def __init__(
        self,
        size: Size | None = None,
        layers: list[Layer] = [],
        debug: Debug | None = None,
    ) -> None: ...
    size: Size
    layers: list[Layer]

    @staticmethod
    def from_yaml_str(yaml_str: str) -> Layout: ...
    @staticmethod
    def from_json_str(json_str: str) -> Layout: ...
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

class Image:
    @property
    def bytes(self) -> bytes: ...
    @property
    def sha256(self) -> str: ...
    def save(self, name: Path | str) -> None: ...

class Generator:
    def __init__(
        self,
        image_search_paths: list[Path | str] | None = None,
        cache_root: Path | str | None = None,
    ) -> None: ...
    async def render(self, layout: Layout) -> Image: ...
