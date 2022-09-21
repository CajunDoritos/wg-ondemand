import SwiftRs;
import CoreWLAN;

@_cdecl("ext_get_ssid")
func getWifiSSID(interface: CWInterface) -> SRString {
    
    return SRString((interface.ssid())!);
}

@_cdecl("ext_get_interface")
func getWifiInterface() -> CWInterface {
    return(CWWiFiClient.shared().interface()!);
}
