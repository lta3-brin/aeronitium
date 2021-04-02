use std::env;
use crate::app::AppError;


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

pub fn get_configs() -> Result<AppEnvironment, AppError> {
    let srv = env::var("APP_ADDRESS")?;
    let dtc = env::var("DTC_ADDRESS")?;
    let nats = env::var("NATS_ADDRESS")?;

    Ok(AppEnvironment::new(srv, dtc, nats))
}
