use core::ffi::{c_void, c_int};
use crate::interface::callbacks::{__shim_mutex_lock, __shim_mutex_unlock};
use crate::{log_debug, log_error, IoTd, OperationResult};
use crate::allocator::Allocator;
use sx1278::error::SX1278Error;
use crate::{log_alert, log_info};

#[derive(Debug)]
#[repr(C)]
pub struct IotdConfig {
    /* Comm params */
    pub spreading_factor: u8,
    pub coding_rate: u8,
    pub bandwidth: u8,
    pub use_header: bool,
    pub use_payload_crc: bool,
    pub use_cad: bool,
    pub use_ldro: bool,
    pub preamble_length: u16,

    /* Device parameters */
    pub xtal_freq: u32,
    pub xtal_is_tcxo: bool,

    /* RF parameters */
    pub carrier_frequency: u32,
    pub output_power: i16,
    pub pa_boost: u8,

    /* Regulatory settings */
    pub nation_id: u16,

    /* Node settings */
    pub node_id: u32,
    pub node_key: [u8; 32]
}


#[unsafe(no_mangle)]
pub extern "C" fn r_driver_init(ctx: *mut c_void, polling: bool, aps_proto: bool, config: &IotdConfig) -> *mut c_void {
    let mut device = match IoTd::from_context(ctx, polling, aps_proto, config) {
        Ok(x) => x,
        Err(SX1278Error::SPIError(x)) => {
            log_alert!("Device initialization failed due to hardware error. Error {x}\n");
            return -16i32 as *mut c_void; /*-EBUSY*/
        },
        Err(SX1278Error::InvalidDevice(exp, found)) => {
            log_alert!("Device signature invalid. Expected {exp} found {found}\n");
            return -71i32 as *mut c_void; /*-EPROTO*/
        },
        Err(_) => {
            log_alert!("Device initialization failed. Unknown error.\n");
            return -22i32 as *mut c_void; /*-EINVAL*/
        }
    };

    if let Err(e) = device.config(config) {
        log_error!("Device initialization failed during post config. {:?}\n", e);
        return -22i32 as *mut c_void; /*-EINVAL*/
    };
    
    let kdevice = Allocator::kernel_alloc::<IoTd>();
    kdevice.write(device);
    let ptr = kdevice.as_mut_ptr() as *mut _ as *mut c_void;

    log_debug!("Polling mode: {}\n", polling);
    log_info!("Device driver initialized.\n");
    return ptr;
}


#[unsafe(no_mangle)]
pub extern "C" fn r_device_drop(ctx: *mut c_void) {
    let device = unsafe { &mut core::slice::from_raw_parts_mut(ctx as *mut IoTd, 1)[0] };
    let _ = device.dev_drop();
    /*device drop*/
    Allocator::kernel_free(ctx as *mut IoTd);
}

#[unsafe(no_mangle)]
pub extern "C" fn r_irq_notify(ctx: *mut c_void) -> c_int {
    let device = unsafe { &mut core::slice::from_raw_parts_mut(ctx as *mut IoTd, 1)[0] };
    if let Err(e) = device.irq(){
        log_error!("Error during irq handling. {:?}\n", e);
    }
    return 0;
}


#[unsafe(no_mangle)]
pub extern "C" fn r_config_notify(ctx: *mut c_void, new_conf: &IotdConfig) {
    let device = unsafe { &mut core::slice::from_raw_parts_mut(ctx as *mut IoTd, 1)[0] };
    if let Err(e) = device.config(new_conf){
        log_error!("Error occurred during device configuration. {:?}\n", e)
    }
}



#[unsafe(no_mangle)]
pub extern "C" fn r_dev_opened(ctx: *mut c_void, _write: bool) -> i32 {
    let device = unsafe { &mut core::slice::from_raw_parts_mut(ctx as *mut IoTd, 1)[0] };
    if let Err(e) = device.dev_opened() {
        log_error!("Cannot set the device in standby mode. {:?}\n", e)
    }
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn r_dev_close(ctx: *mut c_void) -> i32 {
    let device = unsafe { &mut core::slice::from_raw_parts_mut(ctx as *mut IoTd, 1)[0] };
    if let Err(e) = device.dev_closed() {
        log_error!("Cannot set the device in sleep mode. {:?}\n", e)
    }
    return 0;
}


#[unsafe(no_mangle)]
pub extern "C" fn r_transmit(ctx: *mut c_void, payload: *const u8, sz: usize) -> i32 {
    let device = unsafe { &mut core::slice::from_raw_parts_mut(ctx as *mut IoTd, 1)[0] };
    let buf = unsafe { core::slice::from_raw_parts(payload, sz) };
    match device.trx(buf) {
        Ok(crate::OperationResult::Success) => return sz as i32,
        Ok(crate::OperationResult::Pending) => {
            unsafe {
                __shim_mutex_lock(device.opaque_context);
                __shim_mutex_unlock(device.opaque_context);
            };
            return sz as i32;
        }
        _ => return 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn r_receive(ctx: *mut c_void, buf: *mut u8, sz: usize) -> i32 {
    let device = unsafe { &mut core::slice::from_raw_parts_mut(ctx as *mut IoTd, 1)[0] };
    let buf = unsafe { core::slice::from_raw_parts_mut(buf, sz) };
    match device.rx(buf) {
        Ok((OperationResult::Success, l)) => {
            log_info!("Returning {} after success\n", l);
            return l as i32
        },
        _ => return 0,
    }
}