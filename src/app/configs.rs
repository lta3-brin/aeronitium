use std::env;


#[derive(Debug)]
pub struct AppEnvironment {
    server_address: String,
    dtc_address: String,
    nats_address: String
}

impl AppEnvironment {
    pub fn new(
        server_address: String,
        dtc_address: String,
        nats_address: String
    ) -> Self {
        Self { server_address, dtc_address, nats_address }
    }

    pub fn get_server_addr(&self) -> String {
        self.server_address.clone()
    }

    pub fn get_dtc_addr(&self) -> String {
        self.dtc_address.clone()
    }

    pub fn get_nats_addr(&self) -> String {
        self.nats_address.clone()
    }
}

pub fn get_configs() -> AppEnvironment {
    let srv = match env::var("SERVER_ADDRESS") {
        Ok(s) => s,
        Err(_) => String::from("0.0.0.0:8080")
    };

    let dtc = match env::var("DTC_ADDRESS") {
        Ok(s) => s,
        Err(_) => String::from("192.168.129.119:8400")
    };

    let nats = match env::var("NATS_ADDRESS") {
        Ok(s) => s,
        Err(_) => String::from("127.0.0.1:4222")
    };

    AppEnvironment::new(srv, dtc, nats)
}
