use carrot_utils::logger;
use clap::ArgAction;
use clap::Parser;
use log::{info, LevelFilter};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};

use nvidia_checker::NvidiaEnvironment;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, action=ArgAction::SetTrue)]
    latest: bool,
    #[clap(long, default_value = None)]
    diff: Option<String>,
    #[clap(long, default_value = "Info")]
    log_level: String,
}

fn main() {
    // Arg
    let args = Args::parse();

    // Set logger
    let log_level: LevelFilter = logger::get_log_level(&args.log_level);
    let _logger = logger::Logger::new(
        "./data/{TIME_SEC}",
        "log_{TIME_SEC}.txt",
        log_level,
        log_level,
        LevelFilter::Debug,
    );

    // Read latest.toml
    let home_path = dirs::home_dir().unwrap();
    let directory_path = home_path.join(".local/share/nvidia-checker/");
    fs::create_dir_all(&directory_path).unwrap();
    let latest_file_path = directory_path.join("latest.toml");

    let mut latest_config_str = String::new();
    fs::File::open(&latest_file_path)
        .and_then(|mut f| f.read_to_string(&mut latest_config_str))
        .unwrap();
    let latest_environment: NvidiaEnvironment = toml::from_str(&latest_config_str).unwrap();

    info!("===== Start nvidia-checker =====");

    // Init Nvidia environment struct
    let now_environment = NvidiaEnvironment::init();

    // Print now environment
    info!("===== Your environment =====\n{:#?}", &now_environment,);

    // Print the check of difference from the last run
    if args.latest {
        println!("Before checked at {}", &latest_environment.checked_time);
        now_environment.print_check_results(&latest_environment);
    }

    // Print check with desirable config
    if args.diff.is_some() {
        let mut target_environment_str = String::new();
        fs::File::open(&args.diff.unwrap())
            .and_then(|mut f| f.read_to_string(&mut target_environment_str))
            .unwrap();
        let target_environment: NvidiaEnvironment =
            toml::from_str(&target_environment_str).unwrap();

        now_environment.print_check_results(&target_environment);
    }

    // Make latest.toml
    let mut latest_environment_file = fs::File::create(&latest_file_path).unwrap();
    let toml = toml::to_string(&now_environment).unwrap();
    write!(latest_environment_file, "{}", toml).unwrap();
    latest_environment_file.flush().unwrap();
}
