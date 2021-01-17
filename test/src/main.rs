#[macro_use]
extern crate log;
extern crate log4rs;

use std::default::Default;
use std::thread;
use std::time::Duration;

use log::{error, info, warn};
//use log4rs;

fn main() {
    log4rs::init_file("log.yml", Default::default()).unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));
        warn!("main");
        error!("error main");
        a::foo();
        b::foo();
    }
}

mod a {
    pub fn foo() {
        info!("a");
    }
}

mod b {
    pub fn foo() {
        info!("1 aaa bbb ccc");
        info!("2 aaa ccc");
        debug!("3 aaa bbb ccc");
    }
}
