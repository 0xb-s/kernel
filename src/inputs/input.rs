impl TryFrom<u16> for Key {
    type Error = NotSupportedKey;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Key::Reserved),
            1 => Ok(Key::ESC),
            2 => Ok(Key::One),
            3 => Ok(Key::Two),
            4 => Ok(Key::Three),
            5 => Ok(Key::Four),
            6 => Ok(Key::Five),
            7 => Ok(Key::Six),
            8 => Ok(Key::Seven),
            9 => Ok(Key::Eight),
            10 => Ok(Key::Nine),
            11 => Ok(Key::Zero),
            12 => Ok(Key::Minus),
            13 => Ok(Key::Equal),
            14 => Ok(Key::BackSpace),
            15 => Ok(Key::Tab),
            16 => Ok(Key::Q),
            17 => Ok(Key::W),
            18 => Ok(Key::E),
            19 => Ok(Key::R),
            20 => Ok(Key::T),
            21 => Ok(Key::Y),
            22 => Ok(Key::U),
            23 => Ok(Key::I),
            24 => Ok(Key::O),
            25 => Ok(Key::P),
            26 => Ok(Key::LeftBrace),
            27 => Ok(Key::RightBrace),
            28 => Ok(Key::Enter),
            29 => Ok(Key::LeftCtrl),
            30 => Ok(Key::A),
            31 => Ok(Key::S),
            32 => Ok(Key::D),
            33 => Ok(Key::F),
            34 => Ok(Key::G),
            35 => Ok(Key::H),
            36 => Ok(Key::J),
            37 => Ok(Key::K),
            38 => Ok(Key::L),
            39 => Ok(Key::SemiColon),
            40 => Ok(Key::Apostrophe),
            41 => Ok(Key::Grave),
            42 => Ok(Key::LeftShift),
            43 => Ok(Key::BackSlash),
            44 => Ok(Key::Z),
            45 => Ok(Key::X),
            46 => Ok(Key::C),
            47 => Ok(Key::V),
            48 => Ok(Key::B),
            49 => Ok(Key::N),
            50 => Ok(Key::M),
            51 => Ok(Key::Comma),
            52 => Ok(Key::Dot),
            53 => Ok(Key::Slash),
            54 => Ok(Key::RightShift),
            55 => Ok(Key::KpAsterisk),
            56 => Ok(Key::LeftAlt),
            57 => Ok(Key::Space),
            58 => Ok(Key::Capslock),
            59 => Ok(Key::F1),
            60 => Ok(Key::F2),
            61 => Ok(Key::F3),
            62 => Ok(Key::F4),
            63 => Ok(Key::F5),
            64 => Ok(Key::F6),
            65 => Ok(Key::F7),
            66 => Ok(Key::F8),
            67 => Ok(Key::F9),
            68 => Ok(Key::F10),
            69 => Ok(Key::NumLock),
            70 => Ok(Key::ScrollLock),
            71 => Ok(Key::Kp7),
            72 => Ok(Key::Kp8),
            73 => Ok(Key::Kp9),
            74 => Ok(Key::KpMinus),
            75 => Ok(Key::Kp4),
            76 => Ok(Key::Kp5),
            77 => Ok(Key::Kp6),
            78 => Ok(Key::KpPlus),
            79 => Ok(Key::Kp1),
            80 => Ok(Key::Kp2),
            81 => Ok(Key::Kp3),
            82 => Ok(Key::Kp0),
            83 => Ok(Key::KpDot),
            87 => Ok(Key::F11),
            88 => Ok(Key::F12),
            96 => Ok(Key::KpEnter),
            97 => Ok(Key::RightCtrl),
            98 => Ok(Key::KpSlash),
            100 => Ok(Key::RightAlt),
            101 => Ok(Key::LineFeed),
            102 => Ok(Key::Home),
            103 => Ok(Key::Up),
            104 => Ok(Key::PageUp),
            105 => Ok(Key::Left),
            106 => Ok(Key::Right),
            107 => Ok(Key::End),
            108 => Ok(Key::Down),
            109 => Ok(Key::PageDown),
            110 => Ok(Key::Insert),
            111 => Ok(Key::Delete),
            125 => Ok(Key::LeftMeta),
            _ => Err(NotSupportedKey::KeyIsNotSupported),
        }
    }
}
pub enum NotSupportedKey {
    KeyIsNotSupported,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u16)]
pub enum Key {
    Reserved = 0,
    ESC = 1,
    One = 2,
    Two = 3,
    Three = 4,
    Four = 5,
    Five = 6,
    Six = 7,
    Seven = 8,
    Eight = 9,
    Nine = 10,
    Zero = 11,
    Minus = 12,
    Equal = 13,
    BackSpace = 14,
    Tab = 15,
    Q = 16,
    W = 17,
    E = 18,
    R = 19,
    T = 20,
    Y = 21,
    U = 22,
    I = 23,
    O = 24,
    P = 25,

    LeftBrace = 26,

    RightBrace = 27,
    Enter = 28,
    LeftCtrl = 29,
    A = 30,
    S = 31,
    D = 32,
    F = 33,
    G = 34,
    H = 35,
    J = 36,
    K = 37,
    L = 38,

    SemiColon = 39,

    Apostrophe = 40,

    Grave = 41,
    LeftShift = 42,

    BackSlash = 43,
    Z = 44,
    X = 45,
    C = 46,
    V = 47,
    B = 48,
    N = 49,
    M = 50,
    Comma = 51,
    Dot = 52,
    Slash = 53,
    RightShift = 54,
    KpAsterisk = 55,
    LeftAlt = 56,
    Space = 57,
    Capslock = 58,
    F1 = 59,
    F2 = 60,
    F3 = 61,
    F4 = 62,
    F5 = 63,
    F6 = 64,
    F7 = 65,
    F8 = 66,
    F9 = 67,
    F10 = 68,
    NumLock = 69,
    ScrollLock = 70,
    Kp7 = 71,
    Kp8 = 72,
    Kp9 = 73,
    KpMinus = 74,
    Kp4 = 75,
    Kp5 = 76,
    Kp6 = 77,
    KpPlus = 78,
    Kp1 = 79,
    Kp2 = 80,
    Kp3 = 81,
    Kp0 = 82,
    KpDot = 83,

    F11 = 87,
    F12 = 88,

    KpEnter = 96,
    RightCtrl = 97,
    KpSlash = 98,

    RightAlt = 100,
    LineFeed = 101,
    Home = 102,
    Up = 103,
    PageUp = 104,
    Left = 105,
    Right = 106,
    End = 107,
    Down = 108,
    PageDown = 109,
    Insert = 110,
    Delete = 111,

    LeftMeta = 125,
}
impl core::fmt::Display for NotSupportedKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NotSupportedKey::KeyIsNotSupported => {
                write!(f, "The provided key value is not supported.")
            }
        }
    }
}
