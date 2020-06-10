mod gaze;
mod actor;

use std::thread;
use futures::future::{join_all};

#[tokio::main]
async fn main() {
    for i in 1..2000 {
        tokio::spawn(actor::run());
    }

    thread::park();
    ()
}