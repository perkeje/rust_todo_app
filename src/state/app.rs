use std::sync::Arc;
use crate::state::pool;
use diesel::r2d2::{PooledConnection,ConnectionManager};
use diesel::pg::PgConnection;



pub struct StaticData{
    pub db: pool::DBPool
}


#[derive(Clone)]
pub struct AppState{
    pub static_data: Arc<StaticData>
}

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

impl AppState{
    pub fn get_connectinon(&self) ->  Connection{
        self.static_data.db.get().expect("Cannot get connection to the pool.")
    }
}

pub fn initialize_pool() -> AppState{
    AppState{
        static_data: Arc::new(StaticData{db: pool::connect_pool()})
    }
}

