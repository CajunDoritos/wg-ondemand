use swift_rs::types::SRString;
use swift_rs::types::SRObject;

#[repr(C)]
pub struct CWInterface {
    interface_name: SRString
}

extern "C" {
    pub fn ext_get_ssid(interface: SRObject<CWInterface>) -> SRString;
    pub fn ext_get_interface() -> SRObject<CWInterface>;
}