use std::{io::Write, sync::{Mutex, mpsc, Arc}, fs::{self, File, OpenOptions}, thread, time::Duration};
use daemonize::Daemonize;
use whoami;

mod daemon;
mod task;

fn main() {
    daemon::start();
}

