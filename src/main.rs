use std::{
    // env,
    sync::{Arc, Mutex},
};

use log::info;

mod controllers;
mod models;
mod routers;
pub mod config;
use routers::router::get_router;
use models::mysql_conn_pool::init as db_init;
use sqlx;
use lazy_static::lazy_static;
use crate::models::mysql_conn_pool::get_sqlite;

lazy_static! {
    pub static ref GLOBAL_CONFIG:config::conf::GlobalConfig=config::conf::load_global_config().unwrap();
}
async fn  init(){
    db_init().await;
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    init().await;
    
    sqlx::query!("insert into users (name) values(?)","name".to_string()).execute(get_sqlite()).await?;
    //计数器
    let count = Arc::new(Mutex::new(0u32));
    //日志设置
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let conf = &GLOBAL_CONFIG;
    // info!("conf {:#?}",conf.mysql);
    info!("starting HTTP server at http://localhost:{}",conf.server.port);
    //获取路由
    let route = get_router(count);
    
    //启动服务器
    warp::serve(route).run(([127, 0, 0, 1], conf.server.port)).await;
    Ok(())
}

// async fn testdb(pool: &MySqlPool) -> anyhow::Result<()> {
//     for i in 0..10 {
//         sqlx::query!(
//             "insert into users (name) values(?)",
//             "name".to_string() + &i.to_string()
//         )
//         .execute(pool)
//         .await?;
//     }

//     let recs = sqlx::query!(" SELECT * FROM users").fetch_all(pool).await?;
//     for rec in recs {
//         println!("- [{}] {}:{:?}", rec.id, rec.name.unwrap(), rec.created_at);
//     }

//     Ok(())
// }

// async fn test(pool: &MySqlPool) {
//     let s = pool.clone();
//     let a = tokio::spawn(async {
//         {
//             for i in 0..100 {
//                 println!("第一个携程{}", i)
//             }
//         }
//     });
//     let c = tokio::spawn(async {
//         {
//             for i in 0..100 {
//                 println!("第二个携程{}", i)
//             }
//         }
//     });
//     let d = tokio::spawn(async {
//         {
//             for i in 0..100 {
//                 println!("第三·个携程{}", i)
//             }
//         }
//     });
//     let b = tokio::spawn(async move { testdb(&s).await });
//     tokio::join!(a, c, d, b);
// }
