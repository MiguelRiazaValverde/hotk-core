#![allow(dead_code)]

use std::str::FromStr;

use global_hotkey::hotkey::Modifiers;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::{Display, EnumString};

/**
 * Represents keyboard key codes used to define hotkeys.
 * Each variant corresponds to a specific physical key.
 */
#[napi(string_enum)]
#[derive(Debug, Display, EnumString, EnumIter)]
pub enum KeyCode {
  Backquote,
  Backslash,
  BracketLeft,
  BracketRight,
  Comma,
  Digit0,
  Digit1,
  Digit2,
  Digit3,
  Digit4,
  Digit5,
  Digit6,
  Digit7,
  Digit8,
  Digit9,
  Equal,
  IntlBackslash,
  IntlRo,
  IntlYen,
  KeyA,
  KeyB,
  KeyC,
  KeyD,
  KeyE,
  KeyF,
  KeyG,
  KeyH,
  KeyI,
  KeyJ,
  KeyK,
  KeyL,
  KeyM,
  KeyN,
  KeyO,
  KeyP,
  KeyQ,
  KeyR,
  KeyS,
  KeyT,
  KeyU,
  KeyV,
  KeyW,
  KeyX,
  KeyY,
  KeyZ,
  Minus,
  Period,
  Quote,
  Semicolon,
  Slash,
  AltLeft,
  AltRight,
  Backspace,
  CapsLock,
  ContextMenu,
  ControlLeft,
  ControlRight,
  Enter,
  MetaLeft,
  MetaRight,
  ShiftLeft,
  ShiftRight,
  Space,
  Tab,
  Convert,
  KanaMode,
  Lang1,
  Lang2,
  Lang3,
  Lang4,
  Lang5,
  NonConvert,
  Delete,
  End,
  Help,
  Home,
  Insert,
  PageDown,
  PageUp,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  NumLock,
  Numpad0,
  Numpad1,
  Numpad2,
  Numpad3,
  Numpad4,
  Numpad5,
  Numpad6,
  Numpad7,
  Numpad8,
  Numpad9,
  NumpadAdd,
  NumpadBackspace,
  NumpadClear,
  NumpadClearEntry,
  NumpadComma,
  NumpadDecimal,
  NumpadDivide,
  NumpadEnter,
  NumpadEqual,
  NumpadHash,
  NumpadMemoryAdd,
  NumpadMemoryClear,
  NumpadMemoryRecall,
  NumpadMemoryStore,
  NumpadMemorySubtract,
  NumpadMultiply,
  NumpadParenLeft,
  NumpadParenRight,
  NumpadStar,
  NumpadSubtract,
  Escape,
  Fn,
  FnLock,
  PrintScreen,
  ScrollLock,
  Pause,
  BrowserBack,
  BrowserFavorites,
  BrowserForward,
  BrowserHome,
  BrowserRefresh,
  BrowserSearch,
  BrowserStop,
  Eject,
  LaunchApp1,
  LaunchApp2,
  LaunchMail,
  MediaPlayPause,
  MediaSelect,
  MediaStop,
  MediaTrackNext,
  MediaTrackPrevious,
  Power,
  Sleep,
  AudioVolumeDown,
  AudioVolumeMute,
  AudioVolumeUp,
  WakeUp,
  Hyper,
  Super,
  Turbo,
  Abort,
  Resume,
  Suspend,
  Again,
  Copy,
  Cut,
  Find,
  Open,
  Paste,
  Props,
  Select,
  Undo,
  Hiragana,
  Katakana,
  Unidentified,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,
  F25,
  F26,
  F27,
  F28,
  F29,
  F30,
  F31,
  F32,
  F33,
  F34,
  F35,
  BrightnessDown,
  BrightnessUp,
  DisplayToggleIntExt,
  KeyboardLayoutSelect,
  LaunchAssistant,
  LaunchControlPanel,
  LaunchScreenSaver,
  MailForward,
  MailReply,
  MailSend,
  MediaFastForward,
  MediaPause,
  MediaPlay,
  MediaRecord,
  MediaRewind,
  MicrophoneMuteToggle,
  PrivacyScreenToggle,
  SelectTask,
  ShowAllWindows,
  ZoomToggle,
}

impl KeyCode {
  pub fn global_hotkeys(&self) -> global_hotkey::hotkey::Code {
    global_hotkey::hotkey::Code::from_str(&self.to_string()).unwrap()
  }

  pub fn from_global_hotkeys(code: global_hotkey::hotkey::Code) -> Option<KeyCode> {
    code.to_string().parse().ok()
  }
}

/**
 * Returns a list of all available key code names as strings.
 *
 * Useful for getting all possible key codes supported by the hotkey manager.
 *
 * @returns {string[]} An array of key code names.
 */
#[napi]
pub fn key_code_keys() -> Vec<String> {
  KeyCode::iter().map(|e| e.to_string()).collect()
}

/**
 * Converts a KeyCode enum variant to a human-readable string representation.
 *
 * This is useful for displaying key codes in a user-friendly format.
 *
 * @param {KeyCode} key_code - The key code to convert.
 * @returns {string | null} A human-readable string representing the key code, or null if no mapping exists.
 */
#[napi]
pub fn key_code_to_human(key_code: KeyCode) -> Option<String> {
  let c = match key_code {
    KeyCode::Digit0 => "0",
    KeyCode::Digit1 => "1",
    KeyCode::Digit2 => "2",
    KeyCode::Digit3 => "3",
    KeyCode::Digit4 => "4",
    KeyCode::Digit5 => "5",
    KeyCode::Digit6 => "6",
    KeyCode::Digit7 => "7",
    KeyCode::Digit8 => "8",
    KeyCode::Digit9 => "9",
    KeyCode::KeyA => "a",
    KeyCode::KeyB => "b",
    KeyCode::KeyC => "c",
    KeyCode::KeyD => "d",
    KeyCode::KeyE => "e",
    KeyCode::KeyF => "f",
    KeyCode::KeyG => "g",
    KeyCode::KeyH => "h",
    KeyCode::KeyI => "i",
    KeyCode::KeyJ => "j",
    KeyCode::KeyK => "k",
    KeyCode::KeyL => "l",
    KeyCode::KeyM => "m",
    KeyCode::KeyN => "n",
    KeyCode::KeyO => "o",
    KeyCode::KeyP => "p",
    KeyCode::KeyQ => "q",
    KeyCode::KeyR => "r",
    KeyCode::KeyS => "s",
    KeyCode::KeyT => "t",
    KeyCode::KeyU => "u",
    KeyCode::KeyV => "v",
    KeyCode::KeyW => "w",
    KeyCode::KeyX => "x",
    KeyCode::KeyY => "y",
    KeyCode::KeyZ => "z",
    KeyCode::Backquote => "`",
    KeyCode::Backslash => "\\",
    KeyCode::BracketLeft => "[",
    KeyCode::BracketRight => "]",
    KeyCode::Comma => ",",
    KeyCode::Equal => "=",
    KeyCode::Minus => "-",
    KeyCode::Period => ".",
    KeyCode::Quote => "\"",
    KeyCode::Semicolon => ";",
    KeyCode::Slash => "/",
    KeyCode::F1 => "f1",
    KeyCode::F2 => "f2",
    KeyCode::F3 => "f3",
    KeyCode::F4 => "f4",
    KeyCode::F5 => "f5",
    KeyCode::F6 => "f6",
    KeyCode::F7 => "f7",
    KeyCode::F8 => "f8",
    KeyCode::F9 => "f9",
    KeyCode::F10 => "f10",
    KeyCode::F11 => "f11",
    KeyCode::F12 => "f12",
    KeyCode::F13 => "f13",
    KeyCode::F14 => "f14",
    KeyCode::F15 => "f15",
    KeyCode::F16 => "f16",
    KeyCode::F17 => "f17",
    KeyCode::F18 => "f18",
    KeyCode::F19 => "f19",
    KeyCode::F20 => "f20",
    KeyCode::F21 => "f21",
    KeyCode::F22 => "f22",
    KeyCode::F23 => "f23",
    KeyCode::F24 => "f24",
    KeyCode::F25 => "f25",
    KeyCode::F26 => "f26",
    KeyCode::F27 => "f27",
    KeyCode::F28 => "f28",
    KeyCode::F29 => "f29",
    KeyCode::F30 => "f30",
    KeyCode::F31 => "f31",
    KeyCode::F32 => "f32",
    KeyCode::F33 => "f33",
    KeyCode::F34 => "f34",
    KeyCode::F35 => "f35",

    _ => return None,
  };

  Some(c.to_string())
}

/**
 * Modifier keys used in hotkey combinations.
 */
#[napi(string_enum)]
pub enum Mods {
  Control,
  Alt,
  AltGraph,
  CapsLock,
  Fn,
  FnLock,
  Meta,
  NumLock,
  ScrollLock,
  Symbol,
  SymbolLock,
  Hyper,
  Shift,
  Super,
}

impl Mods {
  pub fn global_hotkeys(&self) -> Modifiers {
    match self {
      Mods::Control => Modifiers::CONTROL,
      Mods::Alt => Modifiers::ALT,
      Mods::Shift => Modifiers::SHIFT,
      Mods::Super => Modifiers::SUPER,
      Mods::AltGraph => Modifiers::ALT_GRAPH,
      Mods::CapsLock => Modifiers::CAPS_LOCK,
      Mods::Fn => Modifiers::FN,
      Mods::FnLock => Modifiers::FN_LOCK,
      Mods::Meta => Modifiers::META,
      Mods::NumLock => Modifiers::NUM_LOCK,
      Mods::ScrollLock => Modifiers::SCROLL_LOCK,
      Mods::Symbol => Modifiers::SYMBOL,
      Mods::SymbolLock => Modifiers::SYMBOL_LOCK,
      Mods::Hyper => Modifiers::HYPER,
    }
  }

  pub fn from_global_hotkeys(modifier: Modifiers) -> Option<Mods> {
    match modifier {
      Modifiers::CONTROL => Some(Mods::Control),
      Modifiers::ALT => Some(Mods::Alt),
      Modifiers::SHIFT => Some(Mods::Shift),
      Modifiers::SUPER => Some(Mods::Super),
      Modifiers::ALT_GRAPH => Some(Mods::AltGraph),
      Modifiers::CAPS_LOCK => Some(Mods::CapsLock),
      Modifiers::FN => Some(Mods::Fn),
      Modifiers::FN_LOCK => Some(Mods::FnLock),
      Modifiers::META => Some(Mods::Meta),
      Modifiers::NUM_LOCK => Some(Mods::NumLock),
      Modifiers::SCROLL_LOCK => Some(Mods::ScrollLock),
      Modifiers::SYMBOL => Some(Mods::Symbol),
      Modifiers::SYMBOL_LOCK => Some(Mods::SymbolLock),
      Modifiers::HYPER => Some(Mods::Hyper),
      _ => None,
    }
  }
}

/**
 * Descriptor for a hotkey combination.
 *
 * Contains the key code and modifier keys.
 */
#[napi(object)]
#[derive(Clone)]
pub struct Desc {
  pub code: KeyCode,
  pub mods: Vec<Mods>,
}

impl Desc {
  pub fn new(code: KeyCode, mods: Vec<Mods>) -> Self {
    Self { code, mods }
  }
}

/**
 * Describes the type of a hotkey event.
 *
 * Possible values:
 * - `Pressed`: The hotkey was pressed.
 * - `Released`: The hotkey was released.
 */
#[napi]
pub enum EventType {
  Pressed,
  Released,
}

/**
 * Represents a global hotkey event.
 *
 * Properties:
 * - `id` (number): The unique identifier of the hotkey.
 * - `code` (KeyCode): The key code associated with the hotkey.
 * - `mods` (Mods[]): An array of modifier keys (e.g., Control, Shift).
 * - `event_type` (EventType): The type of the event (pressed or released).
 */
#[napi(object)]
pub struct Event {
  pub id: u32,
  pub code: KeyCode,
  pub mods: Vec<Mods>,
  pub event_type: EventType,
}
