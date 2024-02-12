use carrot_utils::command::{get_command_output, print_command_result};
use colored::*;
use log::info;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NvidiaEnvironment {
    pub checked_time: String,
    pub kernel: String,
    pub os: String,
    pub nvidia_driver: String,
    pub cuda: String,
    pub cudnn: String,
    pub tensorrt: String,
}

impl NvidiaEnvironment {
    pub fn init() -> NvidiaEnvironment {
        let now_time = chrono::Local::now().format("%Y.%m.%d %H:%M:%S").to_string();
        let os_version = get_os_version(true);
        let kernel_version = get_kernel_version(true);
        let nvidia_driver_version = get_nvidia_driver_version(true);
        let cuda_version = get_cuda_version(true);
        let cudnn_version = get_cudnn_version(true);
        let tensorrt_version = get_tensorrt_version(true);
        NvidiaEnvironment {
            checked_time: now_time,
            os: os_version,
            kernel: kernel_version,
            nvidia_driver: nvidia_driver_version,
            cuda: cuda_version,
            cudnn: cudnn_version,
            tensorrt: tensorrt_version,
        }
    }

    /// Print check results.
    /// # Arguments
    /// - target: Desirable software environment.
    pub fn print_check_results(&self, target: &NvidiaEnvironment) {
        info!("===== Check environment =====");
        print_check_result("os", &self.os, &target.os);
        print_check_result("kernel", &self.kernel, &target.kernel);
        print_check_result("nvidia driver", &self.nvidia_driver, &target.nvidia_driver);
        print_check_result("cuda", &self.cuda, &target.cuda);
        print_check_result("cudnn", &self.cudnn, &target.cudnn);
        print_check_result("tensorrt", &self.tensorrt, &target.tensorrt);
    }
}

/// Print check result.
/// # Arguments
/// - environment: The kind of environment.
/// - now_version: Software version for now.
/// - target_version: Desirable software version.
pub fn print_check_result(environment: &str, now_version: &str, target_version: &str) {
    let check_string: ColoredString;
    if now_version == target_version {
        check_string = "OK".green();
    } else {
        check_string = "NG".red();
    }
    info!("{}: {}", environment, check_string);
}

/// Get Kernel version.
/// # Arguments
/// - print_terminal: If this parameter is true, print command output to terminal.
pub fn get_kernel_version(print_terminal: bool) -> String {
    let command = "uname -r";
    let command_output = get_command_output(command);
    if print_terminal == true {
        print_command_result(command, &command_output);
    }

    command_output.to_string().replace("\n", "")
}

/// Get OS version.
/// # Arguments
/// - print_terminal: If this parameter is true, print command output to terminal.
pub fn get_os_version(print_terminal: bool) -> String {
    let command = "cat /etc/os-release";
    let command_output = get_command_output(command);
    if print_terminal == true {
        print_command_result(command, &command_output);
    }

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
/// - print_terminal: If this parameter is true, print command output to terminal.
pub fn get_nvidia_driver_version(print_terminal: bool) -> String {
    let command = "cat /proc/driver/nvidia/version";
    let command_output = get_command_output(command);
    if print_terminal == true {
        print_command_result(command, &command_output);
    }

    for string_line in command_output.lines() {
        let mut string_array: Vec<&str> = string_line.split(" ").collect();
        string_array.retain(|&x| x != "");
        return string_array[7].to_string();
    }
    return "".to_string();
}

/// Get CUDA version.
/// # Arguments
/// - print_terminal: If this parameter is true, print command output to terminal.
pub fn get_cuda_version(print_terminal: bool) -> String {
    let command = "nvcc -V";
    let command_output = get_command_output(command);
    if print_terminal == true {
        print_command_result(command, &command_output);
    }

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

/// Get cuDNN version.
/// # Arguments
/// - print_terminal: If this parameter is true, print command output to terminal.
pub fn get_cudnn_version(print_terminal: bool) -> String {
    let command = "dpkg -l | grep cudnn";
    let command_output = get_command_output(command);
    if print_terminal == true {
        print_command_result(command, &command_output);
    }
    get_dpkg_version(&command_output, "libcudnn8")
}

/// Get TensorRT version.
/// # Arguments
/// - print_terminal: If this parameter is true, print command output to terminal.
pub fn get_tensorrt_version(print_terminal: bool) -> String {
    let command = "dpkg -l | grep TensorRT";
    let command_output = get_command_output(command);
    if print_terminal == true {
        print_command_result(command, &command_output);
    }
    get_dpkg_version(&command_output, "libnv")
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
