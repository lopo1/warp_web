use anyhow::Ok;
use sqlx::MySqlPool;
use once_cell::sync::OnceCell;

pub static GLOBAL_DB: OnceCell<MySqlPool> = OnceCell::new();
type DbConnPool = MySqlPool;
pub fn get_sqlite<'a>() -> &'a DbConnPool {
    GLOBAL_DB.get().unwrap()
}
pub async fn init() {
    let conf = &crate::GLOBAL_CONFIG;
    let url = format!("mysql://{}:{}@{}:{}/{}", conf.mysql.username, conf.mysql.password, conf.mysql.host,conf.mysql.port,conf.mysql.db_name); 
    let pool = MySqlPool::
    connect(&url)
    .await
    .expect("Init datasource failed.");
    Ok(GLOBAL_DB.set(pool));
   
}
