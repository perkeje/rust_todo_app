use diesel::pg::PgConnection;
use diesel::r2d2::{Pool,ConnectionManager};

use std::env;
use dotenv::dotenv;

    
    pub type DBPool = Pool<ConnectionManager<PgConnection>>;
    
    
    pub fn connect_pool() -> DBPool{
        dotenv().ok();
        
        let manager = ConnectionManager::new(env::var("DATABASE_URL").expect("No DATABASE_URL in .env file"));
        
        Pool::builder()
        .build(manager)
        .expect("Unable to connect to database pool.")
    }

 