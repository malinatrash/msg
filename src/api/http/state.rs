use crate::usecase::Service;

#[derive(Clone)]
pub struct AppState {
    pub uc: Service,
}

impl AppState {
    pub fn new(uc: Service) -> Self {
        Self { uc }
    }
}
