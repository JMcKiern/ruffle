use ruffle_core::backend::input::{InputBackend, MouseCursor};
use ruffle_core::events::KeyCode;
use ruffle_web_common::JsResult;
use std::collections::HashSet;
use web_sys::{HtmlCanvasElement, KeyboardEvent};

/// An implementation of `InputBackend` utilizing `web_sys` bindings to input
/// APIs
pub struct WebInputBackend {
    keys_down: HashSet<String>,
    canvas: HtmlCanvasElement,
    cursor_visible: bool,
    cursor: MouseCursor,
    last_key: KeyCode,
    last_char: Option<char>,
    last_ascii: Option<u8>,
}

impl WebInputBackend {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        Self {
            keys_down: HashSet::new(),
            canvas: canvas.clone(),
            cursor_visible: true,
            cursor: MouseCursor::Arrow,
            last_key: KeyCode::Unknown,
            last_char: None,
            last_ascii: None,
        }
    }

    /// Register a key press for a given code string.
    pub fn keydown(&mut self, event: &KeyboardEvent) {
        let code = event.code();
        self.last_key = web_to_ruffle_key_code(&code).unwrap_or(KeyCode::Unknown);
        self.keys_down.insert(code);
        self.last_char = web_key_to_codepoint(&event.key());
        self.last_ascii = web_key_to_ascii(&event.key());
    }

    /// Register a key release for a given code string.
    pub fn keyup(&mut self, event: &KeyboardEvent) {
        let code = event.code();
        self.last_key = web_to_ruffle_key_code(&code).unwrap_or(KeyCode::Unknown);
        self.keys_down.remove(&code);
        self.last_char = web_key_to_codepoint(&event.key());
        self.last_ascii = web_key_to_ascii(&event.key());
    }

    fn update_mouse_cursor(&self) {
        let cursor = if self.cursor_visible {
            match self.cursor {
                MouseCursor::Arrow => "auto",
                MouseCursor::Hand => "pointer",
                MouseCursor::IBeam => "text",
                MouseCursor::Grab => "grab",
            }
        } else {
            "none"
        };
        self.canvas
            .style()
            .set_property("cursor", cursor)
            .warn_on_error();
    }
}

impl InputBackend for WebInputBackend {
    fn is_key_down(&self, key: KeyCode) -> bool {
        match key {
            KeyCode::Unknown => false,
            KeyCode::Backspace => self.keys_down.contains("Backspace"),
            KeyCode::Return => self.keys_down.contains("Enter"),
            KeyCode::Shift => {
                self.keys_down.contains("ShiftLeft") || self.keys_down.contains("ShiftRight")
            }
            KeyCode::Control => {
                self.keys_down.contains("ControlLeft") || self.keys_down.contains("ControlRight")
            }
            KeyCode::Alt => {
                self.keys_down.contains("AltLeft") || self.keys_down.contains("AltRight")
            }
            KeyCode::CapsLock => self.keys_down.contains("CapsLock"),
            KeyCode::Escape => self.keys_down.contains("Escape"),
            KeyCode::Space => self.keys_down.contains("Space"),
            KeyCode::Key0 => self.keys_down.contains("Digit0"),
            KeyCode::Key1 => self.keys_down.contains("Digit1"),
            KeyCode::Key2 => self.keys_down.contains("Digit2"),
            KeyCode::Key3 => self.keys_down.contains("Digit3"),
            KeyCode::Key4 => self.keys_down.contains("Digit4"),
            KeyCode::Key5 => self.keys_down.contains("Digit5"),
            KeyCode::Key6 => self.keys_down.contains("Digit6"),
            KeyCode::Key7 => self.keys_down.contains("Digit7"),
            KeyCode::Key8 => self.keys_down.contains("Digit8"),
            KeyCode::Key9 => self.keys_down.contains("Digit9"),
            KeyCode::A => self.keys_down.contains("KeyA"),
            KeyCode::B => self.keys_down.contains("KeyB"),
            KeyCode::C => self.keys_down.contains("KeyC"),
            KeyCode::D => self.keys_down.contains("KeyD"),
            KeyCode::E => self.keys_down.contains("KeyE"),
            KeyCode::F => self.keys_down.contains("KeyF"),
            KeyCode::G => self.keys_down.contains("KeyG"),
            KeyCode::H => self.keys_down.contains("KeyH"),
            KeyCode::I => self.keys_down.contains("KeyI"),
            KeyCode::J => self.keys_down.contains("KeyJ"),
            KeyCode::K => self.keys_down.contains("KeyK"),
            KeyCode::L => self.keys_down.contains("KeyL"),
            KeyCode::M => self.keys_down.contains("KeyM"),
            KeyCode::N => self.keys_down.contains("KeyN"),
            KeyCode::O => self.keys_down.contains("KeyO"),
            KeyCode::P => self.keys_down.contains("KeyP"),
            KeyCode::Q => self.keys_down.contains("KeyQ"),
            KeyCode::R => self.keys_down.contains("KeyR"),
            KeyCode::S => self.keys_down.contains("KeyS"),
            KeyCode::T => self.keys_down.contains("KeyT"),
            KeyCode::U => self.keys_down.contains("KeyU"),
            KeyCode::V => self.keys_down.contains("KeyV"),
            KeyCode::W => self.keys_down.contains("KeyW"),
            KeyCode::X => self.keys_down.contains("KeyX"),
            KeyCode::Y => self.keys_down.contains("KeyY"),
            KeyCode::Z => self.keys_down.contains("KeyZ"),
            KeyCode::Semicolon => self.keys_down.contains("Semicolon"),
            KeyCode::Equals => self.keys_down.contains("Equal"),
            KeyCode::Comma => self.keys_down.contains("Comma"),
            KeyCode::Minus => self.keys_down.contains("Minus"),
            KeyCode::Period => self.keys_down.contains("Period"),
            KeyCode::Slash => self.keys_down.contains("Slash"),
            KeyCode::Grave => self.keys_down.contains("Backquote"),
            KeyCode::LBracket => self.keys_down.contains("BracketLeft"),
            KeyCode::Backslash => self.keys_down.contains("Backslash"),
            KeyCode::RBracket => self.keys_down.contains("BracketRight"),
            KeyCode::Apostrophe => self.keys_down.contains("Quote"),
            KeyCode::Numpad0 => self.keys_down.contains("Numpad0"),
            KeyCode::Numpad1 => self.keys_down.contains("Numpad1"),
            KeyCode::Numpad2 => self.keys_down.contains("Numpad2"),
            KeyCode::Numpad3 => self.keys_down.contains("Numpad3"),
            KeyCode::Numpad4 => self.keys_down.contains("Numpad4"),
            KeyCode::Numpad5 => self.keys_down.contains("Numpad5"),
            KeyCode::Numpad6 => self.keys_down.contains("Numpad6"),
            KeyCode::Numpad7 => self.keys_down.contains("Numpad7"),
            KeyCode::Numpad8 => self.keys_down.contains("Numpad8"),
            KeyCode::Numpad9 => self.keys_down.contains("Numpad9"),
            KeyCode::Multiply => self.keys_down.contains("NumpadMultiply"),
            KeyCode::Plus => self.keys_down.contains("NumpadAdd"),
            KeyCode::NumpadMinus => self.keys_down.contains("NumpadSubtract"),
            KeyCode::NumpadPeriod => self.keys_down.contains("NumpadDecimal"),
            KeyCode::NumpadSlash => self.keys_down.contains("NumpadDivide"),
            KeyCode::PgUp => self.keys_down.contains("PageUp"),
            KeyCode::PgDown => self.keys_down.contains("PageDown"),
            KeyCode::End => self.keys_down.contains("End"),
            KeyCode::Home => self.keys_down.contains("Home"),
            KeyCode::Left => self.keys_down.contains("ArrowLeft"),
            KeyCode::Up => self.keys_down.contains("ArrowUp"),
            KeyCode::Right => self.keys_down.contains("ArrowRight"),
            KeyCode::Down => self.keys_down.contains("ArrowDown"),
            KeyCode::Insert => self.keys_down.contains("Insert"),
            KeyCode::Delete => self.keys_down.contains("Delete"),
            KeyCode::Pause => self.keys_down.contains("Pause"),
            KeyCode::ScrollLock => self.keys_down.contains("ScrollLock"),
            KeyCode::F1 => self.keys_down.contains("F1"),
            KeyCode::F2 => self.keys_down.contains("F2"),
            KeyCode::F3 => self.keys_down.contains("F3"),
            KeyCode::F4 => self.keys_down.contains("F4"),
            KeyCode::F5 => self.keys_down.contains("F5"),
            KeyCode::F6 => self.keys_down.contains("F6"),
            KeyCode::F7 => self.keys_down.contains("F7"),
            KeyCode::F8 => self.keys_down.contains("F8"),
            KeyCode::F9 => self.keys_down.contains("F9"),
            KeyCode::F10 => self.keys_down.contains("F10"),
            KeyCode::F11 => self.keys_down.contains("F11"),
            KeyCode::F12 => self.keys_down.contains("F12"),
        }
    }

    fn last_key_code(&self) -> KeyCode {
        self.last_key
    }

    fn last_key_char(&self) -> Option<char> {
        self.last_char
    }

    fn last_key_ascii(&self) -> Option<u8> {
        self.last_ascii
    }

    fn mouse_visible(&self) -> bool {
        self.cursor_visible
    }

    fn hide_mouse(&mut self) {
        self.cursor_visible = false;
        self.update_mouse_cursor();
    }

    fn show_mouse(&mut self) {
        self.cursor_visible = true;
        self.update_mouse_cursor();
    }

    fn set_mouse_cursor(&mut self, cursor: MouseCursor) {
        self.cursor = cursor;
        self.update_mouse_cursor();
    }

    fn set_clipboard_content(&mut self, _content: String) {
        log::warn!("set clipboard not implemented");
    }
}

/// Converts a Web `KeyboardEvent.code` value into a Ruffle `KeyCode`.
/// Returns `None` if there is no matching Flash key key.
pub fn web_to_ruffle_key_code(key_code: &str) -> Option<KeyCode> {
    let out = match key_code {
        "Backspace" => KeyCode::Backspace,
        "Enter" => KeyCode::Return,
        "ShiftLeft" | "ShiftRight" => KeyCode::Shift,
        "ControlLeft" | "ControlRight" => KeyCode::Control,
        "AltLeft" | "AltRight" => KeyCode::Alt,
        "CapsLock" => KeyCode::CapsLock,
        "Escape" => KeyCode::Escape,
        "Space" => KeyCode::Space,
        "Digit0" => KeyCode::Key0,
        "Digit1" => KeyCode::Key1,
        "Digit2" => KeyCode::Key2,
        "Digit3" => KeyCode::Key3,
        "Digit4" => KeyCode::Key4,
        "Digit5" => KeyCode::Key5,
        "Digit6" => KeyCode::Key6,
        "Digit7" => KeyCode::Key7,
        "Digit8" => KeyCode::Key8,
        "Digit9" => KeyCode::Key9,
        "KeyA" => KeyCode::A,
        "KeyB" => KeyCode::B,
        "KeyC" => KeyCode::C,
        "KeyD" => KeyCode::D,
        "KeyE" => KeyCode::E,
        "KeyF" => KeyCode::F,
        "KeyG" => KeyCode::G,
        "KeyH" => KeyCode::H,
        "KeyI" => KeyCode::I,
        "KeyJ" => KeyCode::J,
        "KeyK" => KeyCode::K,
        "KeyL" => KeyCode::L,
        "KeyM" => KeyCode::M,
        "KeyN" => KeyCode::N,
        "KeyO" => KeyCode::O,
        "KeyP" => KeyCode::P,
        "KeyQ" => KeyCode::Q,
        "KeyR" => KeyCode::R,
        "KeyS" => KeyCode::S,
        "KeyT" => KeyCode::T,
        "KeyU" => KeyCode::U,
        "KeyV" => KeyCode::V,
        "KeyW" => KeyCode::W,
        "KeyX" => KeyCode::X,
        "KeyY" => KeyCode::Y,
        "KeyZ" => KeyCode::Z,
        "Semicolon" => KeyCode::Semicolon,
        "Equal" => KeyCode::Equals,
        "Comma" => KeyCode::Comma,
        "Minus" => KeyCode::Minus,
        "Period" => KeyCode::Period,
        "Slash" => KeyCode::Slash,
        "Backquote" => KeyCode::Grave,
        "BracketLeft" => KeyCode::LBracket,
        "Backslash" => KeyCode::Backslash,
        "BracketRight" => KeyCode::RBracket,
        "Quote" => KeyCode::Apostrophe,
        "Numpad0" => KeyCode::Numpad0,
        "Numpad1" => KeyCode::Numpad1,
        "Numpad2" => KeyCode::Numpad2,
        "Numpad3" => KeyCode::Numpad3,
        "Numpad4" => KeyCode::Numpad4,
        "Numpad5" => KeyCode::Numpad5,
        "Numpad6" => KeyCode::Numpad6,
        "Numpad7" => KeyCode::Numpad7,
        "Numpad8" => KeyCode::Numpad8,
        "Numpad9" => KeyCode::Numpad9,
        "NumpadMultiply" => KeyCode::Multiply,
        "NumpadAdd" => KeyCode::Plus,
        "NumpadSubtract" => KeyCode::NumpadMinus,
        "NumpadDecimal" => KeyCode::NumpadPeriod,
        "NumpadDivide" => KeyCode::NumpadSlash,
        "PageUp" => KeyCode::PgUp,
        "PageDown" => KeyCode::PgDown,
        "End" => KeyCode::End,
        "Home" => KeyCode::Home,
        "ArrowLeft" => KeyCode::Left,
        "ArrowUp" => KeyCode::Up,
        "ArrowRight" => KeyCode::Right,
        "ArrowDown" => KeyCode::Down,
        "Insert" => KeyCode::Insert,
        "Delete" => KeyCode::Delete,
        "Pause" => KeyCode::Pause,
        "ScrollLock" => KeyCode::ScrollLock,
        "F1" => KeyCode::F1,
        "F2" => KeyCode::F2,
        "F3" => KeyCode::F3,
        "F4" => KeyCode::F4,
        "F5" => KeyCode::F5,
        "F6" => KeyCode::F6,
        "F7" => KeyCode::F7,
        "F8" => KeyCode::F8,
        "F9" => KeyCode::F9,
        "F10" => KeyCode::F10,
        "F11" => KeyCode::F11,
        "F12" => KeyCode::F12,
        _ => return None,
    };
    Some(out)
}

/// Converts a Web `KeyboardEvent.key` value into a character codepoint.
/// Returns `None` if they input was not a printable character.
pub fn web_key_to_codepoint(key: &str) -> Option<char> {
    // TODO: This is a very cheesy way to tell if a KeyboardEvent.key is a printable character.
    // Single character strings will be an actual printable char that we can use as text input.
    // All the other special values are multiple characters (e.g. "ArrowLeft").
    // It's probably better to explicitly match on all the variants.
    let mut chars = key.chars();
    let (c1, c2) = (chars.next(), chars.next());
    if c2.is_none() {
        // Single character.
        c1
    } else {
        // Check for special characters.
        match key {
            "Backspace" => Some(8 as char),
            "Delete" => Some(127 as char),
            _ => None,
        }
    }
}


// TODO: check if this is platform dependent (probably is) and actionscript dependent
// Looks like Windows-1252 in my (@jmckiern) testing
pub fn web_key_to_ascii(key: &str) -> Option<u8> {
    let ascii = match key {
        // TODO: Verify the below
        //NUL 0,
        //SOH 1,
        //STX 2,
        //ETX 3,
        //EOT 4,
        //ENQ 5,
        //ACK 6,
        //BEL 7,
        "Backspace" => 8,
        "Tab" => 9,
        //LF 10,
        //VT 11,
        //FF 12,
        //CR 13,
        //SO 14,
        //SI 15,
        //DLE 16,
        //DC1 17,
        //DC2 18,
        //DC3 19,
        //DC4 20,
        //NAK 21,
        //SYN 22,
        //ETB 23,
        //CAN 24,
        //EM 25,
        //SUB 26,
        "Escape" => 27,
        //FS 28,
        //GS 29,
        //RS 30,
        //US 31,
        //SP 32,
        "!" => 33,
        "\"" => 34,
        "#" => 35,
        "$" => 36,
        "%" => 37,
        "&" => 38,
        "'" => 39,
        "(" => 40,
        ")" => 41,
        "*" => 42,
        "+" => 43,
        "," => 44,
        "-" => 45,
        "." => 46,
        "/" => 47,
        "0" => 48,
        "1" => 49,
        "2" => 50,
        "3" => 51,
        "4" => 52,
        "5" => 53,
        "6" => 54,
        "7" => 55,
        "8" => 56,
        "9" => 57,
        ":" => 58,
        ";" => 59,
        "<" => 60,
        "=" => 61,
        ">" => 62,
        "?" => 63,
        "@" => 64,
        "A" => 65,
        "B" => 66,
        "C" => 67,
        "D" => 68,
        "E" => 69,
        "F" => 70,
        "G" => 71,
        "H" => 72,
        "I" => 73,
        "J" => 74,
        "K" => 75,
        "L" => 76,
        "M" => 77,
        "N" => 78,
        "O" => 79,
        "P" => 80,
        "Q" => 81,
        "R" => 82,
        "S" => 83,
        "T" => 84,
        "U" => 85,
        "V" => 86,
        "W" => 87,
        "X" => 88,
        "Y" => 89,
        "Z" => 90,
        "[" => 91,
        "\\" => 92,
        "]" => 93,
        "^" => 94,
        "_" => 95,
        "`" => 96,
        "a" => 97,
        "b" => 98,
        "c" => 99,
        "d" => 100,
        "e" => 101,
        "f" => 102,
        "g" => 103,
        "h" => 104,
        "i" => 105,
        "j" => 106,
        "k" => 107,
        "l" => 108,
        "m" => 109,
        "n" => 110,
        "o" => 111,
        "p" => 112,
        "q" => 113,
        "r" => 114,
        "s" => 115,
        "t" => 116,
        "u" => 117,
        "v" => 118,
        "w" => 119,
        "x" => 120,
        "y" => 121,
        "z" => 122,
        "{" => 123,
        "|" => 124,
        "}" => 125,
        "~" => 126,
        "Delete" => 127,
        "€" => 128,
        "‚" => 130,
        "ƒ" => 131,
        "„" => 132,
        "…" => 133,
        "†" => 134,
        "‡" => 135,
        "ˆ" => 136,
        "‰" => 137,
        "Š" => 138,
        "‹" => 139,
        "Œ" => 140,
        "Ž" => 142,
        "‘" => 145,
        "’" => 146,
        "“" => 147,
        "”" => 148,
        "•" => 149,
        "–" => 150,
        "—" => 151,
        "˜" => 152,
        "™" => 153,
        "š" => 154,
        "›" => 155,
        "œ" => 156,
        "ž" => 158,
        "Ÿ" => 159,
        //NBSP 160,
        "¡" => 161,
        "¢" => 162,
        "£" => 163,
        "¤" => 164,
        "¥" => 165,
        "¦" => 166,
        "§" => 167,
        "¨" => 168,
        "©" => 169,
        "ª" => 170,
        "«" => 171,
        "¬" => 172,
        //SHY 173,
        "®" => 174,
        "¯" => 175,
        "°" => 176,
        "±" => 177,
        "²" => 178,
        "³" => 179,
        "´" => 180,
        "µ" => 181,
        "¶" => 182,
        "·" => 183,
        "¸" => 184,
        "¹" => 185,
        "º" => 186,
        "»" => 187,
        "¼" => 188,
        "½" => 189,
        "¾" => 190,
        "¿" => 191,
        "À" => 192,
        "Á" => 193,
        "Â" => 194,
        "Ã" => 195,
        "Ä" => 196,
        "Å" => 197,
        "Æ" => 198,
        "Ç" => 199,
        "È" => 200,
        "É" => 201,
        "Ê" => 202,
        "Ë" => 203,
        "Ì" => 204,
        "Í" => 205,
        "Î" => 206,
        "Ï" => 207,
        "Ð" => 208,
        "Ñ" => 209,
        "Ò" => 210,
        "Ó" => 211,
        "Ô" => 212,
        "Õ" => 213,
        "Ö" => 214,
        "×" => 215,
        "Ø" => 216,
        "Ù" => 217,
        "Ú" => 218,
        "Û" => 219,
        "Ü" => 220,
        "Ý" => 221,
        "Þ" => 222,
        "ß" => 223,
        "à" => 224,
        "á" => 225,
        "â" => 226,
        "ã" => 227,
        "ä" => 228,
        "å" => 229,
        "æ" => 230,
        "ç" => 231,
        "è" => 232,
        "é" => 233,
        "ê" => 234,
        "ë" => 235,
        "ì" => 236,
        "í" => 237,
        "î" => 238,
        "ï" => 239,
        "ð" => 240,
        "ñ" => 241,
        "ò" => 242,
        "ó" => 243,
        "ô" => 244,
        "õ" => 245,
        "ö" => 246,
        "÷" => 247,
        "ø" => 248,
        "ù" => 249,
        "ú" => 250,
        "û" => 251,
        "ü" => 252,
        "ý" => 253,
        "þ" => 254,
        "ÿ" => 255,
        _ => 63, // '?' if not recognised
    };
    Some(ascii) // TODO: Option<u8> -> u8   ?
}
