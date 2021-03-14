use actix_web::web;
use crate::app::handlers::root;
use crate::app::handlers::version;
use crate::dtc::routers::dtc_routers;

pub fn app_routers(cfg: &mut web::ServiceConfig) {
    cfg.service(root::root_handler).service(
        web::scope("/v1")
            .service(version::version_one)
            .configure(dtc_routers)
    );
}
