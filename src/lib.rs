pub mod touch_pad_enum;
use crate::touch_pad_enum::*;
pub mod touch_pad_error;
use crate::touch_pad_error::*;
use esp_idf_svc::sys::*;
use std::os::raw::c_void;

/// Initialize touch module.
///
/// # Error
///
/// * ESP_ERR_NO_MEM Touch pad init error
/// * ESP_ERR_NOT_SUPPORTED Touch pad is providing current to external XTAL
pub fn init() -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_init()) }
}

/// Deinitialize touch module.
///
/// # Error
///
/// * ESP_FAIL Touch pad driver not initialized
pub fn deinit() -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_deinit()) }
}

/// Trigger a touch sensor measurement, only support in SW mode of FSM.
pub fn sw_start() -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_sw_start()) }
}

/// Set touch sensor interrupt trigger source. There are two sets of touch signals. Set1 and set2 can be mapped to several touch signals.
/// Either set will be triggered if at least one of its touch signal is 'touched'.
/// The interrupt can be configured to be generated if set1 is triggered, or only if both sets are triggered.
///
/// # Arguments
///
/// * `src` - TouchTriggerSource.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG if argument is wrong
pub fn set_trigger_source(src: TouchTriggerSource) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_trigger_source(src as u32)) }
}

/// Get touch sensor interrupt trigger source.
///
/// # Arguments
///
/// * `src` - *mut TouchTriggerSource.
pub fn get_trigger_source(src: *mut TouchTriggerSource) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_trigger_source(src as *mut u32)) }
}

/// Set touch sensor interrupt trigger mode. Interrupt can be triggered either when counter result is less than threshold or when counter result is more than threshold.
///
/// # Arguments
///
/// * `mode` - TouchTriggerMode.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG if argument is wrong
pub fn set_trigger_mode(mode: TouchTriggerMode) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_trigger_mode(mode as u32)) }
}

/// Get touch sensor interrupt trigger mode.
///
/// # Arguments
///
/// * `mode` - *mut TouchTriggerMode.
pub fn get_trigger_mode(mode: *mut TouchTriggerMode) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_trigger_mode(mode as *mut u32)) }
}

/// Set touch sensor interrupt threshold.
///
/// # Arguments
///
/// * `touch_num` - TouchPadChannel.
/// * `threshold` - u16.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG if argument is wrong
pub fn set_thresh(touch_num: TouchPadChannel, threshold: u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_thresh(touch_num as u32, threshold)) }
}

/// Get touch sensor interrupt threshold.
///
/// # Arguments
///
/// * `touch_num` - TouchPadChannel.
/// * `threshold` - *mut u16.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG if argument is wrong
pub fn get_thresh(touch_num: TouchPadChannel, threshold: *mut u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_thresh(touch_num as u32, threshold)) }
}

/// Set touch sensor measurement and sleep time. Excessive total time will slow down the touch response.
/// Too small measurement time will not be sampled enough, resulting in inaccurate measurements.
///
/// # Arguments
///
/// * `sleep_cycle` - u16.
/// * `meas_cycle` - u16.
pub fn set_meas_time(sleep_cycle: u16, meas_cycle: u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_meas_time(sleep_cycle, meas_cycle)) }
}

/// Get touch sensor measurement and sleep time.
///
/// # Arguments
///
/// * `sleep_cycle` - *mut u16.
/// * `meas_cycle` - *mut u16.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG The input parameter is NULL
pub fn get_meas_time(sleep_cycle: *mut u16, meas_cycle: *mut u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_meas_time(sleep_cycle, meas_cycle)) }
}

/// Set the interval between two measurements.
///
/// # Arguments
///
/// * `interval_cycle` - u16.
pub fn set_measurement_interval(interval_cycle: u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_measurement_interval(interval_cycle)) }
}

/// Get the interval between two measurements.
///
/// # Arguments
///
/// * `interval_cycle` - *mut u16.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG The input parameter is NULL
pub fn get_measurement_interval(interval_cycle: *mut u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_measurement_interval(interval_cycle)) }
}

pub type FilterCbT = Option<unsafe extern "C" fn(raw_value: *mut u16, filtered_value: *mut u16)>;

/// Register the callback function that is called after each IIR filter calculation.
///
/// # Arguments
///
/// * `filter_cb` - FilterCbT.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG set error
pub fn set_filter_read_cb(filter_cb: FilterCbT) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_filter_read_cb(filter_cb)) }
}

/// Set touch sensor high voltage threshold of chanrge. The touch sensor measures the channel capacitance value by charging and discharging the channel.
/// So the high threshold should be less than the supply voltage.
///
/// # Arguments
///
/// * `touch_high_voltage` - TouchHighVoltage.
/// * `touch_low_voltage` - TouchLowVoltage.
/// * `touch_voltage_attenuation` - TouchVoltageAttenuation.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG if argument is wrong
pub fn set_voltage(
    touch_high_voltage: TouchHighVoltage,
    touch_low_voltage: TouchLowVoltage,
    touch_voltage_attenuation: TouchVoltageAttenuation,
) -> Result<(), EspErr> {
    unsafe {
        EspErr::return_message(touch_pad_set_voltage(
            touch_high_voltage as i32,
            touch_low_voltage as i32,
            touch_voltage_attenuation as i32,
        ))
    }
}

/// Get touch sensor reference voltage.
///
/// # Arguments
///
/// * `touch_high_voltage` - *mut TouchHighVoltage.
/// * `touch_low_voltage` - *mut TouchLowVoltage.
/// * `touch_voltage_attenuation` - *mut TouchVoltageAttenuation.

pub fn get_volatge(
    touch_high_voltage: *mut TouchHighVoltage,
    touch_low_voltage: *mut TouchLowVoltage,
    touch_voltage_attenuation: *mut TouchVoltageAttenuation,
) -> Result<(), EspErr> {
    unsafe {
        EspErr::return_message(touch_pad_get_voltage(
            touch_high_voltage as *mut i32,
            touch_low_voltage as *mut i32,
            touch_voltage_attenuation as *mut i32,
        ))
    }
}

/// Configure touch pad interrupt threshold.
///
/// # Arguments
///
/// * `touch_num` - TouchPadChannel.
/// * `threshold` - u16.
///
/// # Errors
///
/// * ESP_ERR_INVALID_ARG if argument wrong
/// * ESP_FAIL if touch pad not initialized
pub fn config(touch_num: TouchPadChannel, threshold: u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_config(touch_num as u32, threshold)) }
}

/// get touch sensor counter value. Each touch sensor has a counter to count the number of charge/discharge cycles. When the pad is not 'touched',
/// we can get a number of the counter. When the pad is 'touched', the value in counter will get smaller because of the larger equivalent capacitance.
///
/// # Arguments
///
/// * `touch_num` - TouchPadChannel.
/// * `touch_value` - *mut u16.
///
/// # Errors
///
/// * ESP_ERR_INVALID_ARG Touch pad parameter error
/// * ESP_ERR_INVALID_STATE This touch pad hardware connection is error, the value of "touch_value" is 0.
/// * ESP_FAIL Touch pad not initialized
pub fn read(touch_num: TouchPadChannel, touch_value: *mut u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_read(touch_num as u32, touch_value)) }
}

/// get filtered touch sensor counter value by IIR filter.
///
/// # Arguments
///
/// * `touch_num` - TouchPadChannel.
/// * `touch_value` - *mut u16.
///
/// # Errors
///
/// * ESP_ERR_INVALID_ARG Touch pad parameter error
/// * ESP_ERR_INVALID_STATE This touch pad hardware connection is error, the value of "touch_value" is 0.
/// * ESP_FAIL Touch pad not initialized
pub fn read_filtered(touch_num: TouchPadChannel, touch_value: *mut u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_read_filtered(touch_num as u32, touch_value)) }
}

/// get raw data (touch sensor counter value) from IIR filter process. Need not request hardware measurements.
///
/// # Arguments
///
/// * `touch_num` - TouchPadChannel.
/// * `touch_value` - *mut u16.
///
/// # Errors
///
/// * ESP_ERR_INVALID_ARG Touch pad parameter error
/// * ESP_ERR_INVALID_STATE This touch pad hardware connection is error, the value of "touch_value" is 0.
/// * ESP_FAIL Touch pad not initialized
pub fn read_raw_data(touch_num: TouchPadChannel, touch_value: *mut u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_read_raw_data(touch_num as u32, touch_value)) }
}

/// start touch pad filter function This API will start a filter to process the noise in order to prevent false triggering when detecting slight change of capacitance.
/// Need to call touch_pad_filter_start before all touch filter APIs
///
/// # Arguments
///
/// * `filter_period_ms` - u32.
///
/// # Errors
///
/// * ESP_ERR_INVALID_ARG parameter error
/// * ESP_ERR_NO_MEM No memory for driver
/// * ESP_ERR_INVALID_STATE driver state error
pub fn filter_start(filter_period_ms: u32) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_filter_start(filter_period_ms)) }
}

/// Get the clock cycles of each measurement.
///
/// # Arguments
///
/// * `clock_cycle` - *mut u16.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG The input parameter is NULL
pub fn get_measurement_clock_cycles(clock_cycle: *mut u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_measurement_clock_cycles(clock_cycle)) }
}

/// Set the clock cycles of each measurement.
///
/// # Arguments
///
/// * `clock_cycle` - u16.
///
/// # Note
///
/// This function will specify the clock cycles of each measurement and the clock is sourced from SOC_MOD_CLK_RTC_FAST, its default frequency is SOC_CLK_RC_FAST_FREQ_APPROX The touch sensor will record the charge and discharge times during these clock cycles as the final result (raw value)
/// If clock cyles is too small, it may lead to inaccurate results.

pub fn set_measurement_clock_cycles(clock_cycle: u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_measurement_clock_cycles(clock_cycle)) }
}

/// Set touch sensor group mask. Touch pad module has two sets of signals, 'Touched' signal is triggered only if at least one of touch pad in this group is \"touched\".
/// This function will set the register bits according to the given bitmask.
///
/// # Arguments
///
/// * set1_mask -- bitmask touch sensor signal group1, it's a 10-bit value
/// * set2_mask -- bitmask touch sensor signal group2, it's a 10-bit value
/// * en_mask -- bitmask of touch sensor work enable, it's a 10-bit value
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG if argument is wrong
pub fn set_group_mask(set1_mask: u16, set2_mask: u16, en_mask: u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_group_mask(set1_mask, set2_mask, en_mask)) }
}

/// Get touch sensor group mask.
///
/// # Arguments
///
/// * set1_mask -- pointer to accept bitmask of touch sensor signal group1, it's a 10-bit value
/// * set2_mask -- pointer to accept bitmask of touch sensor signal group2, it's a 10-bit value
/// * en_mask -- pointer to accept bitmask of touch sensor work enable, it's a 10-bit value

pub fn get_group_mask(
    set1_mask: *mut u16,
    set2_mask: *mut u16,
    en_mask: *mut u16,
) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_group_mask(set1_mask, set2_mask, en_mask)) }
}

/// Clear touch sensor group mask. Touch pad module has two sets of signals, Interrupt is triggered only if at least one of touch pad in this group is \"touched\".
/// This function will clear the register bits according to the given bitmask.
///
/// # Arguments
///
/// * set1_mask -- bitmask touch sensor signal group1, it's a 10-bit value
/// * set2_mask -- bitmask touch sensor signal group2, it's a 10-bit value
/// * en_mask -- bitmask of touch sensor work enable, it's a 10-bit value
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG if argument is wrong
pub fn clear_group_mask(set1_mask: u16, set2_mask: u16, en_mask: u16) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_clear_group_mask(set1_mask, set2_mask, en_mask)) }
}

/// To enable touch pad interrupt.
///
/// # Error
///
/// * ESP_OK on success
pub fn intr_enable() -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_intr_enable()) }
}

/// To disable touch pad interrupt.
pub fn intr_disable() -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_intr_disable()) }
}

/// To clear touch pad interrupt.
pub fn intr_clear() -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_intr_clear()) }
}

/// set touch pad filter calibration period, in ms. Need to call touch_pad_filter_start before all touch filter APIs
///
/// # Arguments
///
/// * `p_period_ms` - u32.
///
/// # Errors
///
/// * ESP_ERR_INVALID_STATE driver state error
/// * ESP_ERR_INVALID_ARG parameter error
pub fn set_filter_period(new_period_ms: u32) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_filter_period(new_period_ms)) }
}

/// get touch pad filter calibration period, in ms Need to call touch_pad_filter_start before all touch filter APIs
///
/// # Arguments
///
/// * `p_period_ms` - *mut u32.
///
/// # Errors
///
/// * ESP_ERR_INVALID_STATE driver state error
/// * ESP_ERR_INVALID_ARG parameter error
pub fn get_filter_period(p_period_ms: *mut u32) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_filter_period(p_period_ms)) }
}

/// stop touch pad filter function Need to call touch_pad_filter_start before all touch filter APIs
///
/// # Error
///
/// * ESP_ERR_INVALID_STATE driver state error
pub fn filter_stop() -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_filter_stop()) }
}

/// delete touch pad filter driver and release the memory Need to call touch_pad_filter_start before all touch filter APIs
///
/// # Error
///
/// * ESP_ERR_INVALID_STATE driver state error
pub fn filter_delete() -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_filter_delete()) }
}

/// Initialize touch pad GPIO.
///
/// # Arguments
///
/// * `src` - TouchTriggerSource.
///
/// # Error
///
/// * ESP_ERR_INVALID_ARG if argument is wrong
pub fn pad_io_init(touch_num: TouchPadChannel) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_io_init(touch_num as u32)) }
}

/// Set touch sensor charge/discharge speed for each pad. If the slope is 0, the counter would always be zero. If the slope is 1,
/// the charging and discharging would be slow, accordingly. If the slope is set 7, which is the maximum value, the charging and discharging would be fast.
///
/// # Arguments
/// touch_num -- TouchPadChannel
/// slope -- TouchCountSlope
/// opt -- TouchTieOption
///
/// # Error
///
/// ESP_ERR_INVALID_ARG if argument is wrong
pub fn set_cnt_mode(
    touch_num: TouchPadChannel,
    slope: TouchCountSlope,
    opt: TouchTieOption,
) -> Result<(), EspErr> {
    unsafe {
        EspErr::return_message(touch_pad_set_cnt_mode(
            touch_num as u32,
            slope as u32,
            opt as u32,
        ))
    }
}

/// Get touch sensor charge/discharge speed for each pad.
///
/// # Arguments
/// touch_num -- TouchPadChannel
/// slope -- TouchCountSlope
/// opt -- TouchTieOption
///
/// # Error
///
/// ESP_ERR_INVALID_ARG if argument is wrong
pub fn get_cnt_mode(
    touch_num: TouchPadChannel,
    slope: *mut TouchCountSlope,
    opt: *mut TouchTieOption,
) -> Result<(), EspErr> {
    unsafe {
        EspErr::return_message(touch_pad_get_cnt_mode(
            touch_num as u32,
            slope as *mut u32,
            opt as *mut u32,
        ))
    }
}

/// Get the touch pad which caused wakeup from deep sleep.
///
/// # Arguments
///
/// touch_num -- TouchPadChannel
///
/// # Error
///
/// ESP_ERR_INVALID_ARG if parameter is NULL
pub fn get_wakeup_status(pad_num: *mut TouchPadChannel) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_wakeup_status(pad_num as *mut u32)) }
}

/// Set touch sensor FSM mode, the test action can be triggered by the timer, as well as by the software.
///
/// # Arguments
///
/// * `mode` - TouchFSMMode.
///
/// # Error
///
/// ESP_ERR_INVALID_ARG if argument is wrong
pub fn set_fsm_mode(mode: TouchFSMMode) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_set_fsm_mode(mode as u32)) }
}

/// Get touch sensor FSM mode.
///
/// # Arguments
///
/// * `mode` - TouchFSMMode.
pub fn get_fsm_mode(mode: *mut TouchFSMMode) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_get_fsm_mode(mode as *mut u32)) }
}

/// To clear the touch sensor channel active status.
pub fn clear_status() -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_clear_status()) }
}

/// Get the touch sensor channel active status mask. The bit position represents the channel number.
/// The 0/1 status of the bit represents the trigger status.
///
/// # Error
///
/// * u32
pub fn get_status() -> u32 {
    unsafe { touch_pad_get_status() }
}

/// Check touch sensor measurement status.
///
/// # Error
///
/// * bool
pub fn meas_is_done() -> bool {
    unsafe { touch_pad_meas_is_done() }
}

pub type IntrHandler = Option<unsafe extern "C" fn(arg: *mut c_void)>;

/// Register touch-pad ISR. The handler will be attached to the same CPU core that this function is running on. // TODO hum
///
/// # Arguments
///
/// * `isr_handler` - IntrHandler.
/// * `arg` - Parameter for ISR.
///
/// # Errors
///
/// * ESP_ERR_INVALID_ARG GPIO error
/// * ESP_ERR_NO_MEM No memory
pub fn isr_register(isr_handler: IntrHandler, arg: *mut c_void) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_isr_register(isr_handler, arg)) }
}

/// Deregister the handler previously registered using touch_pad_isr_handler_register. // TODO hum
///
/// # Arguments
///
/// * `isr_handler` - handler function to call (as passed to isr_register).
/// * `arg` - argument of the handler (as passed to isr_register).
///
/// # Error
///
/// * ESP_ERR_INVALID_STATE if a handler matching both fn and arg isn't registered
pub fn isr_deregister(isr_handler: IntrHandler, arg: *mut c_void) -> Result<(), EspErr> {
    unsafe { EspErr::return_message(touch_pad_isr_deregister(isr_handler, arg)) }
}
