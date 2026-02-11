use crate::database::database::Database;

#[derive(Clone)]
pub struct AppContext {
    pub db: Database,
}
