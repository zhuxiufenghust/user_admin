use crate::entity::sess_tb::{ActiveModel, Session};
use crate::entity::{prelude::*, sess_tb};
use crate::sess::SessMgr;
use anyhow::anyhow;
use anyhow::Result;
use log::*;
//use sea_orm::InsertOption;
use sea_orm::*;
use std::rc::Rc;
use std::time::*;

#[derive(Debug)]
pub struct SessDbMgr {
    db: Option<Rc<DatabaseConnection>>,
}

impl SessDbMgr {
    pub fn New() -> SessDbMgr {
        let mgr = SessDbMgr { db: Option::None };
        return mgr;
    }
    pub async fn initialze(&mut self, str: &str) -> anyhow::Result<()> {
        let mut opt = ConnectOptions::new(str);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(true);

        let db = Database::connect(opt).await?;
        self.db = Some(Rc::new(db));
        return Ok(());
    }
}

impl SessMgr for SessDbMgr {
    async fn get_session(&self, id: &str) -> Result<Session> {
        let db = self.db.as_ref().unwrap();
        let ndb = Rc::clone(db);
        let res = SessTb::find_by_id(id).one(ndb.as_ref()).await?;

        match res {
            Some(t) => Ok(t),
            None => {
                let err = anyhow!("no match res");
                return Err(err);
            }
        }
    }
    async fn add_session(&self, sess: &Session) -> Result<()> {
        let db = self.db.as_ref().unwrap();
        let ndb = Rc::clone(db);
        let nss = sess.clone().into_active_model();
        debug!("add session with session: {nss:#?}");
        //let res = nss.insert(ndb.as_ref()).await;
        let res = sess_tb::Entity::insert(nss)
            .exec_without_returning(ndb.as_ref())
            .await;
        debug!("add session with session res: {res:#?}");
        match res {
            Ok(_) => Ok(()),
            Err(e) => match e {
                DbErr::Exec(RuntimeErr::SqlxError(e)) => match e {
                    sqlx::Error::Database(e) => {
                        debug!("add_session_res err----- {:#?}", e);
                        if e.code().unwrap() == "23000" {
                            debug!("insert duplicate {sess:#?}");
                            Ok(())
                        } else {
                            Err(e.into())
                        }
                    }
                    _ => {
                        debug!("add_session_res err----- {:#?}", e);
                        Err(e.into())
                    }
                },
                _ => {
                    debug!("add_session_res err----- {:#?}", e);
                    Err(e.into())
                }
            },
        }
    }
}
