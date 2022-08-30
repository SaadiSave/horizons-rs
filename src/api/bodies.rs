use serde::Serialize;
use thiserror::Error;

#[repr(transparent)]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[error("{0} is not a valid body identifier")]
pub struct InvalidBodyCode(pub i64);

macro_rules! impl_try_from_int {
    ($name:ident $varname:ident [$($int:ty)+] $match:tt) => {
        $(
            impl TryFrom<$int> for $name {
                type Error = InvalidBodyCode;

                fn try_from($varname: $int) -> Result<Self, Self::Error> {
                    $match
                }
            }
        )+
    };
}

macro_rules! bodies {
    ($(#[$attr:meta])* [repr $repr:ty] $name:ident { $($variant:ident = $code:literal,)* }) => {
        #[repr($repr)]
        $(#[$attr])*
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
            $name num [u32 u64 u128 usize i32 i64 i128 isize] {
                match num {
                    $($code => Ok(Self::$variant),)*
                    _ => Err(InvalidBodyCode(num as i64))
                }
            }
        }

        $crate::impl_from_int_for_enum!($repr, $name);
    };
}

bodies! {
    /// Unnamed objects are in the format S+Designation, e.g. S2010J1
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[allow(non_camel_case_types)]
    [repr u32]
    MajorBody {
        // Sun and Solar barycenter
        SolarSystemBary = 0, Sun = 10,

        // Hermean system
        MercuryBary = 1, Mercury = 199,

        // Venusian system
        VenusBary = 2, Venus = 299,

        // Terran system
        EarthMoonBary = 3, Earth = 399, Moon = 301,

        // Lagrange points
        EM_L1 = 3011, EM_L2 = 3012, EM_L4 = 3014, EM_L5 = 3015,
        SEMB_L1 = 31, SEMB_L2 = 32, SEMB_L4 = 34, SEMB_L5 = 35,

        // Martian system
        MarsBary = 4, Mars = 499,
        Phobos = 401, Deimos = 402,

        // Jovian system
        JupiterBary = 5, Jupiter = 599,

        Io = 501, Europa = 502, Ganymede = 503, Callisto = 504, Amalthea = 505,
        Himalia = 506, Elara = 507, Pasiphae = 508, Sinope = 509, Lysithea = 510,
        Carme = 511, Ananke = 512, Leda = 513, Thebe = 514, Adrastea = 515,
        Metis = 516, Callirrhoe = 517, Themisto = 518, Megaclite = 519, Taygete = 520,
        Chaldene = 521, Harpalyke = 522, Kalyke = 523, Iocaste = 524, Erinome = 525,
        Isonoe = 526, Praxidike = 527, Autonoe = 528, Thyone = 529, Hermippe = 530,
        Aitne = 531, Eurydome = 532, Euanthe = 533, Euporie = 534, Orthosie = 535,
        Sponde = 536, Kale = 537, Pasithee = 538, Hegemone = 539, Mneme = 540,
        Aoede = 541, Thelxinoe = 542, Arche = 543, Kallichore = 544, Helike = 545,
        Carpo = 546, Eukelade = 547, Cyllene = 548, Kore = 549, Herse = 550,
        S2010J1 = 551, S2010J2 = 552, Dia = 553, S2016J1 = 554, S2003J18 = 555,
        S2011J2 = 556, Eirene = 557, Philophrosyne = 558, S2017J1 = 559, Eupheme = 560,
        S2003J19 = 561, Valetudo = 562, S2017J2 = 563, S2017J3 = 564, Pandia = 565,
        S2017J5 = 566, S2017J6 = 567, S2017J7 = 568, S2017J8 = 569, S2017J9 = 570,
        Ersa = 571, S2011J1 = 572,

        S2003J2 = 55501, S2003J4 = 55502, S2003J9 = 55503, S2003J10 = 55504,
        S2003J12 = 55505, S2003J16 = 55506, S2003J23 = 55507, S2003J24 = 55508,

        // Kronian system
        SaturnBary = 6, Saturn = 699,

        Mimas = 601, Enceladus = 602, Tethys = 603, Dione = 604, Rhea = 605,
        Titan = 606, Hyperion = 607, Iapetus = 608, Phoebe = 609, Janus = 610,
        Epimetheus = 611, Helene = 612, Telesto = 613, Calypso = 614, Atlas = 615,
        Prometheus = 616, Pandora = 617, Pan = 618, Ymir = 619, Paaliaq = 620,
        Tarvos = 621, Ijiraq = 622, Suttungr = 623, Kiviuq = 624, Mundilfari = 625,
        Albiorix = 626, Skathi = 627, Erriapus = 628, Siarnaq = 629, Thrymr = 630,
        Narvi = 631, Methone = 632, Pallene = 633, Polydeuces = 634, Daphnis = 635,
        Aegir = 636, Bebhionn = 637, Bergelmir = 638, Bestla = 639, Farbauti = 640,
        Fenrir = 641, Fornjot = 642, Hati = 643, Hyrrokkin = 644, Kari = 645,
        Loge = 646, Skoll = 647, Surtur = 648, Anthe = 649, Jarnsaxa = 650,
        Greip = 651, Tarqeq = 652, Aegaeon = 653, Gridr = 654, Angrboda = 655,
        Skrymir = 656, Gerd = 657, S2004S26 = 658, Eggther = 659, S2004S29 = 660,
        Beli = 661, Gunnlod = 662, Thiazzi = 663, S2004S34 = 664, Alvaldi = 665,
        Geirrod = 666,

        S2004S31 = 65067, S2004S24 = 65070, S2004S28 = 65077, S2004S21 = 65079,
        S2004S36 = 65081, S2004S37 = 65082, S2004S39 = 65084, S2004S7 = 65085,
        S2004S12 = 65086, S2004S13 = 65087, S2004S17 = 65088, S2006S1 = 65089,
        S2006S3 = 65090, S2007S2 = 65091, S2007S3 = 65092, S2019S1 = 65093,

        // Uranian system
        UranusBary = 7, Uranus = 799,
        Ariel = 701, Umbriel = 702, Titania = 703, Oberon = 704, Miranda = 705,
        Cordelia = 706, Ophelia = 707, Bianca = 708, Cressida = 709, Desdemona = 710,
        Juliet = 711, Portia = 712, Rosalind = 713, Belinda = 714, Puck = 715,
        Caliban = 716, Sycorax = 717, Prospero = 718, Setebos = 719, Stephano = 720,
        Trinculo = 721, Francisco = 722, Margaret = 723, Ferdinand = 724, Perdita = 725,
        Mab = 726, Cupid = 727,

        // Neptunian system
        NeptuneBary = 8, Neptune = 899,
        Triton = 801, Nereid = 802, Naiad = 803, Thalassa = 804, Despina = 805,
        Galatea = 806, Larissa = 807, Proteus = 808, Halimede = 809, Psamathe = 810,
        Sao = 811, Laomedeia = 812, Neso = 813, Hippocamp = 814,

        // Hadean system
        PlutoBary = 9, Pluto = 999,
        Charon = 901, Nix = 902, Hydra = 903, Kerberos = 904, Styx = 905,
    }
}

impl Serialize for MajorBody {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_u32(u32::from(self))
    }
}
