pub fn get(code: i32) -> String {
    match code {
        -3 => String::from("Illegal Command ID"),
        -5 => String::from("Parameter Is Missing"),
        -6 => String::from("Value Too Low"),
        -7 => String::from("Value Too High"),
        -10 => String::from("Upper Val < Low Val"),
        -11 => String::from("Bad Name For Parameter"),
        -12 => String::from("Need Integer Number"),
        -13 => String::from("Need Float Point"),
        -14 => String::from("Illegal CRS Number"),
        -18 => String::from("Bad Sensor Port Number"),
        -19 => String::from("Bad Upper Port Number"),
        -20 => String::from("Upper Port Number < Lower Number"),
        -21 => String::from("Bad Scanner Number"),
        -25 => String::from("Bad Upper Scanner Number"),
        -27 => String::from("Bad Logical Range"),
        -32 => String::from("Too Many Parameters"),
        -39 => String::from("Non-Vol. Mem. Error"),
        -53 => String::from("No Module This CRS"),
        -68 => String::from("Port Not Defined in Scan Table"),
        -69 => String::from("Port Not Defined"),
        -75 => String::from("SDU Table Not Defined"),
        -80 => String::from("DATA Not Acquired"),
        -82 => String::from("Data Acquisition Aborted"),
        - 230 => String::from("Module Not Ready"),
        _ => String::from("Response code is unknown")
    }
}
