pub fn get(code: u8) -> String {
    match code {
        4 => String::from(
            "Confirmation packet: indicates that command was successfully executed without error"
        ),
        8 => String::from(
            "Packet contains a Single Value as a long integer number"
        ),
        9 => String::from(
            "Packet contains a Single Value as an IEEE Floating point number"
        ),
        16 => String::from(
            "Packet contains Stream Data with raw 2-byte binary numbers"
        ),
        17 => String::from(
            "Packet contains Stream Data with raw 3-byte binary numbers"
        ),
        19 => String::from(
            "Packet contains Stream Data with EU 4-byte IEEE Floating point numbers"
        ),
        33 => String::from(
            "Packet contains Array Data as a 4-byte IEEE Floating point number"
        ),
        128 => String::from(
            "Error Packet: indicates error was encountered when executing command"
        ),
        _ => String::from("Response type is unknown")
    }
}
