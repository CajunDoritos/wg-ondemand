use swift_rs::types::*;

use crate::bindings::*;

pub fn get_ssid(interface: SRObject<CWInterface>) -> String {
   let sr_ssid: SRString = unsafe { ext_get_ssid(interface) };
   String::from(sr_ssid.as_str())
}

pub fn get_interface() -> SRObject<CWInterface> {
   unsafe { ext_get_interface() }
}