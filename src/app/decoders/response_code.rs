pub fn get(code: u8) -> String {
    if code >= 10 && code <= 19 {
        String::from("SDU Initialization (SDx) commands")
    } else if code >= 30 && code <= 39 {
        String::from("Initialization (PCx) commands")
    } else if code >= 100 && code <= 109 {
        String::from("Data Acquisition (ADx) commands")
    } else if code >= 120 && code <= 129 {
        String::from("Calibration (CAx) commands")
    } else if code >= 130 && code <= 139 {
        String::from("Calibration (CAx) commands")
    } else if code >= 140 && code <= 149 {
        String::from("Valve Control (CVx) commands")
    } else if code >= 150 && code <= 159 {
        String::from("Look-At (LAx) commands")
    } else if code >= 160 && code <= 169 {
        String::from("Set Calibration Parameter (CPx) commands")
    } else if code >= 180 && code <= 189 {
        String::from("System Communication (SCx) commands")
    } else if code == 250 {
        String::from("0xFA Data Acquisition (DA) Stream Data Packet (measurement set)")
    } else if code == 251 {
        String::from("0xFB Data Acquisition (DA) Stream Compensation Data Packet (comp-set)")
    } else {
        String::from("Response code is unknown")
    }
}
