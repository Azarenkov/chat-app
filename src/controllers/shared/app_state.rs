use actix_web::web;

use crate::services::auth_service_abstract::AuthServiceAbstract;

pub struct AppState {
    pub auth_service: Box<dyn AuthServiceAbstract>,
}

impl AppState {
    pub fn new(auth_service: Box<dyn AuthServiceAbstract>) -> web::Data<Self> {
        web::Data::new(Self { auth_service })
    }
}
