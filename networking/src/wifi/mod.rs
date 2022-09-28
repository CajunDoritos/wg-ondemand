use crate::platform::mac::wifi::*;

pub fn get_ssid() -> String {
   mac_get_ssid(mac_get_interface())
}