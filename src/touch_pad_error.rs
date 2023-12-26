use std::fmt;
use std::fmt::Display;

#[repr(i32)]
#[derive(Debug, PartialEq)]
pub enum EspErr {
    EspOk = 0,
    EspFail = -1,
    EspErrNoMem = 0x101,
    EspErrInvalidArg = 0x102,
    EspErrInvalidState = 0x103,
    EspErrInvalidSize = 0x104,
    EspErrNotFound = 0x105,
    EspErrNotSupported = 0x106,
    EspErrTimeout = 0x107,
    EspErrInvalidResponse = 0x108,
    EspErrInvalidCrc = 0x109,
    EspErrInvalidVersion = 0x10A,
    EspErrInvalidMac = 0x10B,
    EspErrNotFinished = 0x10C,
    EspErrNotAllowed = 0x10D,
    EspErrWifiBase = 0x3000,
    EspErrMeshBase = 0x4000,
    EspErrFlashBase = 0x6000,
    EspErrHwCryptoBase = 0xc000,
    EspErrMemprotBase = 0xd000,
    EspErrUnknown,
}

impl std::error::Error for EspErr {}

impl Display for EspErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.return_message_as_text())
    }
}

impl EspErr {
    pub fn return_message(valeur: i32) -> Result<(), EspErr> {
        match valeur {
            0 => Ok(()),
            -1 => Err(EspErr::EspFail),
            0x101 => Err(EspErr::EspErrNoMem),
            0x102 => Err(EspErr::EspErrInvalidArg),
            0x103 => Err(EspErr::EspErrInvalidState),
            0x104 => Err(EspErr::EspErrInvalidSize),
            0x105 => Err(EspErr::EspErrNotFound),
            0x106 => Err(EspErr::EspErrNotSupported),
            0x107 => Err(EspErr::EspErrTimeout),
            0x108 => Err(EspErr::EspErrInvalidResponse),
            0x109 => Err(EspErr::EspErrInvalidCrc),
            0x10A => Err(EspErr::EspErrInvalidVersion),
            0x10B => Err(EspErr::EspErrInvalidMac),
            0x10C => Err(EspErr::EspErrNotFinished),
            0x10D => Err(EspErr::EspErrNotAllowed),
            0x3000 => Err(EspErr::EspErrWifiBase),
            0x4000 => Err(EspErr::EspErrMeshBase),
            0x6000 => Err(EspErr::EspErrFlashBase),
            0xC000 => Err(EspErr::EspErrHwCryptoBase),
            0xD000 => Err(EspErr::EspErrMemprotBase),
            _ => Err(EspErr::EspErrUnknown),
        }
    }

    pub fn return_message_as_text(&self) -> &'static str {
        match self {
            EspErr::EspOk => "Success",
            EspErr::EspFail => "Generic failure",
            EspErr::EspErrNoMem => "Out of memory",
            EspErr::EspErrInvalidArg => "Invalid argument",
            EspErr::EspErrInvalidState => "Invalid state",
            EspErr::EspErrInvalidSize => "Invalid size",
            EspErr::EspErrNotFound => "Requested resource not found",
            EspErr::EspErrNotSupported => "Operation or feature not supported",
            EspErr::EspErrTimeout => "Operation timed out",
            EspErr::EspErrInvalidResponse => "Received response was invalid",
            EspErr::EspErrInvalidCrc => "CRC or checksum was invalid",
            EspErr::EspErrInvalidVersion => "Version was invalid",
            EspErr::EspErrInvalidMac => "MAC address was invalid",
            EspErr::EspErrNotFinished => "Operation has not fully completed",
            EspErr::EspErrNotAllowed => "Operation is not allowed",
            EspErr::EspErrWifiBase => "WiFi error",
            EspErr::EspErrMeshBase => "MESH error",
            EspErr::EspErrFlashBase => "Flash error",
            EspErr::EspErrHwCryptoBase => "HW cryptography module error",
            EspErr::EspErrMemprotBase => "Memory Protection API error",
            EspErr::EspErrUnknown => "Unknown error",
        }
    }
}
