/// Touch pad channel
#[repr(u32)]
pub enum TouchPadChannel {
    /// GPIO4(ESP32)
    /// Touch pad channel 0 is GPIO4(ESP32)
    Num0 = 0,
    /// GPIO0(ESP32) / GPIO1(ESP32-S2)
    /// Touch pad channel 1 is GPIO0(ESP32) / GPIO1(ESP32-S2)
    Num1 = 1,
    /// GPIO2(ESP32) / GPIO2(ESP32-S2)
    /// Touch pad channel 2 is GPIO2(ESP32) / GPIO2(ESP32-S2)
    Num2 = 2,
    /// GPIO15(ESP32) / GPIO3(ESP32-S2)
    /// Touch pad channel 3 is GPIO15(ESP32) / GPIO3(ESP32-S2)
    Num3 = 3,
    /// GPIO13(ESP32) / GPIO4(ESP32-S2)
    /// Touch pad channel 4 is GPIO13(ESP32) / GPIO4(ESP32-S2)
    Num4 = 4,
    /// GPIO12(ESP32) / GPIO5(ESP32-S2)
    /// Touch pad channel 5 is GPIO12(ESP32) / GPIO5(ESP32-S2)
    Num5 = 5,
    /// GPIO14(ESP32) / GPIO6(ESP32-S2)
    /// Touch pad channel 6 is GPIO14(ESP32) / GPIO6(ESP32-S2)
    Num6 = 6,
    /// GPIO27(ESP32) / GPIO7(ESP32-S2)
    /// Touch pad channel 7 is GPIO27(ESP32) / GPIO7(ESP32-S2)
    Num7 = 7,
    /// GPIO33(ESP32) / GPIO8(ESP32-S2)
    /// Touch pad channel 8 is GPIO33(ESP32) / GPIO8(ESP32-S2)
    Num8 = 8,
    /// GPIO32(ESP32) / GPIO9(ESP32-S2)
    /// Touch pad channel 9 is GPIO32(ESP32) / GPIO9(ESP32-S2)
    Num9 = 9,
    /// Maximum value
    /// Maximum value
    Max = 10,
}

/// Touch sensor high reference voltage
#[repr(i32)]
pub enum TouchHighVoltage {
    /// No change
    /// Touch sensor high reference voltage, no change
    Keep = -1,
    /// 2.4V
    /// Touch sensor high reference voltage, 2.4V
    V2_4 = 0,
    /// 2.5V
    /// Touch sensor high reference voltage, 2.5V
    V2_5 = 1,
    /// 2.6V
    /// Touch sensor high reference voltage, 2.6V
    V2_6 = 2,
    /// 2.7V
    /// Touch sensor high reference voltage, 2.7V
    V2_7 = 3,
    /// Maximum value
    /// Touch sensor high reference voltage maximum value
    Max = 4,
}

/// Touch sensor low reference voltage
#[repr(i32)]
pub enum TouchLowVoltage {
    /// No change
    /// Touch sensor low reference voltage, no change
    Keep = -1,
    /// 0.5V
    /// Touch sensor low reference voltage, 0.5V
    V0_5 = 0,
    /// 0.6V
    /// Touch sensor low reference voltage, 0.6V
    V0_6 = 1,
    /// 0.7V
    /// Touch sensor low reference voltage, 0.7V
    V0_7 = 2,
    /// 0.8V
    /// Touch sensor low reference voltage, 0.8V
    V0_8 = 3,
    /// Maximum value
    /// Touch sensor low reference voltage maximum value
    Max = 4,
}

/// Touch sensor high reference voltage attenuation
#[repr(i32)]
pub enum TouchVoltageAttenuation {
    /// No change
    /// Touch sensor high reference voltage attenuation, no change
    Keep = -1,
    /// 1.5V attenuation
    /// Touch sensor high reference voltage attenuation, 1.5V attenuation
    V1_5 = 0,
    /// 1.0V attenuation
    /// Touch sensor high reference voltage attenuation, 1.0V attenuation
    V1_0 = 1,
    /// 0.5V attenuation
    /// Touch sensor high reference voltage attenuation, 0.5V attenuation
    V0_5 = 2,
    /// 0V attenuation
    /// Touch sensor high reference voltage attenuation, 0V attenuation
    V0_0 = 3,
    /// Maximum value
    /// Touch sensor high reference voltage attenuation maximum value
    Max = 4,
}

/// Touch sensor charge/discharge speed
#[repr(u32)]
pub enum TouchCountSlope {
    /// Always zero
    /// Touch sensor charge/discharge speed, always zero
    Slope0 = 0,
    /// Slowest
    /// Touch sensor charge/discharge speed, slowest
    Slope1 = 1,
    /// Speed 2
    /// Touch sensor charge/discharge speed, 2
    Slope2 = 2,
    /// Speed 3
    /// Touch sensor charge/discharge speed, 3
    Slope3 = 3,
    /// Speed 4
    /// Touch sensor charge/discharge speed, 4
    Slope4 = 4,
    /// Speed 5
    /// Touch sensor charge/discharge speed, 5
    Slope5 = 5,
    /// Speed 6
    /// Touch sensor charge/discharge speed, 6
    Slope6 = 6,
    /// Fast
    /// Touch sensor charge/discharge speed, fast
    Slope7 = 7,
    /// Maximum value
    /// Touch sensor charge/discharge speed maximum value
    Max = 8,
}

/// Touch sensor initial charge level
#[repr(u32)]
pub enum TouchTieOption {
    /// Low level
    /// Initial level of charging voltage, low level
    Low = 0,
    /// High level
    /// Initial level of charging voltage, high level
    High = 1,
    /// Maximum value
    /// Touch sensor initial charge level maximum value
    Max = 2,
}

/// Touch sensor FSM mode
#[repr(u32)]
pub enum TouchFSMMode {
    /// Start touch FSM by timer
    /// To start touch FSM by timer
    Timer = 0,
    /// Start touch FSM by software trigger
    /// To start touch FSM by software trigger
    SW = 1,
    /// Maximum value
    /// Touch sensor FSM mode maximum value
    Max = 2,
}

/// Touch trigger mode
#[repr(u32)]
pub enum TouchTriggerMode {
    /// Touch interrupt will happen if counter value is less than threshold
    /// Touch interrupt will happen if counter value is less than threshold.
    Below = 0,
    /// Touch interrupt will happen if counter value is larger than threshold
    /// Touch interrupt will happen if counter value is larger than threshold.
    Above = 1,
    /// Maximum value
    /// Touch trigger mode maximum value
    Max = 2,
}

/// Touch trigger source
#[repr(u32)]
pub enum TouchTriggerSource {
    /// Wakeup interrupt is generated if both SET1 and SET2 are "touched"
    /// Wakeup interrupt is generated if both SET1 and SET2 are \"touched\"
    Both = 0,
    /// Wakeup interrupt is generated if SET1 is "touched"
    /// Wakeup interrupt is generated if SET1 is \"touched\"
    Set1 = 1,
    /// Maximum value
    /// Touch trigger source maximum value
    Max = 2,
}

/// GPIO port
#[repr(u32)]
pub enum GPIOPort {
    /// Port 0
    /// GPIO port 0
    Port0 = 0,
    /// Maximum value
    /// GPIO port maximum value
    Max = 1,
}

/// GPIO number type
#[repr(i32)]
pub enum GpioNum {
    /// < Use to signal not connected to S/W
    GpioNumNc = -1,
    /// < GPIO0, input and output
    GpioNum0 = 0,
    /// < GPIO1, input and output
    GpioNum1 = 1,
    /// < GPIO2, input and output
    GpioNum2 = 2,
    /// < GPIO3, input and output
    GpioNum3 = 3,
    /// < GPIO4, input and output
    GpioNum4 = 4,
    /// < GPIO5, input and output
    GpioNum5 = 5,
    /// < GPIO6, input and output
    GpioNum6 = 6,
    /// < GPIO7, input and output
    GpioNum7 = 7,
    /// < GPIO8, input and output
    GpioNum8 = 8,
    /// < GPIO9, input and output
    GpioNum9 = 9,
    /// < GPIO10, input and output
    GpioNum10 = 10,
    /// < GPIO11, input and output
    GpioNum11 = 11,
    /// < GPIO12, input and output
    GpioNum12 = 12,
    /// < GPIO13, input and output
    GpioNum13 = 13,
    /// < GPIO14, input and output
    GpioNum14 = 14,
    /// < GPIO15, input and output
    GpioNum15 = 15,
    /// < GPIO16, input and output
    GpioNum16 = 16,
    /// < GPIO17, input and output
    GpioNum17 = 17,
    /// < GPIO18, input and output
    GpioNum18 = 18,
    /// < GPIO19, input and output
    GpioNum19 = 19,
    /// < GPIO20, input and output
    GpioNum20 = 20,
    /// < GPIO21, input and output
    GpioNum21 = 21,
    /// < GPIO22, input and output
    GpioNum22 = 22,
    /// < GPIO23, input and output
    GpioNum23 = 23,
    /// < GPIO25, input and output
    GpioNum25 = 25,
    /// < GPIO26, input and output
    GpioNum26 = 26,
    /// < GPIO27, input and output
    GpioNum27 = 27,
    /// < GPIO28, input and output
    GpioNum28 = 28,
    /// < GPIO29, input and output
    GpioNum29 = 29,
    /// < GPIO30, input and output
    GpioNum30 = 30,
    /// < GPIO31, input and output
    GpioNum31 = 31,
    /// < GPIO32, input and output
    GpioNum32 = 32,
    /// < GPIO33, input and output
    GpioNum33 = 33,
    /// < GPIO34, input mode only
    GpioNum34 = 34,
    /// < GPIO35, input mode only
    GpioNum35 = 35,
    /// < GPIO36, input mode only
    GpioNum36 = 36,
    /// < GPIO37, input mode only
    GpioNum37 = 37,
    /// < GPIO38, input mode only
    GpioNum38 = 38,
    /// < GPIO39, input mode only
    GpioNum39 = 39,
    /// < Maximum GPIO number
    GpioNumMax = 40,
}

/// GPIO interrupt type
#[repr(u32)]
pub enum GpioIntType {
    /// < Disable GPIO interrupt
    GpioIntrDisable = 0,
    /// < GPIO interrupt type: rising edge
    GpioIntrPosedge = 1,
    /// < GPIO interrupt type: falling edge
    GpioIntrNegedge = 2,
    /// < GPIO interrupt type: both rising and falling edge
    GpioIntrAnyedge = 3,
    /// < GPIO interrupt type: input low level trigger
    GpioIntrLowLevel = 4,
    /// < GPIO interrupt type: input high level trigger
    GpioIntrHighLevel = 5,
    /// < Maximum GPIO interrupt type value
    GpioIntrMax = 6,
}

/// GPIO mode type
#[repr(u32)]
pub enum GpioMode {
    /// < Disable GPIO input and output
    GpioModeDisable = 0,
    /// < GPIO mode: input only
    GpioModeInput = 1,
    /// < GPIO mode: output only
    GpioModeOutput = 2,
    /// < GPIO mode: output only with open-drain mode
    GpioModeOutputOd = 6,
    /// < GPIO mode: output and input with open-drain mode
    GpioModeInputOutputOd = 7,
    /// < GPIO mode: output and input mode
    GpioModeInputOutput = 3,
}

/// GPIO pull-up resistor type
#[repr(u32)]
pub enum GpioPullup {
    /// < Disable GPIO pull-up resistor
    GpioPullupDisable = 0,
    /// < Enable GPIO pull-up resistor
    GpioPullupEnable = 1,
}

/// GPIO pull-down resistor type
#[repr(u32)]
pub enum GpioPulldown {
    /// < Disable GPIO pull-down resistor
    GpioPulldownDisable = 0,
    /// < Enable GPIO pull-down resistor
    GpioPulldownEnable = 1,
}

/// GPIO pull mode type
#[repr(u32)]
pub enum GpioPullMode {
    /// < Pad pull up
    GpioPullupOnly = 0,
    /// < Pad pull down
    GpioPulldownOnly = 1,
    /// < Pad pull up + pull down
    GpioPullupPulldown = 2,
    /// < Pad floating
    GpioFloating = 3,
}

/// GPIO drive capability type
#[repr(u32)]
pub enum GpioDriveCapability {
    /// < Pad drive capability: weak
    GpioDriveCap0 = 0,
    /// < Pad drive capability: stronger
    GpioDriveCap1 = 1,
    /// < Pad drive capability: medium and default
    GpioDriveCap2 = 2,
    /// < Pad drive capability: strongest
    GpioDriveCap3 = 3,
    /// < Maximum pad drive capability value
    GpioDriveCapMax = 4,
}

/// GPIO hysteresis control mode type
#[repr(u32)]
pub enum GpioHysteresisControlMode {
    /// < Pad input hysteresis controlled by efuse
    GpioHysCtrlEfuse = 0,
    /// < Pad input hysteresis enabled by software
    GpioHysSoftEnable = 1,
    /// < Pad input hysteresis disabled by software
    GpioHysSoftDisable = 2,
}

/// EXT1 wakeup mode type
#[repr(u32)]
pub enum EspSleepExt1WakeupMode {
    /// !< Wake the chip when all selected GPIOs go low
    EspExt1WakeupAllLow = 0,
    /// !< Wake the chip when any of the selected GPIOs go high
    EspExt1WakeupAnyHigh = 1,
}

/// Power domains which can be powered down in sleep mode
#[repr(u32)]
pub enum EspSleepPdDomain {
    /// !< RTC peripherals power domain
    EspPdDomainRtcPeriph = 0,
    /// !< RTC slow memory power domain
    EspPdDomainRtcSlowMem = 1,
    /// !< RTC fast memory power domain
    EspPdDomainRtcFastMem = 2,
    /// !< XTAL oscillator power domain
    EspPdDomainXtal = 3,
    /// !< Internal Fast oscillator power domain
    EspPdDomainRcFast = 4,
    /// !< VDD_SDIO power domain
    EspPdDomainVddsdio = 5,
    /// !< MODEM power domain (WiFi, Bluetooth, IEEE802.15.4)
    EspPdDomainModem = 6,
    /// !< Maximum power domain value
    EspPdDomainMax = 7,
}

/// Power down options
#[repr(u32)]
pub enum EspSleepPdOption {
    /// !< Power down the power domain in sleep mode
    EspPdOptionOff = 0,
    /// !< Keep power domain enabled during sleep mode
    EspPdOptionOn = 1,
    /// !< Keep power domain enabled in sleep mode if needed by wakeup options, otherwise power it down
    EspPdOptionAuto = 2,
}

/// Sleep wakeup cause
#[repr(u32)]
pub enum EspSleepWakeupCause {
    /// !< In case of deep sleep, reset was not caused by exit from deep sleep
    EspSleepWakeupUndefined = 0,
    /// !< Not a wakeup cause, used to disable all wakeup sources with esp_sleep_disable_wakeup_source
    EspSleepWakeupAll = 1,
    /// !< Wakeup caused by external signal using RTC_IO
    EspSleepWakeupExt0 = 2,
    /// !< Wakeup caused by external signal using RTC_CNTL
    EspSleepWakeupExt1 = 3,
    /// !< Wakeup caused by timer
    EspSleepWakeupTimer = 4,
    /// !< Wakeup caused by touchpad
    EspSleepWakeupTouchpad = 5,
    /// !< Wakeup caused by ULP program
    EspSleepWakeupUlp = 6,
    /// !< Wakeup caused by GPIO (light sleep only on ESP32, S2, and S3)
    EspSleepWakeupGpio = 7,
    /// !< Wakeup caused by UART (light sleep only)
    EspSleepWakeupUart = 8,
    /// !< Wakeup caused by WIFI (light sleep only)
    EspSleepWakeupWifi = 9,
    /// !< Wakeup caused by COCPU int
    EspSleepWakeupCocpu = 10,
    /// !< Wakeup caused by COCPU crash
    EspSleepWakeupCocpuTrapTrig = 11,
    /// !< Wakeup caused by BT (light sleep only)
    EspSleepWakeupBt = 12,
}

/// Sleep mode
#[repr(u32)]
pub enum EspSleepMode {
    /// !< Light sleep mode
    EspSleepModeLightSleep = 0,
    /// !< Deep sleep mode
    EspSleepModeDeepSleep = 1,
}
