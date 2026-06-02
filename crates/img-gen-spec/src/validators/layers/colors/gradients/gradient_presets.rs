#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/*
These presets are taken from https://webgradients.com/
Most of these presets are present in the Qt (C++) framework.

NOTE: Some of these presets are linear approximations of layered CSS gradients
(consisting of both linear and radial gradients).

NOTE: Some gradients' names were altered to satisfy use as enumerations

- 2 different gradients named "Deep Blue" are enumerated as DeepBlue and DeepBlue2
- Arielle's Smile is enumerated as AriellesSmile

MAINTENANCE:
Use scripts/sync_gradient_presets.py to
compare and regenerate preset stops/comments
from https://github.com/itmeo/webgradients sources.
*/

macro_rules! enumerate_specs {
    ( $( $preset_name:ident = $preset_index:literal, [ $( ($color:expr, $point:expr) $(,)? )+ ] $(,)? )+ ) => {

        #[cfg_attr(feature = "pyo3", pyclass(eq, eq_int, module = "img_gen", from_py_object))]
        #[derive(Debug, PartialEq, Clone, Copy)]
        #[doc = "A collection of presets ported from the [webgradients.com](https://webgradients.com)."]
        #[doc = "These are also present in the Qt framework."]
        pub enum Presets {
            $(
                #[doc = concat!("Uses the `", stringify!($preset_name), "` preset gradient.")]
                $preset_name = $preset_index,
            )+
        }

        $(
            struct $preset_name {
                domain: Vec<f32>,
                colors: Vec<&'static str>,
            }

            impl $preset_name {
                pub fn default() -> Self {
                    $preset_name {
                        domain: [
                            $($point,)+
                        ].to_vec(),
                        colors: [
                            $($color,)+
                        ].to_vec()
                    }
                }
            }

        )+


        impl Presets {
            /// Returns the preset index based on declaration order in `enumerate_specs!`.
            pub const fn index(self) -> u8 {
                self as u8
            }

            /// Parses a preset from its declaration-order index in `enumerate_specs!`.
            pub const fn try_from_index(i: u8) -> Option<Self> {
                match i {
                    $(
                        $preset_index => Some(Presets::$preset_name),
                    )*
                    _ => None,
                }
            }

            /// Parses a preset name from its exact enum variant string.
            pub fn try_from_str(val: &str) -> Option<Self> {
                match val {
                    $(
                        stringify!($preset_name) => Some(Presets::$preset_name),
                    )*
                    _ => None,
                }
            }

            /// Returns the preset's canonical color-stop specification.
            pub(crate) fn get_stops(preset: Presets) -> Vec<(f32, &'static str)> {
                match preset {
                    $(
                        Presets::$preset_name => {
                            let preset = $preset_name::default();
                            preset.domain.into_iter().zip(preset.colors).collect()
                        },
                    )*
                }
            }

        }
    };
}

enumerate_specs![
    MonoChrome = 0,
    [("black", 0.0f32), ("white", 1.0f32)],
    // background-image: linear-gradient(45deg, #ff9a9e 0%, #fad0c4 99%, #fad0c4 100%);
    WarmLight = 1,
    [
        ("#ff9a9e", 0.0f32),
        ("#fad0c4", 0.99f32),
        ("#fad0c4", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #a18cd1 0%, #fbc2eb 100%);
    NightFade = 2,
    [("#a18cd1", 0.0f32), ("#fbc2eb", 1.0f32)],
    // background-image: linear-gradient(to top, #fad0c4 0%, #fad0c4 1%, #ffd1ff 100%);
    SpringWarmth = 3,
    [
        ("#fad0c4", 0.0f32),
        ("#fad0c4", 0.01f32),
        ("#ffd1ff", 1.0f32),
    ],
    // background-image: linear-gradient(to right, #ffecd2 0%, #fcb69f 100%);
    JuicyPeach = 4,
    [("#ffecd2", 0.0f32), ("#fcb69f", 1.0f32)],
    // background-image: linear-gradient(to right, #ff8177 0%, #ff867a 0%, #ff8c7f 21%, #f99185 52%, #cf556c 78%, #b12a5b 100%);
    YoungPassion = 5,
    [
        ("#ff8177", 0.0f32),
        ("#ff867a", 0.01f32),
        ("#ff8c7f", 0.21f32),
        ("#f99185", 0.52f32),
        ("#cf556c", 0.78f32),
        ("#b12a5b", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #ff9a9e 0%, #fecfef 99%, #fecfef 100%);
    LadyLips = 6,
    [
        ("#ff9a9e", 0.0f32),
        ("#fecfef", 0.99f32),
        ("#fecfef", 1.0f32),
    ],
    // background-image: linear-gradient(120deg, #f6d365 0%, #fda085 100%);
    SunnyMorning = 7,
    [("#f6d365", 0.0f32), ("#fda085", 1.0f32)],
    // background-image: linear-gradient(to top, #fbc2eb 0%, #a6c1ee 100%);
    RainyAshville = 8,
    [("#fbc2eb", 0.0f32), ("#a6c1ee", 1.0f32)],
    // background-image: linear-gradient(to top, #fdcbf1 0%, #fdcbf1 1%, #e6dee9 100%);
    FrozenDreams = 9,
    [
        ("#fdcbf1", 0.0f32),
        ("#fdcbf1", 0.01f32),
        ("#e6dee9", 1.0f32),
    ],
    // background-image: linear-gradient(120deg, #a1c4fd 0%, #c2e9fb 100%);
    WinterNeva = 10,
    [("#a1c4fd", 0.0f32), ("#c2e9fb", 1.0f32)],
    // background-image: linear-gradient(120deg, #d4fc79 0%, #96e6a1 100%);
    DustyGrass = 11,
    [("#d4fc79", 0.0f32), ("#96e6a1", 1.0f32)],
    // background-image: linear-gradient(120deg, #84fab0 0%, #8fd3f4 100%);
    TemptingAzure = 12,
    [("#84fab0", 0.0f32), ("#8fd3f4", 1.0f32)],
    // background-image: linear-gradient(to top, #cfd9df 0%, #e2ebf0 100%);
    HeavyRain = 13,
    [("#cfd9df", 0.0f32), ("#e2ebf0", 1.0f32)],
    // background-image: linear-gradient(120deg, #a6c0fe 0%, #f68084 100%);
    AmyCrisp = 14,
    [("#a6c0fe", 0.0f32), ("#f68084", 1.0f32)],
    // background-image: linear-gradient(120deg, #fccb90 0%, #d57eeb 100%);
    MeanFruit = 15,
    [("#fccb90", 0.0f32), ("#d57eeb", 1.0f32)],
    // background-image: linear-gradient(120deg, #e0c3fc 0%, #8ec5fc 100%);
    DeepBlue = 16,
    [("#e0c3fc", 0.0f32), ("#8ec5fc", 1.0f32)],
    // background-image: linear-gradient(120deg, #f093fb 0%, #f5576c 100%);
    RipeMalinka = 17,
    [("#f093fb", 0.0f32), ("#f5576c", 1.0f32)],
    // background-image: linear-gradient(120deg, #fdfbfb 0%, #ebedee 100%);
    CloudyKnoxville = 18,
    [("#fdfbfb", 0.0f32), ("#ebedee", 1.0f32)],
    // background-image: linear-gradient(to right, #4facfe 0%, #00f2fe 100%);
    MalibuBeach = 19,
    [("#4facfe", 0.0f32), ("#00f2fe", 1.0f32)],
    // background-image: linear-gradient(to right, #43e97b 0%, #38f9d7 100%);
    NewLife = 20,
    [("#43e97b", 0.0f32), ("#38f9d7", 1.0f32)],
    // background-image: linear-gradient(to right, #fa709a 0%, #fee140 100%);
    TrueSunset = 21,
    [("#fa709a", 0.0f32), ("#fee140", 1.0f32)],
    // background-image: linear-gradient(to top, #30cfd0 0%, #330867 100%);
    MorpheusDen = 22,
    [("#30cfd0", 0.0f32), ("#330867", 1.0f32)],
    // background-image: linear-gradient(to top, #a8edea 0%, #fed6e3 100%);
    RareWind = 23,
    [("#a8edea", 0.0f32), ("#fed6e3", 1.0f32)],
    // background-image: linear-gradient(to top, #5ee7df 0%, #b490ca 100%);
    NearMoon = 24,
    [("#5ee7df", 0.0f32), ("#b490ca", 1.0f32)],
    // background-image: linear-gradient(to top, #d299c2 0%, #fef9d7 100%);
    WildApple = 25,
    [("#d299c2", 0.0f32), ("#fef9d7", 1.0f32)],
    // background-image: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
    SaintPetersburg = 26,
    [("#f5f7fa", 0.0f32), ("#c3cfe2", 1.0f32)],
    // background-image: radial-gradient(circle 248px at center, #16d9e3 0%, #30c7ec 47%, #46aef7 100%);
    AriellesSmile = 27,
    [
        ("#16d9e3", 0.0f32),
        ("#30c7ec", 0.47f32),
        ("#46aef7", 1.0f32),
    ],
    // background-image: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    PlumPlate = 28,
    [("#667eea", 0.0f32), ("#764ba2", 1.0f32)],
    // background-image: linear-gradient(135deg, #fdfcfb 0%, #e2d1c3 100%);
    EverlastingSky = 29,
    [("#fdfcfb", 0.0f32), ("#e2d1c3", 1.0f32)],
    // background-image: linear-gradient(120deg, #89f7fe 0%, #66a6ff 100%);
    HappyFisher = 30,
    [("#89f7fe", 0.0f32), ("#66a6ff", 1.0f32)],
    // background-image: linear-gradient(to top, #fddb92 0%, #d1fdff 100%);
    Blessing = 31,
    [("#fddb92", 0.0f32), ("#d1fdff", 1.0f32)],
    // background-image: linear-gradient(to top, #9890e3 0%, #b1f4cf 100%);
    SharpeyeEagle = 32,
    [("#9890e3", 0.0f32), ("#b1f4cf", 1.0f32)],
    // background-image: linear-gradient(to top, #ebc0fd 0%, #d9ded8 100%);
    LadogaBottom = 33,
    [("#ebc0fd", 0.0f32), ("#d9ded8", 1.0f32)],
    // background-image: linear-gradient(to top, #96fbc4 0%, #f9f586 100%);
    LemonGate = 34,
    [("#96fbc4", 0.0f32), ("#f9f586", 1.0f32)],
    // background-image: linear-gradient(180deg, #2af598 0%, #009efd 100%);
    ItmeoBranding = 35,
    [("#2af598", 0.0f32), ("#009efd", 1.0f32)],
    // background-image: linear-gradient(to top, #cd9cf2 0%, #f6f3ff 100%);
    ZeusMiracle = 36,
    [("#cd9cf2", 0.0f32), ("#f6f3ff", 1.0f32)],
    // background-image: linear-gradient(to right, #e4afcb 0%, #b8cbb8 0%, #b8cbb8 0%, #e2c58b 30%, #c2ce9c 64%, #7edbdc 100%);
    OldHat = 37,
    [
        ("#e4afcb", 0.0f32),
        ("#b8cbb8", 0.01f32),
        ("#e2c58b", 0.3f32),
        ("#c2ce9c", 0.64f32),
        ("#7edbdc", 1.0f32),
    ],
    // background-image: linear-gradient(to right, #b8cbb8 0%, #b8cbb8 0%, #b465da 0%, #cf6cc9 33%, #ee609c 66%, #ee609c 100%);
    StarWine = 38,
    [
        ("#b8cbb8", 0.0f32),
        ("#b465da", 0.01f32),
        ("#cf6cc9", 0.33f32),
        ("#ee609c", 0.66f32),
        ("#ee609c", 1.0f32),
    ],
    // background-image: linear-gradient(to right, #6a11cb 0%, #2575fc 100%);
    DeepBlue2 = 39,
    [("#6a11cb", 0.0f32), ("#2575fc", 1.0f32)],
    // background-image: linear-gradient(to top, #37ecba 0%, #72afd3 100%);
    HappyAcid = 40,
    [("#37ecba", 0.0f32), ("#72afd3", 1.0f32)],
    // background-image: linear-gradient(to top, #ebbba7 0%, #cfc7f8 100%);
    AwesomePine = 41,
    [("#ebbba7", 0.0f32), ("#cfc7f8", 1.0f32)],
    // background-image: linear-gradient(to top, #fff1eb 0%, #ace0f9 100%);
    NewYork = 42,
    [("#fff1eb", 0.0f32), ("#ace0f9", 1.0f32)],
    // background-image: linear-gradient(to right, #eea2a2 0%, #bbc1bf 19%, #57c6e1 42%, #b49fda 79%, #7ac5d8 100%);
    ShyRainbow = 43,
    [
        ("#eea2a2", 0.0f32),
        ("#bbc1bf", 0.19f32),
        ("#57c6e1", 0.42f32),
        ("#b49fda", 0.79f32),
        ("#7ac5d8", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #c471f5 0%, #fa71cd 100%);
    MixedHopes = 44,
    [("#c471f5", 0.0f32), ("#fa71cd", 1.0f32)],
    // background-image: linear-gradient(to top, #48c6ef 0%, #6f86d6 100%);
    FlyHigh = 45,
    [("#48c6ef", 0.0f32), ("#6f86d6", 1.0f32)],
    // background-image: linear-gradient(to right, #f78ca0 0%, #f9748f 19%, #fd868c 60%, #fe9a8b 100%);
    StrongBliss = 46,
    [
        ("#f78ca0", 0.0f32),
        ("#f9748f", 0.19f32),
        ("#fd868c", 0.6f32),
        ("#fe9a8b", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #feada6 0%, #f5efef 100%);
    FreshMilk = 47,
    [("#feada6", 0.0f32), ("#f5efef", 1.0f32)],
    // background-image: linear-gradient(to top, #e6e9f0 0%, #eef1f5 100%);
    SnowAgain = 48,
    [("#e6e9f0", 0.0f32), ("#eef1f5", 1.0f32)],
    // background-image: linear-gradient(to top, #accbee 0%, #e7f0fd 100%);
    FebruaryInk = 49,
    [("#accbee", 0.0f32), ("#e7f0fd", 1.0f32)],
    // background-image: linear-gradient(-20deg, #e9defa 0%, #fbfcdb 100%);
    KindSteel = 50,
    [("#e9defa", 0.0f32), ("#fbfcdb", 1.0f32)],
    // background-image: linear-gradient(to top, #c1dfc4 0%, #deecdd 100%);
    SoftGrass = 51,
    [("#c1dfc4", 0.0f32), ("#deecdd", 1.0f32)],
    // background-image: linear-gradient(to top, #0ba360 0%, #3cba92 100%);
    GrownEarly = 52,
    [("#0ba360", 0.0f32), ("#3cba92", 1.0f32)],
    // background-image: linear-gradient(to top, #00c6fb 0%, #005bea 100%);
    SharpBlues = 53,
    [("#00c6fb", 0.0f32), ("#005bea", 1.0f32)],
    // background-image: linear-gradient(to right, #74ebd5 0%, #9face6 100%);
    ShadyWater = 54,
    [("#74ebd5", 0.0f32), ("#9face6", 1.0f32)],
    // background-image: linear-gradient(to top, #6a85b6 0%, #bac8e0 100%);
    DirtyBeauty = 55,
    [("#6a85b6", 0.0f32), ("#bac8e0", 1.0f32)],
    // background-image: linear-gradient(to top, #a3bded 0%, #6991c7 100%);
    GreatWhale = 56,
    [("#a3bded", 0.0f32), ("#6991c7", 1.0f32)],
    // background-image: linear-gradient(to top, #9795f0 0%, #fbc8d4 100%);
    TeenNotebook = 57,
    [("#9795f0", 0.0f32), ("#fbc8d4", 1.0f32)],
    // background-image: linear-gradient(to top, #a7a6cb 0%, #8989ba 52%, #8989ba 100%);
    PoliteRumors = 58,
    [
        ("#a7a6cb", 0.0f32),
        ("#8989ba", 0.52f32),
        ("#8989ba", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #3f51b1 0%, #5a55ae 13%, #7b5fac 25%, #8f6aae 38%, #a86aa4 50%, #cc6b8e 62%, #f18271 75%, #f3a469 87%, #f7c978 100%);
    SweetPeriod = 59,
    [
        ("#3f51b1", 0.0f32),
        ("#5a55ae", 0.13f32),
        ("#7b5fac", 0.25f32),
        ("#8f6aae", 0.38f32),
        ("#a86aa4", 0.5f32),
        ("#cc6b8e", 0.62f32),
        ("#f18271", 0.75f32),
        ("#f3a469", 0.87f32),
        ("#f7c978", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #fcc5e4 0%, #fda34b 15%, #ff7882 35%, #c8699e 52%, #7046aa 71%, #0c1db8 87%, #020f75 100%);
    WideMatrix = 60,
    [
        ("#fcc5e4", 0.0f32),
        ("#fda34b", 0.15f32),
        ("#ff7882", 0.35f32),
        ("#c8699e", 0.52f32),
        ("#7046aa", 0.71f32),
        ("#0c1db8", 0.87f32),
        ("#020f75", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #dbdcd7 0%, #dddcd7 24%, #e2c9cc 30%, #e7627d 46%, #b8235a 59%, #801357 71%, #3d1635 84%, #1c1a27 100%);
    SoftCherish = 61,
    [
        ("#dbdcd7", 0.0f32),
        ("#dddcd7", 0.24f32),
        ("#e2c9cc", 0.3f32),
        ("#e7627d", 0.46f32),
        ("#b8235a", 0.59f32),
        ("#801357", 0.71f32),
        ("#3d1635", 0.84f32),
        ("#1c1a27", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #f43b47 0%, #453a94 100%);
    RedSalvation = 62,
    [("#f43b47", 0.0f32), ("#453a94", 1.0f32)],
    // background-image: linear-gradient(to top, #4fb576 0%, #44c489 30%, #28a9ae 46%, #28a2b7 59%, #4c7788 71%, #6c4f63 86%, #432c39 100%);
    BurningSpring = 63,
    [
        ("#4fb576", 0.0f32),
        ("#44c489", 0.3f32),
        ("#28a9ae", 0.46f32),
        ("#28a2b7", 0.59f32),
        ("#4c7788", 0.71f32),
        ("#6c4f63", 0.86f32),
        ("#432c39", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #0250c5 0%, #d43f8d 100%);
    NightParty = 64,
    [("#0250c5", 0.0f32), ("#d43f8d", 1.0f32)],
    // background-image: linear-gradient(to top, #88d3ce 0%, #6e45e2 100%);
    SkyGlider = 65,
    [("#88d3ce", 0.0f32), ("#6e45e2", 1.0f32)],
    // background-image: linear-gradient(to top, #d9afd9 0%, #97d9e1 100%);
    HeavenPeach = 66,
    [("#d9afd9", 0.0f32), ("#97d9e1", 1.0f32)],
    // background-image: linear-gradient(to top, #7028e4 0%, #e5b2ca 100%);
    PurpleDivision = 67,
    [("#7028e4", 0.0f32), ("#e5b2ca", 1.0f32)],
    // background-image: linear-gradient(15deg, #13547a 0%, #80d0c7 100%);
    AquaSplash = 68,
    [("#13547a", 0.0f32), ("#80d0c7", 1.0f32)],
    // background-image: linear-gradient(to left, #BDBBBE 0%, #9D9EA3 100%), radial-gradient(88% 271%, rgba(255, 255, 255, 0.25) 0%, rgba(254, 254, 254, 0.25) 1%, rgba(0, 0, 0, 0.25) 100%), radial-gradient(50% 100%, rgba(255, 255, 255, 0.30) 0%, rgba(0, 0, 0, 0.30) 100%);
    AboveClouds = 69,
    [("#BDBBBE", 0.0f32), ("#9D9EA3", 1.0f32)],
    // background-image: linear-gradient(to top, #505285 0%, #585e92 12%, #65689f 25%, #7474b0 37%, #7e7ebb 50%, #8389c7 62%, #9795d4 75%, #a2a1dc 87%, #b5aee4 100%);
    SpikyNaga = 70,
    [
        ("#505285", 0.0f32),
        ("#585e92", 0.12f32),
        ("#65689f", 0.25f32),
        ("#7474b0", 0.37f32),
        ("#7e7ebb", 0.5f32),
        ("#8389c7", 0.62f32),
        ("#9795d4", 0.75f32),
        ("#a2a1dc", 0.87f32),
        ("#b5aee4", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #ff0844 0%, #ffb199 100%);
    LoveKiss = 71,
    [("#ff0844", 0.0f32), ("#ffb199", 1.0f32)],
    // background: #C9CCD3;
    // background-image: linear-gradient(-180deg, rgba(255,255,255,0.50) 0%, rgba(0,0,0,0.50) 100%);
    // background-blend-mode: lighten;
    SharpGrass = 72,
    [("#e3e5e8", 0.0f32), ("#c9ccd3", 1.0f32)],
    // background-image: linear-gradient(45deg, #93a5cf 0%, #e4efe9 100%);
    CleanMirror = 73,
    [("#93a5cf", 0.0f32), ("#e4efe9", 1.0f32)],
    // background-image: linear-gradient(to right, #434343 0%, black 100%);
    PremiumDark = 74,
    [("#434343", 0.0f32), ("black", 1.0f32)],
    // background-image: linear-gradient(to top, #0c3483 0%, #a2b6df 100%, #6b8cce 100%, #a2b6df 100%);
    ColdEvening = 75,
    [
        ("#0c3483", 0.0f32),
        ("#a2b6df", 0.98f32),
        ("#6b8cce", 0.99f32),
        ("#a2b6df", 1.0f32),
    ],
    // background-image: linear-gradient(45deg, #93a5cf 0%, #e4efe9 100%);
    CochitiLake = 76,
    [("#93a5cf", 0.0f32), ("#e4efe9", 1.0f32)],
    // background-image: linear-gradient(to right, #92fe9d 0%, #00c9ff 100%);
    SummerGames = 77,
    [("#92fe9d", 0.0f32), ("#00c9ff", 1.0f32)],
    // background-image: linear-gradient(to right, #ff758c 0%, #ff7eb3 100%);
    PassionateBed = 78,
    [("#ff758c", 0.0f32), ("#ff7eb3", 1.0f32)],
    // background-image: linear-gradient(to right, #868f96 0%, #596164 100%);
    MountainRock = 79,
    [("#868f96", 0.0f32), ("#596164", 1.0f32)],
    // background-image: linear-gradient(to top, #c79081 0%, #dfa579 100%);
    DesertHump = 80,
    [("#c79081", 0.0f32), ("#dfa579", 1.0f32)],
    // background-image: linear-gradient(45deg, #8baaaa 0%, #ae8b9c 100%);
    JungleDay = 81,
    [("#8baaaa", 0.0f32), ("#ae8b9c", 1.0f32)],
    // background-image: linear-gradient(to right, #f83600 0%, #f9d423 100%);
    PhoenixStart = 82,
    [("#f83600", 0.0f32), ("#f9d423", 1.0f32)],
    // background-image: linear-gradient(-20deg, #b721ff 0%, #21d4fd 100%);
    OctoberSilence = 83,
    [("#b721ff", 0.0f32), ("#21d4fd", 1.0f32)],
    // background-image: linear-gradient(-20deg, #6e45e2 0%, #88d3ce 100%);
    FarawayRiver = 84,
    [("#6e45e2", 0.0f32), ("#88d3ce", 1.0f32)],
    // background-image: linear-gradient(-20deg, #d558c8 0%, #24d292 100%);
    AlchemistLab = 85,
    [("#d558c8", 0.0f32), ("#24d292", 1.0f32)],
    // background-image: linear-gradient(60deg, #abecd6 0%, #fbed96 100%);
    OverSun = 86,
    [("#abecd6", 0.0f32), ("#fbed96", 1.0f32)],
    // background-image: linear-gradient(to top, #d5d4d0 0%, #d5d4d0 1%, #eeeeec 31%, #efeeec 75%, #e9e9e7 100%);
    PremiumWhite = 87,
    [
        ("#d5d4d0", 0.0f32),
        ("#d5d4d0", 0.01f32),
        ("#eeeeec", 0.31f32),
        ("#efeeec", 0.75f32),
        ("#e9e9e7", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #5f72bd 0%, #9b23ea 100%);
    MarsParty = 88,
    [("#5f72bd", 0.0f32), ("#9b23ea", 1.0f32)],
    // background-image: linear-gradient(to top, #09203f 0%, #537895 100%);
    EternalConstance = 89,
    [("#09203f", 0.0f32), ("#537895", 1.0f32)],
    // background-image: linear-gradient(-20deg, #ddd6f3 0%, #faaca8 100%, #faaca8 100%);
    JapanBlush = 90,
    [("#ddd6f3", 0.0f32), ("#faaca8", 1.0f32)],
    // background-image: linear-gradient(-20deg, #dcb0ed 0%, #99c99c 100%);
    SmilingRain = 91,
    [("#dcb0ed", 0.0f32), ("#99c99c", 1.0f32)],
    // background-image: linear-gradient(to top, #f3e7e9 0%, #e3eeff 99%, #e3eeff 100%);
    CloudyApple = 92,
    [
        ("#f3e7e9", 0.0f32),
        ("#e3eeff", 0.99f32),
        ("#e3eeff", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #c71d6f 0%, #d09693 100%);
    BigMango = 93,
    [("#c71d6f", 0.0f32), ("#d09693", 1.0f32)],
    // background-image: linear-gradient(60deg, #96deda 0%, #50c9c3 100%);
    HealthyWater = 94,
    [("#96deda", 0.0f32), ("#50c9c3", 1.0f32)],
    // background-image: linear-gradient(to top, #f77062 0%, #fe5196 100%);
    AmourAmour = 95,
    [("#f77062", 0.0f32), ("#fe5196", 1.0f32)],
    // background-image: linear-gradient(to top, #c4c5c7 0%, #dcdddf 52%, #ebebeb 100%);
    RiskyConcrete = 96,
    [
        ("#c4c5c7", 0.0f32),
        ("#dcdddf", 0.52f32),
        ("#ebebeb", 1.0f32),
    ],
    // background-image: linear-gradient(to right, #a8caba 0%, #5d4157 100%);
    StrongStick = 97,
    [("#a8caba", 0.0f32), ("#5d4157", 1.0f32)],
    // background-image: linear-gradient(60deg, #29323c 0%, #485563 100%);
    ViciousStance = 98,
    [("#29323c", 0.0f32), ("#485563", 1.0f32)],
    // background-image: linear-gradient(-60deg, #16a085 0%, #f4d03f 100%);
    PaloAlto = 99,
    [("#16a085", 0.0f32), ("#f4d03f", 1.0f32)],
    // background-image: linear-gradient(-60deg, #ff5858 0%, #f09819 100%);
    HappyMemories = 100,
    [("#ff5858", 0.0f32), ("#f09819", 1.0f32)],
    // background-image: linear-gradient(-20deg, #2b5876 0%, #4e4376 100%);
    MidnightBloom = 101,
    [("#2b5876", 0.0f32), ("#4e4376", 1.0f32)],
    // background-image: linear-gradient(-20deg, #00cdac 0%, #8ddad5 100%);
    Crystalline = 102,
    [("#00cdac", 0.0f32), ("#8ddad5", 1.0f32)],
    // background: linear-gradient(-180deg, #BCC5CE 0%, #929EAD 98%), radial-gradient(at top left, rgba(255,255,255,0.30) 0%, rgba(0,0,0,0.30) 100%);
    RaccoonBack = 103,
    [("#BCC5CE", 0.0f32), ("#929EAD", 0.98f32)],
    // background-image: linear-gradient(to top, #4481eb 0%, #04befe 100%);
    PartyBliss = 104,
    [("#4481eb", 0.0f32), ("#04befe", 1.0f32)],
    // background-image: linear-gradient(to top, #dad4ec 0%, #dad4ec 1%, #f3e7e9 100%);
    ConfidentCloud = 105,
    [
        ("#dad4ec", 0.0f32),
        ("#dad4ec", 0.01f32),
        ("#f3e7e9", 1.0f32),
    ],
    // background-image: linear-gradient(45deg, #874da2 0%, #c43a30 100%);
    LeCocktail = 106,
    [("#874da2", 0.0f32), ("#c43a30", 1.0f32)],
    // background-image: linear-gradient(to top, #4481eb 0%, #04befe 100%);
    RiverCity = 107,
    [("#4481eb", 0.0f32), ("#04befe", 1.0f32)],
    // background-image: linear-gradient(to top, #e8198b 0%, #c7eafd 100%);
    FrozenBerry = 108,
    [("#e8198b", 0.0f32), ("#c7eafd", 1.0f32)],
    // background-image: radial-gradient(73% 147%, #EADFDF 59%, #ECE2DF 100%), radial-gradient(91% 146%, rgba(255,255,255,0.50) 47%, rgba(0,0,0,0.50) 100%);
    Elegance = 109,
    [("#EADFDF", 0.59f32), ("#ECE2DF", 1.0f32)],
    // background-image: linear-gradient(-20deg, #f794a4 0%, #fdd6bd 100%);
    ChildCare = 110,
    [("#f794a4", 0.0f32), ("#fdd6bd", 1.0f32)],
    // background-image: linear-gradient(60deg, #64b3f4 0%, #c2e59c 100%);
    FlyingLemon = 111,
    [("#64b3f4", 0.0f32), ("#c2e59c", 1.0f32)],
    // background-image: linear-gradient(to top, #3b41c5 0%, #a981bb 49%, #ffc8a9 100%);
    NewRetrowave = 112,
    [
        ("#3b41c5", 0.0f32),
        ("#a981bb", 0.49f32),
        ("#ffc8a9", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #0fd850 0%, #f9f047 100%);
    HiddenJaguar = 113,
    [("#0fd850", 0.0f32), ("#f9f047", 1.0f32)],
    // background-image: linear-gradient(to top, lightgrey 0%, lightgrey 1%, #e0e0e0 26%, #efefef 48%, #d9d9d9 75%, #bcbcbc 100%);
    AboveTheSky = 114,
    [
        ("lightgrey", 0.0f32),
        ("lightgrey", 0.01f32),
        ("#e0e0e0", 0.26f32),
        ("#efefef", 0.48f32),
        ("#d9d9d9", 0.75f32),
        ("#bcbcbc", 1.0f32),
    ],
    // background-image: linear-gradient(45deg, #ee9ca7 0%, #ffdde1 100%);
    Nega = 115,
    [("#ee9ca7", 0.0f32), ("#ffdde1", 1.0f32)],
    // background-image: linear-gradient(to right, #3ab5b0 0%, #3d99be 31%, #56317a 100%);
    DenseWater = 116,
    [
        ("#3ab5b0", 0.0f32),
        ("#3d99be", 0.31f32),
        ("#56317a", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #209cff 0%, #68e0cf 100%);
    Seashore = 117,
    [("#209cff", 0.0f32), ("#68e0cf", 1.0f32)],
    // background-image: linear-gradient(to top, #bdc2e8 0%, #bdc2e8 1%, #e6dee9 100%);
    MarbleWall = 118,
    [
        ("#bdc2e8", 0.0f32),
        ("#bdc2e8", 0.01f32),
        ("#e6dee9", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #e6b980 0%, #eacda3 100%);
    CheerfulCaramel = 119,
    [("#e6b980", 0.0f32), ("#eacda3", 1.0f32)],
    // background-image: linear-gradient(to top, #1e3c72 0%, #1e3c72 1%, #2a5298 100%);
    NightSky = 120,
    [
        ("#1e3c72", 0.0f32),
        ("#1e3c72", 0.01f32),
        ("#2a5298", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #d5dee7 0%, #ffafbd 0%, #c9ffbf 100%);
    MagicLake = 121,
    [
        ("#d5dee7", 0.0f32),
        ("#ffafbd", 0.01f32),
        ("#c9ffbf", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #9be15d 0%, #00e3ae 100%);
    YoungGrass = 122,
    [("#9be15d", 0.0f32), ("#00e3ae", 1.0f32)],
    // background-image: linear-gradient(to right, #ed6ea0 0%, #ec8c69 100%);
    ColorfulPeach = 123,
    [("#ed6ea0", 0.0f32), ("#ec8c69", 1.0f32)],
    // background-image: linear-gradient(to right, #ffc3a0 0%, #ffafbd 100%);
    GentleCare = 124,
    [("#ffc3a0", 0.0f32), ("#ffafbd", 1.0f32)],
    // background-image: linear-gradient(to top, #cc208e 0%, #6713d2 100%);
    PlumBath = 125,
    [("#cc208e", 0.0f32), ("#6713d2", 1.0f32)],
    // background-image: linear-gradient(to top, #b3ffab 0%, #12fff7 100%);
    HappyUnicorn = 126,
    [("#b3ffab", 0.0f32), ("#12fff7", 1.0f32)],
    // background: linear-gradient(to bottom, #D5DEE7 0%, #E8EBF2 50%, #E2E7ED 100%), linear-gradient(to bottom, rgba(0,0,0,0.02) 50%, rgba(255,255,255,0.02) 61%, rgba(0,0,0,0.02) 73%), linear-gradient(33deg, rgba(255,255,255,0.20) 0%, rgba(0,0,0,0.20) 100%);
    FullMetal = 127,
    [
        ("#D5DEE7", 0.0f32),
        ("#E8EBF2", 0.5f32),
        ("#E2E7ED", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #65bd60 0%, #5ac1a8 25%, #3ec6ed 50%, #b7ddb7 75%, #fef381 100%);
    AfricanField = 128,
    [
        ("#65bd60", 0.0f32),
        ("#5ac1a8", 0.25f32),
        ("#3ec6ed", 0.5f32),
        ("#b7ddb7", 0.75f32),
        ("#fef381", 1.0f32),
    ],
    // background-image: linear-gradient(to right, #243949 0%, #517fa4 100%);
    SolidStone = 129,
    [("#243949", 0.0f32), ("#517fa4", 1.0f32)],
    // background-image: linear-gradient(-20deg, #fc6076 0%, #ff9a44 100%);
    OrangeJuice = 130,
    [("#fc6076", 0.0f32), ("#ff9a44", 1.0f32)],
    // background-image: linear-gradient(to top, #dfe9f3 0%, white 100%);
    GlassWater = 131,
    [("#dfe9f3", 0.0f32), ("white", 1.0f32)],
    // background: linear-gradient(to bottom, #323232 0%, #3F3F3F 40%, #1C1C1C 150%), linear-gradient(to top, rgba(255,255,255,0.40) 0%, rgba(0,0,0,0.25) 200%);
    SlickCarbon = 132,
    [
        ("#323232", 0.0f32),
        ("#3F3F3F", 0.4f32),
        ("#1C1C1C", 1.5f32),
    ],
    // background-image: linear-gradient(to right, #00dbde 0%, #fc00ff 100%);
    NorthMiracle = 133,
    [("#00dbde", 0.0f32), ("#fc00ff", 1.0f32)],
    // background-image: linear-gradient(to right, #f9d423 0%, #ff4e50 100%);
    FruitBlend = 134,
    [("#f9d423", 0.0f32), ("#ff4e50", 1.0f32)],
    // background-image: linear-gradient(to top, #50cc7f 0%, #f5d100 100%);
    MillenniumPine = 135,
    [("#50cc7f", 0.0f32), ("#f5d100", 1.0f32)],
    // background-image: linear-gradient(to right, #0acffe 0%, #495aff 100%);
    HighFlight = 136,
    [("#0acffe", 0.0f32), ("#495aff", 1.0f32)],
    // background-image: linear-gradient(-20deg, #616161 0%, #9bc5c3 100%);
    MoleHall = 137,
    [("#616161", 0.0f32), ("#9bc5c3", 1.0f32)],
    // background-image: linear-gradient(60deg, #3d3393 0%, #2b76b9 37%, #2cacd1 65%, #35eb93 100%);
    SpaceShift = 138,
    [
        ("#3d3393", 0.0f32),
        ("#2b76b9", 0.37f32),
        ("#2cacd1", 0.65f32),
        ("#35eb93", 1.0f32),
    ],
    // background-image: linear-gradient(to top, #df89b5 0%, #bfd9fe 100%);
    ForestInei = 139,
    [("#df89b5", 0.0f32), ("#bfd9fe", 1.0f32)],
    // background-image: linear-gradient(to right, #ed6ea0 0%, #ec8c69 100%);
    RoyalGarden = 140,
    [("#ed6ea0", 0.0f32), ("#ec8c69", 1.0f32)],
    // background-image: linear-gradient(to right, #d7d2cc 0%, #304352 100%);
    RichMetal = 141,
    [("#d7d2cc", 0.0f32), ("#304352", 1.0f32)],
    // background-image: linear-gradient(to top, #e14fad 0%, #f9d423 100%);
    JuicyCake = 142,
    [("#e14fad", 0.0f32), ("#f9d423", 1.0f32)],
    // background-image: linear-gradient(to top, #b224ef 0%, #7579ff 100%);
    SmartIndigo = 143,
    [("#b224ef", 0.0f32), ("#7579ff", 1.0f32)],
    // background-image: linear-gradient(to right, #c1c161 0%, #c1c161 0%, #d4d4b1 100%);
    SandStrike = 144,
    [("#c1c161", 0.0f32), ("#d4d4b1", 1.0f32)],
    // background-image: linear-gradient(to right, #ec77ab 0%, #7873f5 100%);
    NorseBeauty = 145,
    [("#ec77ab", 0.0f32), ("#7873f5", 1.0f32)],
    // background-image: linear-gradient(to top, #007adf 0%, #00ecbc 100%);
    AquaGuidance = 146,
    [("#007adf", 0.0f32), ("#00ecbc", 1.0f32)],
    // background-image: linear-gradient(-225deg, #20E2D7 0%, #F9FEA5 100%);
    SunVeggie = 147,
    [("#20E2D7", 0.0f32), ("#F9FEA5", 1.0f32)],
    // background-image: linear-gradient(-225deg, #2CD8D5 0%, #C5C1FF 56%, #FFBAC3 100%);
    SeaLord = 148,
    [
        ("#2CD8D5", 0.0f32),
        ("#C5C1FF", 0.56f32),
        ("#FFBAC3", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #2CD8D5 0%, #6B8DD6 48%, #8E37D7 100%);
    BlackSea = 149,
    [
        ("#2CD8D5", 0.0f32),
        ("#6B8DD6", 0.48f32),
        ("#8E37D7", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #DFFFCD 0%, #90F9C4 48%, #39F3BB 100%);
    GrassShampoo = 150,
    [
        ("#DFFFCD", 0.0f32),
        ("#90F9C4", 0.48f32),
        ("#39F3BB", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #5D9FFF 0%, #B8DCFF 48%, #6BBBFF 100%);
    LandingAircraft = 151,
    [
        ("#5D9FFF", 0.0f32),
        ("#B8DCFF", 0.48f32),
        ("#6BBBFF", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #A8BFFF 0%, #884D80 100%);
    WitchDance = 152,
    [("#A8BFFF", 0.0f32), ("#884D80", 1.0f32)],
    // background-image: linear-gradient(-225deg, #5271C4 0%, #B19FFF 48%, #ECA1FE 100%);
    SleeplessNight = 153,
    [
        ("#5271C4", 0.0f32),
        ("#B19FFF", 0.48f32),
        ("#ECA1FE", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #FFE29F 0%, #FFA99F 48%, #FF719A 100%);
    AngelCare = 154,
    [
        ("#FFE29F", 0.0f32),
        ("#FFA99F", 0.48f32),
        ("#FF719A", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #22E1FF 0%, #1D8FE1 48%, #625EB1 100%);
    CrystalRiver = 155,
    [
        ("#22E1FF", 0.0f32),
        ("#1D8FE1", 0.48f32),
        ("#625EB1", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #B6CEE8 0%, #F578DC 100%);
    SoftLipstick = 156,
    [("#B6CEE8", 0.0f32), ("#F578DC", 1.0f32)],
    // background-image: linear-gradient(-225deg, #FFFEFF 0%, #D7FFFE 100%);
    SaltMountain = 157,
    [("#FFFEFF", 0.0f32), ("#D7FFFE", 1.0f32)],
    // background-image: linear-gradient(-225deg, #E3FDF5 0%, #FFE6FA 100%);
    PerfectWhite = 158,
    [("#E3FDF5", 0.0f32), ("#FFE6FA", 1.0f32)],
    // background-image: linear-gradient(-225deg, #7DE2FC 0%, #B9B6E5 100%);
    FreshOasis = 159,
    [("#7DE2FC", 0.0f32), ("#B9B6E5", 1.0f32)],
    // background-image: linear-gradient(-225deg, #CBBACC 0%, #2580B3 100%);
    StrictNovember = 160,
    [("#CBBACC", 0.0f32), ("#2580B3", 1.0f32)],
    // background-image: linear-gradient(-225deg, #B7F8DB 0%, #50A7C2 100%);
    MorningSalad = 161,
    [("#B7F8DB", 0.0f32), ("#50A7C2", 1.0f32)],
    // background-image: linear-gradient(-225deg, #7085B6 0%, #87A7D9 50%, #DEF3F8 100%);
    DeepRelief = 162,
    [
        ("#7085B6", 0.0f32),
        ("#87A7D9", 0.5f32),
        ("#DEF3F8", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #77FFD2 0%, #6297DB 48%, #1EECFF 100%);
    SeaStrike = 163,
    [
        ("#77FFD2", 0.0f32),
        ("#6297DB", 0.48f32),
        ("#1EECFF", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #AC32E4 0%, #7918F2 48%, #4801FF 100%);
    NightCall = 164,
    [
        ("#AC32E4", 0.0f32),
        ("#7918F2", 0.48f32),
        ("#4801FF", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #D4FFEC 0%, #57F2CC 48%, #4596FB 100%);
    SupremeSky = 165,
    [
        ("#D4FFEC", 0.0f32),
        ("#57F2CC", 0.48f32),
        ("#4596FB", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #9EFBD3 0%, #57E9F2 48%, #45D4FB 100%);
    LightBlue = 166,
    [
        ("#9EFBD3", 0.0f32),
        ("#57E9F2", 0.48f32),
        ("#45D4FB", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #473B7B 0%, #3584A7 51%, #30D2BE 100%);
    MindCrawl = 167,
    [
        ("#473B7B", 0.0f32),
        ("#3584A7", 0.51f32),
        ("#30D2BE", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #65379B 0%, #886AEA 53%, #6457C6 100%);
    LilyMeadow = 168,
    [
        ("#65379B", 0.0f32),
        ("#886AEA", 0.53f32),
        ("#6457C6", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #A445B2 0%, #D41872 52%, #FF0066 100%);
    SugarLollipop = 169,
    [
        ("#A445B2", 0.0f32),
        ("#D41872", 0.52f32),
        ("#FF0066", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #7742B2 0%, #F180FF 52%, #FD8BD9 100%);
    SweetDessert = 170,
    [
        ("#7742B2", 0.0f32),
        ("#F180FF", 0.52f32),
        ("#FD8BD9", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #FF3CAC 0%, #562B7C 52%, #2B86C5 100%);
    MagicRay = 171,
    [
        ("#FF3CAC", 0.0f32),
        ("#562B7C", 0.52f32),
        ("#2B86C5", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #FF057C 0%, #8D0B93 50%, #321575 100%);
    TeenParty = 172,
    [
        ("#FF057C", 0.0f32),
        ("#8D0B93", 0.5f32),
        ("#321575", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #FF057C 0%, #7C64D5 48%, #4CC3FF 100%);
    FrozenHeat = 173,
    [
        ("#FF057C", 0.0f32),
        ("#7C64D5", 0.48f32),
        ("#4CC3FF", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #69EACB 0%, #EACCF8 48%, #6654F1 100%);
    GagarinView = 174,
    [
        ("#69EACB", 0.0f32),
        ("#EACCF8", 0.48f32),
        ("#6654F1", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #231557 0%, #44107A 29%, #FF1361 67%, #FFF800 100%);
    FabledSunset = 175,
    [
        ("#231557", 0.0f32),
        ("#44107A", 0.29f32),
        ("#FF1361", 0.67f32),
        ("#FFF800", 1.0f32),
    ],
    // background-image: linear-gradient(-225deg, #3D4E81 0%, #5753C9 48%, #6E7FF3 100%);
    PerfectBlue = 176,
    [
        ("#3D4E81", 0.0f32),
        ("#5753C9", 0.48f32),
        ("#6E7FF3", 1.0f32),
    ],
];
