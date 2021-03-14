use actix_web::web;
use crate::dtc::handlers::dtc::dtc_handler;


pub fn dtc_routers(cfg: &mut web::ServiceConfig) {
    cfg.service(dtc_handler);
}
