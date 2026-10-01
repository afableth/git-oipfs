use std::time::Duration;
mod config;
mod vcs;

fn main() {
    let config = config::get_config().expect("failed to load config");
    let _repo = vcs::clone_if_not_exist(&config.vcs_config);
    loop {
        println!("reconciliation!");
        std::thread::sleep(Duration::from_secs(config.reconciliation_cycle * 60));
    }
}
