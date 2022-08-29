use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BodyCodeError(pub i64);

impl std::fmt::Display for BodyCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} is not a valid body code in the horizons system",
            self.0
        )
    }
}

impl std::error::Error for BodyCodeError {}

macro_rules! impl_try_from_int {
    ($name:ident $varname:ident [$($int:ty)+] $match:tt) => {
        $(
            impl TryFrom<$int> for $name {
                type Error = BodyCodeError;

                fn try_from($varname: $int) -> Result<Self, Self::Error> {
                    $match
                }
            }
        )+
    };
}

macro_rules! bodies {
    ($(#[$attr:meta])* [repr $repr:ty] $name:ident { $($variant:ident = $code:literal,)* }) => {
        $(#[$attr])*
        #[repr($repr)]
        pub enum $name {
            $($variant = $code,)*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => f.write_str(stringify!($variant)),)*
                }
            }
        }

        impl_try_from_int! {
            $name num [u16 u32 u64 u128 usize i16 i32 i64 i128 isize] {
                match num {
                    $($code => Ok(Self::$variant),)*
                    _ => Err(BodyCodeError(num as i64))
                }
            }
        }

        $crate::impl_from_int_for_enum!($repr, $name);
    };
}

bodies! {
    /// Unnamed objects are in the format S+Designation, e.g. S2010J1; or the format planet+number, e.g. Jupiter52
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    [repr u16]
    MajorBody {
        // Sun
        SolarSystemBary = 0, Sun = 10,

        // Mercury and Venus
        MercuryBary = 1, Mercury = 199,
        VenusBary = 2, Venus = 299,

        // Terran system
        EarthBary = 3, Earth = 399, Moon = 301,

        // Lagrange points
        Lagrange1 = 31, Lagrange2 = 32, Lagrange4 = 34, Lagrange5 = 35,

        // Martian system
        MarsBary = 4, Mars = 499,
        Phobos = 401, Deimos = 402,

        // Jovian System
        JupiterBary = 5, Jupiter = 599,
        Io = 501, Europa = 502,
        Ganymede = 503, Callisto = 504,

        // Other named jovian moons
        Amalthea = 505, Himalia = 506, Elara = 507, Pasiphae = 508, Sinope = 509,
        Lysithea = 510, Carme = 511, Ananke = 512, Leda = 513, Thebe = 514,
        Adrastea = 515, Metis = 516, Callirrhoe = 517, Themisto = 518, Megaclite = 519,
        Taygete = 520, Chaldene = 521, Harpalyke = 522, Kalyke = 523, Iocaste = 524,
        Erinome = 525, Isonoe = 526, Praxidike = 527, Autonoe = 528, Thyone = 529,
        Hermippe = 530, Aitne = 531, Eurydome = 532, Euanthe = 533, Euporie = 534,
        Orthosie = 535, Sponde = 536, Kale = 537, Pasithee = 538, Hegemone = 539,
        Mneme = 540, Aoede = 541, Thelxinoe = 542, Arche = 543, Kallichore = 544,
        Helike = 545, Carpo = 546, Eukelade = 547, Cyllene = 548, Kore = 549,
        Herse = 550, Dia = 553, Eirene = 557, Philophrosyne = 558, Eupheme = 560,
        Valetudo = 562, Pandia = 565, Ersa = 571,

        // Unnamed jovian moons
        S2010J1 = 551, S2010J2 = 552, S2016J1 = 554, S2003J18 = 555, S2011J2 = 556,
        S2017J1 = 559, S2003J19 = 561, S2017J2 = 563, S2017J3 = 564, S2017J5 = 566,
        S2017J6 = 567, S2017J7 = 568, S2017J8 = 569, S2017J9 = 570,

        // Saturn System
        SaturnBary = 6, Saturn = 699,

        // Uranus System
        UranusBary = 7, Uranus = 799,

        // Neptune System
        NeptuneBary = 8, Neptune = 899,

        // Pluto System
        PlutoBary = 9, Pluto = 999,
        Charon = 901, Nix = 902,
        Hydra = 903, Kerberos = 904,
        Styx = 905,
    }
}

impl Serialize for MajorBody {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_u16(u16::from(self))
    }
}
