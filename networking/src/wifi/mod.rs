use swift_rs::types::SRString;

use crate::bindings::*;

pub fn get_ssid() -> String {
   let sr_ssid: SRString = unsafe { ext_get_ssid() };
   String::from(sr_ssid.as_str())
}