use std::hash::{Hash, Hasher};
use std::num::ToPrimitive;

use sys::scancode as ll;

#[derive(PartialEq, Eq, FromPrimitive, Debug, Copy)]
pub enum ScanCode {
    Unknown            = ll::SDL_SCANCODE_UNKNOWN as isize,
    A                  = ll::SDL_SCANCODE_A as isize,
    B                  = ll::SDL_SCANCODE_B as isize,
    C                  = ll::SDL_SCANCODE_C as isize,
    D                  = ll::SDL_SCANCODE_D as isize,
    E                  = ll::SDL_SCANCODE_E as isize,
    F                  = ll::SDL_SCANCODE_F as isize,
    G                  = ll::SDL_SCANCODE_G as isize,
    H                  = ll::SDL_SCANCODE_H as isize,
    I                  = ll::SDL_SCANCODE_I as isize,
    J                  = ll::SDL_SCANCODE_J as isize,
    K                  = ll::SDL_SCANCODE_K as isize,
    L                  = ll::SDL_SCANCODE_L as isize,
    M                  = ll::SDL_SCANCODE_M as isize,
    N                  = ll::SDL_SCANCODE_N as isize,
    O                  = ll::SDL_SCANCODE_O as isize,
    P                  = ll::SDL_SCANCODE_P as isize,
    Q                  = ll::SDL_SCANCODE_Q as isize,
    R                  = ll::SDL_SCANCODE_R as isize,
    S                  = ll::SDL_SCANCODE_S as isize,
    T                  = ll::SDL_SCANCODE_T as isize,
    U                  = ll::SDL_SCANCODE_U as isize,
    V                  = ll::SDL_SCANCODE_V as isize,
    W                  = ll::SDL_SCANCODE_W as isize,
    X                  = ll::SDL_SCANCODE_X as isize,
    Y                  = ll::SDL_SCANCODE_Y as isize,
    Z                  = ll::SDL_SCANCODE_Z as isize,
    Num1               = ll::SDL_SCANCODE_1 as isize,
    Num2               = ll::SDL_SCANCODE_2 as isize,
    Num3               = ll::SDL_SCANCODE_3 as isize,
    Num4               = ll::SDL_SCANCODE_4 as isize,
    Num5               = ll::SDL_SCANCODE_5 as isize,
    Num6               = ll::SDL_SCANCODE_6 as isize,
    Num7               = ll::SDL_SCANCODE_7 as isize,
    Num8               = ll::SDL_SCANCODE_8 as isize,
    Num9               = ll::SDL_SCANCODE_9 as isize,
    Num0               = ll::SDL_SCANCODE_0 as isize,
    Return             = ll::SDL_SCANCODE_RETURN as isize,
    Escape             = ll::SDL_SCANCODE_ESCAPE as isize,
    Backspace          = ll::SDL_SCANCODE_BACKSPACE as isize,
    Tab                = ll::SDL_SCANCODE_TAB as isize,
    Space              = ll::SDL_SCANCODE_SPACE as isize,
    Minus              = ll::SDL_SCANCODE_MINUS as isize,
    Equals             = ll::SDL_SCANCODE_EQUALS as isize,
    LeftBracket        = ll::SDL_SCANCODE_LEFTBRACKET as isize,
    RightBracket       = ll::SDL_SCANCODE_RIGHTBRACKET as isize,
    Backslash          = ll::SDL_SCANCODE_BACKSLASH as isize,
    NonUsHash          = ll::SDL_SCANCODE_NONUSHASH as isize,
    Semicolon          = ll::SDL_SCANCODE_SEMICOLON as isize,
    Apostrophe         = ll::SDL_SCANCODE_APOSTROPHE as isize,
    Grave              = ll::SDL_SCANCODE_GRAVE as isize,
    Comma              = ll::SDL_SCANCODE_COMMA as isize,
    Period             = ll::SDL_SCANCODE_PERIOD as isize,
    Slash              = ll::SDL_SCANCODE_SLASH as isize,
    CapsLock           = ll::SDL_SCANCODE_CAPSLOCK as isize,
    F1                 = ll::SDL_SCANCODE_F1 as isize,
    F2                 = ll::SDL_SCANCODE_F2 as isize,
    F3                 = ll::SDL_SCANCODE_F3 as isize,
    F4                 = ll::SDL_SCANCODE_F4 as isize,
    F5                 = ll::SDL_SCANCODE_F5 as isize,
    F6                 = ll::SDL_SCANCODE_F6 as isize,
    F7                 = ll::SDL_SCANCODE_F7 as isize,
    F8                 = ll::SDL_SCANCODE_F8 as isize,
    F9                 = ll::SDL_SCANCODE_F9 as isize,
    F10                = ll::SDL_SCANCODE_F10 as isize,
    F11                = ll::SDL_SCANCODE_F11 as isize,
    F12                = ll::SDL_SCANCODE_F12 as isize,
    PrintScreen        = ll::SDL_SCANCODE_PRINTSCREEN as isize,
    ScrollLock         = ll::SDL_SCANCODE_SCROLLLOCK as isize,
    Pause              = ll::SDL_SCANCODE_PAUSE as isize,
    Insert             = ll::SDL_SCANCODE_INSERT as isize,
    Home               = ll::SDL_SCANCODE_HOME as isize,
    PageUp             = ll::SDL_SCANCODE_PAGEUP as isize,
    Delete             = ll::SDL_SCANCODE_DELETE as isize,
    End                = ll::SDL_SCANCODE_END as isize,
    PageDown           = ll::SDL_SCANCODE_PAGEDOWN as isize,
    Right              = ll::SDL_SCANCODE_RIGHT as isize,
    Left               = ll::SDL_SCANCODE_LEFT as isize,
    Down               = ll::SDL_SCANCODE_DOWN as isize,
    Up                 = ll::SDL_SCANCODE_UP as isize,
    NumLockClear       = ll::SDL_SCANCODE_NUMLOCKCLEAR as isize,
    KpDivide           = ll::SDL_SCANCODE_KP_DIVIDE as isize,
    KpMultiply         = ll::SDL_SCANCODE_KP_MULTIPLY as isize,
    KpMinus            = ll::SDL_SCANCODE_KP_MINUS as isize,
    KpPlus             = ll::SDL_SCANCODE_KP_PLUS as isize,
    KpEnter            = ll::SDL_SCANCODE_KP_ENTER as isize,
    Kp1                = ll::SDL_SCANCODE_KP_1 as isize,
    Kp2                = ll::SDL_SCANCODE_KP_2 as isize,
    Kp3                = ll::SDL_SCANCODE_KP_3 as isize,
    Kp4                = ll::SDL_SCANCODE_KP_4 as isize,
    Kp5                = ll::SDL_SCANCODE_KP_5 as isize,
    Kp6                = ll::SDL_SCANCODE_KP_6 as isize,
    Kp7                = ll::SDL_SCANCODE_KP_7 as isize,
    Kp8                = ll::SDL_SCANCODE_KP_8 as isize,
    Kp9                = ll::SDL_SCANCODE_KP_9 as isize,
    Kp0                = ll::SDL_SCANCODE_KP_0 as isize,
    KpPeriod           = ll::SDL_SCANCODE_KP_PERIOD as isize,
    NonUsBackslash     = ll::SDL_SCANCODE_NONUSBACKSLASH as isize,
    Application        = ll::SDL_SCANCODE_APPLICATION as isize,
    Power              = ll::SDL_SCANCODE_POWER as isize,
    KpEquals           = ll::SDL_SCANCODE_KP_EQUALS as isize,
    F13                = ll::SDL_SCANCODE_F13 as isize,
    F14                = ll::SDL_SCANCODE_F14 as isize,
    F15                = ll::SDL_SCANCODE_F15 as isize,
    F16                = ll::SDL_SCANCODE_F16 as isize,
    F17                = ll::SDL_SCANCODE_F17 as isize,
    F18                = ll::SDL_SCANCODE_F18 as isize,
    F19                = ll::SDL_SCANCODE_F19 as isize,
    F20                = ll::SDL_SCANCODE_F20 as isize,
    F21                = ll::SDL_SCANCODE_F21 as isize,
    F22                = ll::SDL_SCANCODE_F22 as isize,
    F23                = ll::SDL_SCANCODE_F23 as isize,
    F24                = ll::SDL_SCANCODE_F24 as isize,
    Execute            = ll::SDL_SCANCODE_EXECUTE as isize,
    Help               = ll::SDL_SCANCODE_HELP as isize,
    Menu               = ll::SDL_SCANCODE_MENU as isize,
    Select             = ll::SDL_SCANCODE_SELECT as isize,
    Stop               = ll::SDL_SCANCODE_STOP as isize,
    Again              = ll::SDL_SCANCODE_AGAIN as isize,
    Undo               = ll::SDL_SCANCODE_UNDO as isize,
    Cut                = ll::SDL_SCANCODE_CUT as isize,
    Copy               = ll::SDL_SCANCODE_COPY as isize,
    Paste              = ll::SDL_SCANCODE_PASTE as isize,
    Find               = ll::SDL_SCANCODE_FIND as isize,
    Mute               = ll::SDL_SCANCODE_MUTE as isize,
    VolumeUp           = ll::SDL_SCANCODE_VOLUMEUP as isize,
    VolumeDown         = ll::SDL_SCANCODE_VOLUMEDOWN as isize,
    KpComma            = ll::SDL_SCANCODE_KP_COMMA as isize,
    KpEqualsAS400      = ll::SDL_SCANCODE_KP_EQUALSAS400 as isize,
    International1     = ll::SDL_SCANCODE_INTERNATIONAL1 as isize,
    International2     = ll::SDL_SCANCODE_INTERNATIONAL2 as isize,
    International3     = ll::SDL_SCANCODE_INTERNATIONAL3 as isize,
    International4     = ll::SDL_SCANCODE_INTERNATIONAL4 as isize,
    International5     = ll::SDL_SCANCODE_INTERNATIONAL5 as isize,
    International6     = ll::SDL_SCANCODE_INTERNATIONAL6 as isize,
    International7     = ll::SDL_SCANCODE_INTERNATIONAL7 as isize,
    International8     = ll::SDL_SCANCODE_INTERNATIONAL8 as isize,
    International9     = ll::SDL_SCANCODE_INTERNATIONAL9 as isize,
    Lang1              = ll::SDL_SCANCODE_LANG1 as isize,
    Lang2              = ll::SDL_SCANCODE_LANG2 as isize,
    Lang3              = ll::SDL_SCANCODE_LANG3 as isize,
    Lang4              = ll::SDL_SCANCODE_LANG4 as isize,
    Lang5              = ll::SDL_SCANCODE_LANG5 as isize,
    Lang6              = ll::SDL_SCANCODE_LANG6 as isize,
    Lang7              = ll::SDL_SCANCODE_LANG7 as isize,
    Lang8              = ll::SDL_SCANCODE_LANG8 as isize,
    Lang9              = ll::SDL_SCANCODE_LANG9 as isize,
    AltErase           = ll::SDL_SCANCODE_ALTERASE as isize,
    SysReq             = ll::SDL_SCANCODE_SYSREQ as isize,
    Cancel             = ll::SDL_SCANCODE_CANCEL as isize,
    Clear              = ll::SDL_SCANCODE_CLEAR as isize,
    Prior              = ll::SDL_SCANCODE_PRIOR as isize,
    Return2            = ll::SDL_SCANCODE_RETURN2 as isize,
    Separator          = ll::SDL_SCANCODE_SEPARATOR as isize,
    Out                = ll::SDL_SCANCODE_OUT as isize,
    Oper               = ll::SDL_SCANCODE_OPER as isize,
    ClearAgain         = ll::SDL_SCANCODE_CLEARAGAIN as isize,
    Crse               = ll::SDL_SCANCODE_CRSEL as isize,
    ExseL              = ll::SDL_SCANCODE_EXSEL as isize,
    Kp00               = ll::SDL_SCANCODE_KP_00 as isize,
    Kp000              = ll::SDL_SCANCODE_KP_000 as isize,
    ThousandsSeparator = ll::SDL_SCANCODE_THOUSANDSSEPARATOR as isize,
    DecimalSeparator   = ll::SDL_SCANCODE_DECIMALSEPARATOR as isize,
    CurrencyUnit       = ll::SDL_SCANCODE_CURRENCYUNIT as isize,
    CurrencySubUnit    = ll::SDL_SCANCODE_CURRENCYSUBUNIT as isize,
    KpLeftParen        = ll::SDL_SCANCODE_KP_LEFTPAREN as isize,
    KpRightParen       = ll::SDL_SCANCODE_KP_RIGHTPAREN as isize,
    KpLeftBrace        = ll::SDL_SCANCODE_KP_LEFTBRACE as isize,
    KpRightBrace       = ll::SDL_SCANCODE_KP_RIGHTBRACE as isize,
    KpTab              = ll::SDL_SCANCODE_KP_TAB as isize,
    KpBackspace        = ll::SDL_SCANCODE_KP_BACKSPACE as isize,
    KpA                = ll::SDL_SCANCODE_KP_A as isize,
    KpB                = ll::SDL_SCANCODE_KP_B as isize,
    KpC                = ll::SDL_SCANCODE_KP_C as isize,
    KpD                = ll::SDL_SCANCODE_KP_D as isize,
    KpE                = ll::SDL_SCANCODE_KP_E as isize,
    KpF                = ll::SDL_SCANCODE_KP_F as isize,
    KpXor              = ll::SDL_SCANCODE_KP_XOR as isize,
    KpPower            = ll::SDL_SCANCODE_KP_POWER as isize,
    KpPercent          = ll::SDL_SCANCODE_KP_PERCENT as isize,
    KpLess             = ll::SDL_SCANCODE_KP_LESS as isize,
    KpGreater          = ll::SDL_SCANCODE_KP_GREATER as isize,
    KpAmpersand        = ll::SDL_SCANCODE_KP_AMPERSAND as isize,
    KpDblAmpersand     = ll::SDL_SCANCODE_KP_DBLAMPERSAND as isize,
    KpVerticalBar      = ll::SDL_SCANCODE_KP_VERTICALBAR as isize,
    KpDblVerticalBar   = ll::SDL_SCANCODE_KP_DBLVERTICALBAR as isize,
    KpColon            = ll::SDL_SCANCODE_KP_COLON as isize,
    KpHash             = ll::SDL_SCANCODE_KP_HASH as isize,
    KpSpace            = ll::SDL_SCANCODE_KP_SPACE as isize,
    KpAt               = ll::SDL_SCANCODE_KP_AT as isize,
    KpExclam           = ll::SDL_SCANCODE_KP_EXCLAM as isize,
    KpMemStore         = ll::SDL_SCANCODE_KP_MEMSTORE as isize,
    KpMemRecall        = ll::SDL_SCANCODE_KP_MEMRECALL as isize,
    KpMemClear         = ll::SDL_SCANCODE_KP_MEMCLEAR as isize,
    KpMemAdd           = ll::SDL_SCANCODE_KP_MEMADD as isize,
    KpMemSubtract      = ll::SDL_SCANCODE_KP_MEMSUBTRACT as isize,
    KpMemMultiply      = ll::SDL_SCANCODE_KP_MEMMULTIPLY as isize,
    KpMemDivide        = ll::SDL_SCANCODE_KP_MEMDIVIDE as isize,
    KpPlusMinus        = ll::SDL_SCANCODE_KP_PLUSMINUS as isize,
    KpClear            = ll::SDL_SCANCODE_KP_CLEAR as isize,
    KpClearEntry       = ll::SDL_SCANCODE_KP_CLEARENTRY as isize,
    KpBinary           = ll::SDL_SCANCODE_KP_BINARY as isize,
    KpOoctal           = ll::SDL_SCANCODE_KP_OCTAL as isize,
    KpDecimal          = ll::SDL_SCANCODE_KP_DECIMAL as isize,
    KpHexadecimal      = ll::SDL_SCANCODE_KP_HEXADECIMAL as isize,
    LCtrl              = ll::SDL_SCANCODE_LCTRL as isize,
    LShift             = ll::SDL_SCANCODE_LSHIFT as isize,
    LAlt               = ll::SDL_SCANCODE_LALT as isize,
    LGui               = ll::SDL_SCANCODE_LGUI as isize,
    RCtrl              = ll::SDL_SCANCODE_RCTRL as isize,
    RShift             = ll::SDL_SCANCODE_RSHIFT as isize,
    RAlt               = ll::SDL_SCANCODE_RALT as isize,
    RGui               = ll::SDL_SCANCODE_RGUI as isize,
    Mode               = ll::SDL_SCANCODE_MODE as isize,
    AudioNext          = ll::SDL_SCANCODE_AUDIONEXT as isize,
    AudioPrev          = ll::SDL_SCANCODE_AUDIOPREV as isize,
    AudioStop          = ll::SDL_SCANCODE_AUDIOSTOP as isize,
    AudioPlay          = ll::SDL_SCANCODE_AUDIOPLAY as isize,
    AudioMute          = ll::SDL_SCANCODE_AUDIOMUTE as isize,
    MediaSelect        = ll::SDL_SCANCODE_MEDIASELECT as isize,
    Www                = ll::SDL_SCANCODE_WWW as isize,
    Mail               = ll::SDL_SCANCODE_MAIL as isize,
    Calculator         = ll::SDL_SCANCODE_CALCULATOR as isize,
    Computer           = ll::SDL_SCANCODE_COMPUTER as isize,
    AcSearch           = ll::SDL_SCANCODE_AC_SEARCH as isize,
    AcHome             = ll::SDL_SCANCODE_AC_HOME as isize,
    AcBack             = ll::SDL_SCANCODE_AC_BACK as isize,
    AcForward          = ll::SDL_SCANCODE_AC_FORWARD as isize,
    AcStop             = ll::SDL_SCANCODE_AC_STOP as isize,
    AcRefresh          = ll::SDL_SCANCODE_AC_REFRESH as isize,
    AcBookmarks        = ll::SDL_SCANCODE_AC_BOOKMARKS as isize,
    BrightnessDown     = ll::SDL_SCANCODE_BRIGHTNESSDOWN as isize,
    BrightnessUp       = ll::SDL_SCANCODE_BRIGHTNESSUP as isize,
    DisplaySwitch      = ll::SDL_SCANCODE_DISPLAYSWITCH as isize,
    KbdIllumToggle     = ll::SDL_SCANCODE_KBDILLUMTOGGLE as isize,
    KbdIllumDown       = ll::SDL_SCANCODE_KBDILLUMDOWN as isize,
    KbdIllumUp         = ll::SDL_SCANCODE_KBDILLUMUP as isize,
    Eject              = ll::SDL_SCANCODE_EJECT as isize,
    Sleep              = ll::SDL_SCANCODE_SLEEP as isize,
    App1               = ll::SDL_SCANCODE_APP1 as isize,
    App2               = ll::SDL_SCANCODE_APP2 as isize,
    Num                = ll::SDL_NUM_SCANCODES as isize,
}

impl Hash for ScanCode {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as i32).hash(state);
    }
}

impl ToPrimitive for ScanCode {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        Some(*self as i64)
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        Some(*self as u64)
    }

    #[inline]
    fn to_int(&self) -> Option<isize> {
        Some(*self as isize)
    }
}
