use actix_web::web;
use crate::app::handlers::root::get_root;
use crate::app::handlers::firmware::get_firmware;
use crate::app::handlers::connscanner::connected_scanners;
use crate::app::handlers::daqparams::daq_params;
use crate::app::handlers::scanlist::scan_list;
use crate::app::handlers::eeprom::eeprom_coef;
use crate::app::handlers::changeunit::set_unit;
use crate::app::handlers::scannerstatus::get_status;
use crate::app::handlers::tablecoef::save_coef;
use crate::app::handlers::rezero::calib_rezero;
use crate::app::handlers::startstream::start_stream;


pub fn app_routers(router: &mut web::ServiceConfig) {
    router.service(get_root)
        .service(get_firmware)
        .service(connected_scanners)
        .service(daq_params)
        .service(scan_list)
        .service(eeprom_coef)
        .service(set_unit)
        .service(get_status)
        .service(save_coef)
        .service(calib_rezero)
        .service(start_stream);
}
