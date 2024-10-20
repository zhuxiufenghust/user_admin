use crate::entity::{prelude::*, user_tb};
use crate::user::*;
use anyhow::anyhow;
use anyhow::Result;
use log::*;
use sea_orm::*;
use sea_orm::{Database, EntityTrait};
use std::rc::Rc;

#[derive(Debug)]
pub struct UserDbMgr {
    db: Option<Rc<DatabaseConnection>>,
}

impl UserDbMgr {
    pub fn New() -> UserDbMgr {
        let mgr = UserDbMgr { db: None };
        return mgr;
    }
    pub async fn initialze(&mut self, str: &str) -> anyhow::Result<()> {
        let db: DatabaseConnection = Database::connect(str).await.unwrap();
        self.db = Some(Rc::new(db));
        return Ok(());
    }
}

impl UserMgr for UserDbMgr {
    async fn get_user(&self, user_id: u64) -> Result<User> {
        let db = self.db.as_ref().unwrap();
        let ndb = Rc::clone(db);
        let res = UserTb::find_by_id(user_id).one(ndb.as_ref()).await?;
        match res {
            Some(t) => Ok(t),
            None => {
                let err = anyhow!("no match res");
                return Err(err);
            }
        }
    }

    async fn get_user_with_name(&self, name: &str) -> Result<User> {
        let db = self.db.as_ref().unwrap();
        let ndb = Rc::clone(db);
        let mut cond = Condition::all();
        cond.add(user_tb::Column::Name.eq(name));
        let res = user_tb::Entity::find()
            .filter(user_tb::Column::Name.eq(name.to_owned()))
            .one(ndb.as_ref())
            .await?;
        match res {
            Some(t) => Ok(t),
            None => {
                let err = anyhow!("no match res");
                return Err(err);
            }
        }
    }

    async fn add_user(&self, user: &User) -> Result<()> {
        let db = self.db.as_ref().unwrap();
        let ndb = Rc::clone(db);
        debug!("add user with user: {user:#?}");
        let res = user_tb::Entity::insert(user.clone().into_active_model())
            .exec_without_returning(ndb.as_ref())
            .await;
        debug!("add user with user res: {res:#?}");
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                match e {
                    DbErr::Exec(RuntimeErr::SqlxError(e)) => match e {
                        sqlx::Error::Database(e) => {
                            // We check the error code thrown by the database (MySQL in this case),
                            // `23000` means `ER_DUP_KEY`: we have a duplicate key in the table.
                            //assert_eq!(e.code().unwrap(), "23000");
                            if e.code().unwrap() == "23000" {
                                debug!("insert duplicate {user:#?}");
                                Ok(())
                            } else {
                                Err(e.into())
                            }
                        }
                        _ => Err(e.into()),
                    },
                    _ => Err(e.into()),
                }
            }
        }
    }
}
