use swift_rs::types::SRString;

extern "C" {
    pub fn ext_get_ssid() -> SRString;
}