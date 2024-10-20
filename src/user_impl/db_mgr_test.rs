#![feature(test)]

#[cfg(test)]

pub fn add_two(a: i32) -> i32 {
    a + 2
}
mod tests {
    use crate::entity::prelude::*;
    use crate::logger::*;
    use crate::user::UserMgr;
    use crate::user_impl::db_mgr::UserDbMgr;
    use log::*;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::*;
    use tokio::test;

    use super::*;

    use std::sync::Once;

    static START: Once = Once::new();

    const DB_URL: &str = "mysql://root:123456@localhost:3306/user_db";
    async fn get_mgr() -> impl UserMgr {
        let mut mgr = UserDbMgr::New();
        let res = mgr.initialze(DB_URL).await;
        debug!("db_connect: {:#?}, url:{}, res:{:#?}", mgr, DB_URL, res);
        return mgr;
    }
    fn do_init() {
        START.call_once(|| {
            // run initialization here
            init_log();
        });
    }

    #[tokio::test]
    async fn add_user() {
        do_init();
        let mgr = get_mgr().await;
        do_add_user(&mgr).await;
        do_get_user(&mgr).await;
    }

    async fn do_add_user(mgr: &impl UserMgr) {
        let user = User {
            id: 2,
            name: "zhuxiufenghust".to_owned(),
            passwd: "zhuxiufenghust".to_owned(),
            create_time: 0,
            update_time: 0,
        };
        debug!("add_user: {user:?}");
        let res = mgr.add_user(&user).await;
        info!("{res:?}");
        assert_eq!(res.is_err(), false);
    }

    #[tokio::test]
    async fn get_user() {
        do_init();
        let mgr = get_mgr().await;
        do_get_user(&mgr);
    }

    async fn do_get_user(mgr: &impl UserMgr) {
        let user = User {
            id: 12,
            name: "jimxfzhu".to_owned(),
            passwd: "jimxfzhu".to_owned(),
            create_time: 0,
            update_time: 0,
        };
        let res = mgr.get_user(1).await;
        assert_eq!(res.is_err(), false);
        assert_eq!(res.unwrap(), user);
    }
}
