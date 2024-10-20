use log4rs;

pub fn init_log() {
    log4rs::init_file(
        "/Users/jimxfzhu/project/rust/user_admin/src/config/log4rs.yml",
        Default::default(),
    )
    .unwrap();
}
