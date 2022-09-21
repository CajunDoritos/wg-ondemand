import SwiftRs;
import CoreWLAN;

@_cdecl("ext_get_ssid")
func getSSID() -> SRString {
    let defaultInterface = CWWiFiClient.shared().interface();
    
    return SRString((defaultInterface?.ssid())!);
}
