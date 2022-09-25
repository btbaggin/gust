use std::collections::HashMap;
use std::str::FromStr;
use std::cell::RefCell;
use crate::V2;
use crate::logger::PanicLogEntry;
use crate::settings::{SettingsFile, SettingNames};

macro_rules! keys_enum {
    (pub enum $name:ident {
        $($value:ident = $default:expr,)+
    }) => {
        #[repr(u8)]
        #[derive(Clone, Copy)]
        #[allow(dead_code)]
        pub enum $name {
            $($value = $default,)+
        }
        impl std::str::FromStr for $name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($value) => Ok($name::$value),)+
                    _ => Err(()),
                }
            }
        }
    };
}

keys_enum! {
    pub enum Key {
        MouseLeft = 0x01,
        MouseRight = 0x02,
        MouseMiddle = 0x04,

        Backspace = 0x08,
        Tab = 0x09,
        Clear = 0x0C,
        Enter = 0x0D,
        Shift = 0x10,
        Control = 0x11,
        Alt = 0x12,
        Pause = 0x13,
        CapsLock = 0x14,
        Escape = 0x1B,
        Space = 0x20,
        PageUp = 0x21,
        PageDown = 0x22,
        End = 0x23,
        Home = 0x24,
        Left = 0x25,
        Up = 0x26,
        Right = 0x27,
        Down = 0x28,
        Print = 0x2A,
        PrintScreen = 0x2C,
        Insert = 0x2D,
        Delete = 0x2E,
        Help = 0x2F,

        Zero = 0x30,
        One = 0x31,
        Two = 0x32,
        Three = 0x33,
        Four = 0x34,
        Five = 0x35,
        Six = 0x36,
        Seven = 0x37,
        Eight = 0x38,
        Nine = 0x39,

        A = 0x41,
        B = 0x42,
        C = 0x43,
        D = 0x44,
        E = 0x45,
        F = 0x46,
        G = 0x47,
        H = 0x48,
        I = 0x49,
        J = 0x4A,
        K = 0x4B,
        L = 0x4C,
        M = 0x4D,
        N = 0x4E,
        O = 0x4F,
        P = 0x50,
        Q = 0x51,
        R = 0x52,
        S = 0x53,
        T = 0x54,
        U = 0x55,
        V = 0x56,
        W = 0x57,
        X = 0x58,
        Y = 0x59,
        Z = 0x5A,

        Num0 = 0x60,
        Num1 = 0x61,
        Num2 = 0x62,
        Num3 = 0x63,
        Num4 = 0x64,
        Num5 = 0x65,
        Num6 = 0x66,
        Num7 = 0x67,
        Num8 = 0x68,
        Num9 = 0x69,

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

        LShift = 0xA0,
        RShift = 0xA1,
        LControl = 0xA2,
        RControl = 0xA3,

        Semicolon = 0xBA,
        Plus = 0xBB,
        Comma = 0xBC,
        Minus = 0xBD,
        Period = 0xBE,
        Slash = 0xBF,
        Tilde = 0xC0,
        LBracket = 0xDB,
        Backslash = 0xDC,
        RBracket = 0xDD,
        Quote = 0xDE,
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Actions {
    Up,
    Down,
    Left,
    Right,
    Accept,
    Cancel,
    Select,
    Slower,
    Faster,
}

const U64_BITS: usize = std::mem::size_of::<u64>() * 8;
const CONSUME_SIZE: usize = 256 / U64_BITS;
pub struct Input {
    consumed_input: RefCell<[u64; CONSUME_SIZE]>,
    previous_input: [u8; 256],
    input: [u8; 256],
    map: HashMap<Actions, Key>,
    mouse_position: V2,
}
#[allow(dead_code)]
impl Input {
    pub fn new() -> Input {
        Input {
            consumed_input: RefCell::new([0; CONSUME_SIZE]),
            previous_input: [0; 256],
            input: [0; 256],
            map: HashMap::new(),
            mouse_position: V2::new(0., 0.),
        }
    }
    fn key_down(input: &[u8; 256], consumed: &RefCell<[u64; CONSUME_SIZE]>, key: Key) -> bool {
        if Self::is_consumed(consumed, key) { return false; }
        input[key as usize] & 0x80 != 0
    }
    fn key_up(input: &[u8; 256], consumed: &RefCell<[u64; CONSUME_SIZE]>, key: Key) -> bool {
        if Self::is_consumed(consumed, key) { return false; }
        input[key as usize] & 0x80 == 0
    }
    fn consume_input(input: &RefCell<[u64; CONSUME_SIZE]>, key: Key) {
        let mut input = input.borrow_mut();
        let index = key as usize / U64_BITS;
        let bit = key as usize % U64_BITS;

        input[index] |= 1 << bit;
    }
    fn is_consumed(input: &RefCell<[u64; CONSUME_SIZE]>, key: Key) -> bool {
        let input = input.borrow();
        let index = key as usize / U64_BITS;
        let bit = key as usize % U64_BITS;

        input[index] & 1 << bit != 0
    }

    pub fn consume_action(&self, action: Actions) {
        let key = self.map.get(&action).expect("All actions should be in input map");
        Self::consume_input(&self.consumed_input, *key)
    }
    pub fn action_down(&self, action: Actions) -> bool {
        let key = self.map.get(&action).expect("All actions should be in input map");
        Self::key_down(&self.input, &self.consumed_input, *key)
    }
    pub fn action_up(&self, action: Actions) -> bool {
        let key = self.map.get(&action).expect("All actions should be in input map");
        Self::key_up(&self.input, &self.consumed_input, *key)
    }
    pub fn action_pressed(&self, action: Actions) -> bool {
        let key = self.map.get(&action).expect("All actions should be in input map");
        Self::key_up(&self.previous_input, &self.consumed_input, *key) &&
            Self::key_down(&self.input, &self.consumed_input, *key)
    }
    pub fn action_released(&self, action: Actions) -> bool {
        let key = self.map.get(&action).expect("All actions should be in input map");
        Self::key_down(&self.previous_input, &self.consumed_input, *key) &&
            Self::key_up(&self.input, &self.consumed_input, *key)
    }
    pub fn mouse_pos(&self) -> V2 {
        self.mouse_position
    }
}

pub fn gather(input: &mut Input, position: V2) {
    input.previous_input = input.input;
    input.mouse_position = position;
    let mut consumed = input.consumed_input.borrow_mut();
    for i in consumed.iter_mut() {
        *i = 0;
    }

    #[cfg(target_os = "windows")]
    {
        use winapi::um::winuser::GetKeyboardState;
        unsafe { GetKeyboardState(&mut input.input as *mut u8); }
    }
}

pub fn load_input_settings(input: &mut Input, settings: &SettingsFile) {
    fn add_action_from_settings(input: &mut Input, settings: &SettingsFile, setting: SettingNames, action: Actions) {
        let setting = settings.get_str(setting);
        let key = Key::from_str(&setting).log_and_panic();
        input.map.insert(action, key);
    }

    add_action_from_settings(input, settings, SettingNames::ActionLeft, Actions::Left);
    add_action_from_settings(input, settings, SettingNames::ActionRight, Actions::Right);
    add_action_from_settings(input, settings, SettingNames::ActionUp, Actions::Up);
    add_action_from_settings(input, settings, SettingNames::ActionDown, Actions::Down);
    add_action_from_settings(input, settings, SettingNames::ActionCancel, Actions::Cancel);
    input.map.insert(Actions::Slower, Key::Minus);
    input.map.insert(Actions::Faster, Key::Plus);
    input.map.insert(Actions::Accept, Key::Enter);
    input.map.insert(Actions::Select, Key::MouseLeft);
}
