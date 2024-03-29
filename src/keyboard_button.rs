use std::fmt::Debug;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyboardButton {
    Undefined = 0x00,
    LButton = 0x01,
    RButton = 0x02,
    Cancel = 0x03,
    MButton = 0x04,
    XButton1 = 0x05,
    XButton2 = 0x06,
    Back = 0x08,
    Tab = 0x09,
    Clear = 0x0C,
    Return = 0x0D,
    Shift = 0x10,
    Control = 0x11,
    Menu = 0x12,
    Pause = 0x13,
    Capital = 0x14,
    KanaOrHangeulOrHanja = 0x15,
    Junja = 0x17,
    Final = 0x18,
    HanjaOrKanji = 0x19,
    Escape = 0x1B,
    Convert = 0x1C,
    NonConvert = 0x1D,
    Accept = 0x1E,
    ModeChange = 0x1F,
    Space = 0x20,
    Prior = 0x21,
    Next = 0x22,
    End = 0x23,
    Home = 0x24,
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,
    Select = 0x29,
    Print = 0x2A,
    Execute = 0x2B,
    Snapshot = 0x2C,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,
    LWin = 0x5B,
    RWin = 0x5C,
    Apps = 0x5D,
    Sleep = 0x5F,
    Numpad0 = 0x60,
    Numpad1 = 0x61,
    Numpad2 = 0x62,
    Numpad3 = 0x63,
    Numpad4 = 0x64,
    Numpad5 = 0x65,
    Numpad6 = 0x66,
    Numpad7 = 0x67,
    Numpad8 = 0x68,
    Numpad9 = 0x69,
    Multiply = 0x6A,
    Add = 0x6B,
    Separator = 0x6C,
    Substract = 0x6D,
    Decimal = 0x6E,
    Divide = 0x6F,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,
    NavigationView = 0x88,
    NavigationMenu = 0x89,
    NavigationUp = 0x8A,
    NavigationDown = 0x8B,
    NavigationLeft = 0x8C,
    NavigationRight = 0x8D,
    NavigationAccept = 0x8E,
    NavigationCancel = 0x8F,
    NumLock = 0x90,
    Scroll = 0x91,
    OemNecEqualOrOemFjJisho = 0x92,
    OemFjMasshou = 0x93,
    OemFjTouroku = 0x94,
    OemFjLoya = 0x95,
    OemFjRoya = 0x96,
    LShift = 0xA0,
    RShift = 0xA1,
    LControl = 0xA2,
    RControl = 0xA3,
    LMenu = 0xA4,
    RMenu = 0xA5,
    BrowserBack = 0xA6,
    BrowserForward = 0xA7,
    BrowserRefresh = 0xA8,
    BrowserStop = 0xA9,
    BrowserSearch = 0xAA,
    BrowserFavorites = 0xAB,
    BrowserHome = 0xAC,
    VolumeMute = 0xAD,
    VolumeDown = 0xAE,
    VolumeUp = 0xAF,
    MediaNextTrack = 0xB0,
    MediaPrevTrack = 0xB1,
    MediaStop = 0xB2,
    MediaPlayPause = 0xB3,
    LaunchMail = 0xB4,
    LaunchMediaSelect = 0xB5,
    LaunchApp1 = 0xB6,
    LaunchApp2 = 0xB7,
    Oem1 = 0xBA,
    OemPlus = 0xBB,
    OemComma = 0xBC,
    OemMinus = 0xBD,
    OemPeriod = 0xBE,
    Oem2 = 0xBF,
    Oem3 = 0xC0,
    GamepadA = 0xC3,
    GamepadB = 0xC4,
    GamepadX = 0xC5,
    GamepadY = 0xC6,
    GamepadRightShoulder = 0xC7,
    GamepadLeftShoulder = 0xC8,
    GamepadLeftTrigger = 0xC9,
    GamepadRightTrigger = 0xCA,
    GamepadDpadUp = 0xCB,
    GamepadDpadDown = 0xCC,
    GamepadDpadLeft = 0xCD,
    GamepadDpadRight = 0xCE,
    GamepadMenu = 0xCF,
    GamepadView = 0xD0,
    GamepadLeftThumbstickButton = 0xD1,
    GamepadRightThumbstickButton = 0xD2,
    GamepadLeftThumbstickUp = 0xD3,
    GamepadLeftThumbstickDown = 0xD4,
    GamepadLeftThumbstickRight = 0xD5,
    GamepadLeftThumbstickLeft = 0xD6,
    GamepadRightThumbstickUp = 0xD7,
    GamepadRightThumbstickDown = 0xD8,
    GamepadRightThumbstickRight = 0xD9,
    GamepadRightThumbstickLeft = 0xDA,
    Oem4 = 0xDB,
    Oem5 = 0xDC,
    Oem6 = 0xDD,
    Oem7 = 0xDE,
    Oem8 = 0xDF,
    OemAx = 0xE1,
    Oem102 = 0xE2,
    IcoHelp = 0xE3,
    Ico00 = 0xE4,
    ProcessKey = 0xE5,
    IcoClear = 0xE6,
    Packet = 0xE7,
    OemReset = 0xE9,
    OemJump = 0xEA,
    OemPa1 = 0xEB,
    OemPa2 = 0xEC,
    OemPa3 = 0xED,
    OemWsctrl = 0xEE,
    OemCusel = 0xEF,
    OemAttn = 0xF0,
    OemFinish = 0xF1,
    OemCopy = 0xF2,
    OemAuto = 0xF3,
    OemEnlw = 0xF4,
    OemBacktab = 0xF5,
    Attn = 0xF6,
    Crsel = 0xF7,
    Exsel = 0xF8,
    Ereof = 0xF9,
    Play = 0xFA,
    Zoom = 0xFB,
    Noname = 0xFC,
    Pa1 = 0xFD,
    OemClear = 0xFE,
}

impl KeyboardButton {
    pub fn from_u32(value: u32) -> KeyboardButton {
        match value {
            0x01 => KeyboardButton::LButton,
            0x02 => KeyboardButton::RButton,
            0x03 => KeyboardButton::Cancel,
            0x04 => KeyboardButton::MButton,
            0x05 => KeyboardButton::XButton1,
            0x06 => KeyboardButton::XButton2,
            0x08 => KeyboardButton::Back,
            0x09 => KeyboardButton::Tab,
            0x0C => KeyboardButton::Clear,
            0x0D => KeyboardButton::Return,
            0x10 => KeyboardButton::Shift,
            0x11 => KeyboardButton::Control,
            0x12 => KeyboardButton::Menu,
            0x13 => KeyboardButton::Pause,
            0x14 => KeyboardButton::Capital,
            0x15 => KeyboardButton::KanaOrHangeulOrHanja,
            0x17 => KeyboardButton::Junja,
            0x18 => KeyboardButton::Final,
            0x19 => KeyboardButton::HanjaOrKanji,
            0x1B => KeyboardButton::Escape,
            0x1C => KeyboardButton::Convert,
            0x1D => KeyboardButton::NonConvert,
            0x1E => KeyboardButton::Accept,
            0x1F => KeyboardButton::ModeChange,
            0x20 => KeyboardButton::Space,
            0x21 => KeyboardButton::Prior,
            0x22 => KeyboardButton::Next,
            0x23 => KeyboardButton::End,
            0x24 => KeyboardButton::Home,
            0x25 => KeyboardButton::Left,
            0x26 => KeyboardButton::Up,
            0x27 => KeyboardButton::Right,
            0x28 => KeyboardButton::Down,
            0x29 => KeyboardButton::Select,
            0x2A => KeyboardButton::Print,
            0x2B => KeyboardButton::Execute,
            0x2C => KeyboardButton::Snapshot,
            0x2D => KeyboardButton::Insert,
            0x2E => KeyboardButton::Delete,
            0x2F => KeyboardButton::Help,
            0x5B => KeyboardButton::LWin,
            0x5C => KeyboardButton::RWin,
            0x5D => KeyboardButton::Apps,
            0x5F => KeyboardButton::Sleep,
            0x60 => KeyboardButton::Numpad0,
            0x61 => KeyboardButton::Numpad1,
            0x62 => KeyboardButton::Numpad2,
            0x63 => KeyboardButton::Numpad3,
            0x64 => KeyboardButton::Numpad4,
            0x65 => KeyboardButton::Numpad5,
            0x66 => KeyboardButton::Numpad6,
            0x67 => KeyboardButton::Numpad7,
            0x68 => KeyboardButton::Numpad8,
            0x69 => KeyboardButton::Numpad9,
            0x6A => KeyboardButton::Multiply,
            0x6B => KeyboardButton::Add,
            0x6C => KeyboardButton::Separator,
            0x6D => KeyboardButton::Substract,
            0x6E => KeyboardButton::Decimal,
            0x6F => KeyboardButton::Divide,
            0x70 => KeyboardButton::F1,
            0x71 => KeyboardButton::F2,
            0x72 => KeyboardButton::F3,
            0x73 => KeyboardButton::F4,
            0x74 => KeyboardButton::F5,
            0x75 => KeyboardButton::F6,
            0x76 => KeyboardButton::F7,
            0x77 => KeyboardButton::F8,
            0x78 => KeyboardButton::F9,
            0x79 => KeyboardButton::F10,
            0x7A => KeyboardButton::F11,
            0x7B => KeyboardButton::F12,
            0x7C => KeyboardButton::F13,
            0x7D => KeyboardButton::F14,
            0x7E => KeyboardButton::F15,
            0x7F => KeyboardButton::F16,
            0x80 => KeyboardButton::F17,
            0x81 => KeyboardButton::F18,
            0x82 => KeyboardButton::F19,
            0x83 => KeyboardButton::F20,
            0x84 => KeyboardButton::F21,
            0x85 => KeyboardButton::F22,
            0x86 => KeyboardButton::F23,
            0x87 => KeyboardButton::F24,
            0x88 => KeyboardButton::NavigationView,
            0x89 => KeyboardButton::NavigationMenu,
            0x8A => KeyboardButton::NavigationUp,
            0x8B => KeyboardButton::NavigationDown,
            0x8C => KeyboardButton::NavigationLeft,
            0x8D => KeyboardButton::NavigationRight,
            0x8E => KeyboardButton::NavigationAccept,
            0x8F => KeyboardButton::NavigationCancel,
            0x90 => KeyboardButton::NumLock,
            0x91 => KeyboardButton::Scroll,
            0x92 => KeyboardButton::OemNecEqualOrOemFjJisho,
            0x93 => KeyboardButton::OemFjMasshou,
            0x94 => KeyboardButton::OemFjTouroku,
            0x95 => KeyboardButton::OemFjLoya,
            0x96 => KeyboardButton::OemFjRoya,
            0xA0 => KeyboardButton::LShift,
            0xA1 => KeyboardButton::RShift,
            0xA2 => KeyboardButton::LControl,
            0xA3 => KeyboardButton::RControl,
            0xA4 => KeyboardButton::LMenu,
            0xA5 => KeyboardButton::RMenu,
            0xA6 => KeyboardButton::BrowserBack,
            0xA7 => KeyboardButton::BrowserForward,
            0xA8 => KeyboardButton::BrowserRefresh,
            0xA9 => KeyboardButton::BrowserStop,
            0xAA => KeyboardButton::BrowserSearch,
            0xAB => KeyboardButton::BrowserFavorites,
            0xAC => KeyboardButton::BrowserHome,
            0xAD => KeyboardButton::VolumeMute,
            0xAE => KeyboardButton::VolumeDown,
            0xAF => KeyboardButton::VolumeUp,
            0xB0 => KeyboardButton::MediaNextTrack,
            0xB1 => KeyboardButton::MediaPrevTrack,
            0xB2 => KeyboardButton::MediaStop,
            0xB3 => KeyboardButton::MediaPlayPause,
            0xB4 => KeyboardButton::LaunchMail,
            0xB5 => KeyboardButton::LaunchMediaSelect,
            0xB6 => KeyboardButton::LaunchApp1,
            0xB7 => KeyboardButton::LaunchApp2,
            0xBA => KeyboardButton::Oem1,
            0xBB => KeyboardButton::OemPlus,
            0xBC => KeyboardButton::OemComma,
            0xBD => KeyboardButton::OemMinus,
            0xBE => KeyboardButton::OemPeriod,
            0xBF => KeyboardButton::Oem2,
            0xC0 => KeyboardButton::Oem3,
            0xC3 => KeyboardButton::GamepadA,
            0xC4 => KeyboardButton::GamepadB,
            0xC5 => KeyboardButton::GamepadX,
            0xC6 => KeyboardButton::GamepadY,
            0xC7 => KeyboardButton::GamepadRightShoulder,
            0xC8 => KeyboardButton::GamepadLeftShoulder,
            0xC9 => KeyboardButton::GamepadLeftTrigger,
            0xCA => KeyboardButton::GamepadRightTrigger,
            0xCB => KeyboardButton::GamepadDpadUp,
            0xCC => KeyboardButton::GamepadDpadDown,
            0xCD => KeyboardButton::GamepadDpadLeft,
            0xCE => KeyboardButton::GamepadDpadRight,
            0xCF => KeyboardButton::GamepadMenu,
            0xD0 => KeyboardButton::GamepadView,
            0xD1 => KeyboardButton::GamepadLeftThumbstickButton,
            0xD2 => KeyboardButton::GamepadRightThumbstickButton,
            0xD3 => KeyboardButton::GamepadLeftThumbstickUp,
            0xD4 => KeyboardButton::GamepadLeftThumbstickDown,
            0xD5 => KeyboardButton::GamepadLeftThumbstickRight,
            0xD6 => KeyboardButton::GamepadLeftThumbstickLeft,
            0xD7 => KeyboardButton::GamepadRightThumbstickUp,
            0xD8 => KeyboardButton::GamepadRightThumbstickDown,
            0xD9 => KeyboardButton::GamepadRightThumbstickRight,
            0xDA => KeyboardButton::GamepadRightThumbstickLeft,
            0xDB => KeyboardButton::Oem4,
            0xDC => KeyboardButton::Oem5,
            0xDD => KeyboardButton::Oem6,
            0xDE => KeyboardButton::Oem7,
            0xDF => KeyboardButton::Oem8,
            0xE1 => KeyboardButton::OemAx,
            0xE2 => KeyboardButton::Oem102,
            0xE3 => KeyboardButton::IcoHelp,
            0xE4 => KeyboardButton::Ico00,
            0xE5 => KeyboardButton::ProcessKey,
            0xE6 => KeyboardButton::IcoClear,
            0xE7 => KeyboardButton::Packet,
            0xE9 => KeyboardButton::OemReset,
            0xEA => KeyboardButton::OemJump,
            0xEB => KeyboardButton::OemPa1,
            0xEC => KeyboardButton::OemPa2,
            0xED => KeyboardButton::OemPa3,
            0xEE => KeyboardButton::OemWsctrl,
            0xEF => KeyboardButton::OemCusel,
            0xF0 => KeyboardButton::OemAttn,
            0xF1 => KeyboardButton::OemFinish,
            0xF2 => KeyboardButton::OemCopy,
            0xF3 => KeyboardButton::OemAuto,
            0xF4 => KeyboardButton::OemEnlw,
            0xF5 => KeyboardButton::OemBacktab,
            0xF6 => KeyboardButton::Attn,
            0xF7 => KeyboardButton::Crsel,
            0xF8 => KeyboardButton::Exsel,
            0xF9 => KeyboardButton::Ereof,
            0xFA => KeyboardButton::Play,
            0xFB => KeyboardButton::Zoom,
            0xFC => KeyboardButton::Noname,
            0xFD => KeyboardButton::Pa1,
            0xFE => KeyboardButton::OemClear,
            _ => KeyboardButton::Undefined,
        }
    }
}

impl PartialEq<u32> for KeyboardButton {
    fn eq(&self, other: &u32) -> bool {
        *self as u32 == *other
    }
}
