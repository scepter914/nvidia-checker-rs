use carrot_utils::logger;
use clap::ArgAction;
use clap::Parser;
use colored::*;
use log::{info, LevelFilter};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, action=ArgAction::SetTrue)]
    latest: bool,
    #[clap(long, default_value = None)]
    diff: Option<String>,
    #[clap(long, default_value = "Info")]
    log_level: String,
}

/// Print check result.
/// # Arguments
/// - environment: The kind of environment.
/// - now_version: Software version for now.
/// - target_version: Desirable software version.
fn print_check_result(environment: &str, now_version: &str, target_version: &str) {
    let check_string: ColoredString;
    if now_version == target_version {
        check_string = "OK".green();
    } else {
        check_string = "NG".red();
    }
    info!("{}: {}", environment, check_string);
}

#[derive(Debug, Deserialize, Serialize)]
struct Environment {
    checked_time: String,
    os: String,
    kernel: String,
    nvidia_driver: String,
    cuda: String,
    cudnn: String,
    tensorrt: String,
}

impl Environment {
    /// Print check results.
    /// # Arguments
    /// - target: Desirable software environment.
    pub fn print_check_results(&self, target: &Environment) {
        info!("===== Check environment =====");
        print_check_result("os", &self.os, &target.os);
        print_check_result("kernel", &self.kernel, &target.kernel);
        print_check_result("nvidia driver", &self.nvidia_driver, &target.nvidia_driver);
        print_check_result("cuda", &self.cuda, &target.cuda);
        print_check_result("cudnn", &self.cudnn, &target.cudnn);
        print_check_result("tensorrt", &self.tensorrt, &target.tensorrt);
    }
}

/// Get command output.
/// # Arguments
/// - command: The input command
fn get_command_output(command: &str) -> String {
    let args_list = ["-c", command];
    let command_output = Command::new("sh")
        .args(&args_list)
        .output()
        .expect("failed command");
    String::from_utf8_lossy(&command_output.stdout).to_string()
}

/// Print command result.
/// # Arguments
/// - command: The input command
/// - command_output: The output of command
fn print_command_result(command: &str, command_output: &str) {
    info!("command: {}\n\n{}", command, command_output);
}

/// Get OS version.
/// # Arguments
/// - command_output: The output of command
fn get_os_version(command_output: &str) -> String {
    for string_line in command_output.lines() {
        if string_line.contains("PRETTY_NAME") {
            let string_array: Vec<&str> = string_line.split("=").collect();
            let version = string_array[1].to_string().replace("\"", "");
            return version;
        }
    }
    return "".to_string();
}

/// Get NVIDIA driver version.
/// # Arguments
/// - command_output: The output of command
fn get_nvidia_driver_version(command_output: &str) -> String {
    for string_line in command_output.lines() {
        let mut string_array: Vec<&str> = string_line.split(" ").collect();
        string_array.retain(|&x| x != "");
        return string_array[7].to_string();
    }
    return "".to_string();
}

/// Get CUDA version.
/// # Arguments
/// - command_output: The output of command
fn get_cuda_version(command_output: &str) -> String {
    for string_line in command_output.lines() {
        if string_line.contains("Build") {
            // "Build cuda_12.3.r12.3/compiler.33567101_0" -> ["Build", "cuda_12.3.r12.3/compiler.33567101_0"]
            let temp_string_array: Vec<&str> = string_line.split(" ").collect();
            // "cuda_12.3.r12.3/compiler.33567101_0" -> ["cuda_12.3.r12.3", "compiler.33567101_0"]
            let temp_string_array_2: Vec<&str> = temp_string_array[1].split("/").collect();
            let version = temp_string_array_2[0].to_string();
            return version;
        }
    }
    return "".to_string();
}

/// Get version string with dpkg command.
/// # Arguments
/// - command_output: The output of command
/// - pattern: Target package name
fn get_dpkg_version(command_output: &str, pattern: &str) -> String {
    let mut version_list: Vec<&str> = vec![];
    for string_line in command_output.lines() {
        if string_line.contains(pattern) {
            // hi, libcudnn8 8.9.5.29-1+cuda12.2, amd64, cuDNN runtime libraries
            let mut temp_string_array: Vec<&str> = string_line.split(" ").collect();
            temp_string_array.retain(|&x| x != "");
            version_list.push(temp_string_array[2]);
        }
    }
    version_list.sort();
    version_list.dedup();
    version_list.join(",")
}

fn main() {
    info!("===== Start nvidia-checker =====");

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

    // Time
    let now_time = chrono::Local::now().format("%Y.%m.%d %H:%M:%S").to_string();

    // Linux kernel
    let command = "uname -r";
    let command_output = get_command_output(command);
    let kernel_version = command_output.to_string().replace("\n", "");
    print_command_result(command, &command_output);

    // OS
    let command = "cat /etc/os-release";
    let command_output = get_command_output(command);
    let os_version = get_os_version(&command_output);
    print_command_result(command, &command_output);

    // Nvidia driver
    let command = "cat /proc/driver/nvidia/version";
    let command_output = get_command_output(command);
    let nvidia_driver_version = get_nvidia_driver_version(&command_output);
    print_command_result(command, &command_output);

    // CUDA
    let command = "nvcc -V";
    let command_output = get_command_output(command);
    let cuda_version = get_cuda_version(&command_output);
    print_command_result(command, &command_output);

    // cuDNN
    let command = "dpkg -l | grep cudnn";
    let command_output = get_command_output(command);
    let cudnn_version = get_dpkg_version(&command_output, "libcudnn8");
    print_command_result(command, &command_output);

    // TensorRT
    let command = "dpkg -l | grep TensorRT";
    let command_output = get_command_output(command);
    let tensorrt_version = get_dpkg_version(&command_output, "libnv");
    print_command_result(command, &command_output);

    let now_environment = Environment {
        checked_time: now_time,
        os: os_version,
        kernel: kernel_version,
        nvidia_driver: nvidia_driver_version,
        cuda: cuda_version,
        cudnn: cudnn_version,
        tensorrt: tensorrt_version,
    };

    // Read latest.toml
    let home_path = dirs::home_dir().unwrap();
    let directory_path = home_path.join(".local/share/nvidia-checker/");
    fs::create_dir_all(&directory_path).unwrap();
    let latest_file_path = directory_path.join("latest.toml");

    let mut latest_config_str = String::new();
    fs::File::open(&latest_file_path)
        .and_then(|mut f| f.read_to_string(&mut latest_config_str))
        .unwrap();
    let latest_environment: Environment = toml::from_str(&latest_config_str).unwrap();

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
        let target_environment: Environment = toml::from_str(&target_environment_str).unwrap();

        now_environment.print_check_results(&target_environment);
    }

    // Make latest.toml
    let mut latest_environment_file = fs::File::create(&latest_file_path).unwrap();
    let toml = toml::to_string(&now_environment).unwrap();
    write!(latest_environment_file, "{}", toml).unwrap();
    latest_environment_file.flush().unwrap();
}
