#![feature(test)]

#[cfg(test)]
mod tests {
    use crate::entity::prelude::*;
    use crate::logger::*;
    use crate::sess::SessMgr;
    use crate::sess_impl::db_mgr::SessDbMgr;
    use log::*;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::*;
    use rust_string_random::{random, Options, RandWay};
    use tokio::test;

    use super::*;
    use anyhow::{Ok, Result};

    use std::sync::Once;

    static START: Once = Once::new();

    const DB_URL: &str = "mysql://root:123456@localhost:3306/user_db";
    async fn get_mgr() -> impl SessMgr {
        let mut mgr = SessDbMgr::New();
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

    /*
     #[tokio::test]
    async fn get_session() {
         do_init();
         let mgr = get_mgr().await;
         let user = Session {
             token: "abcdefg123456".to_owned(),
             session_info: "agagea".to_owned(),
             create_time: 0,
             update_time: 0,
             expire_time: 0,
         };

         let res = do_get_session(&mgr, &user.token).await;
         assert_eq!(res.is_err(), false);
         assert_eq!(res.unwrap(), user);
     } */

    async fn do_get_session(mgr: &impl SessMgr, token: &str) -> Result<Session> {
        debug!("get session {token:}");
        let res = mgr.get_session(token).await;
        return res;
    }

    #[tokio::test]
    async fn add_session() {
        do_init();
        let mgr = get_mgr().await;

        let options = Options {
            rand: RandWay::NORMAL,
            numbers: None,
            letters: None,
            specials: None,
        };
        let res = random(5, options);
        let rd_string = res.unwrap();
        let token = rd_string.clone();
        let session_info = token.clone() + "_session_info";

        let sess = Session {
            token: token,
            session_info: session_info,
            create_time: 0,
            update_time: 0,
            expire_time: 0,
        };

        let res = do_add_session(&mgr, &sess).await;
        debug!("add_session {res:#?}");
        assert_eq!(res.is_err(), false);

        let res = do_get_session(&mgr, &sess.token).await;
        assert_eq!(res.is_err(), false);
        let res_sess = res.unwrap();
        assert_eq!(res_sess, sess);
        debug!("query res: {res_sess:#?}");
    }
    async fn do_add_session(mgr: &impl SessMgr, sess: &Session) -> Result<()> {
        mgr.add_session(sess).await
    }
}
