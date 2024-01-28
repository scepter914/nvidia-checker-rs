# nvidia-checker-rs

The crate `nvidia-checker` is a checker CLI tool for nvidia software environment.
It can be used when developers want to use the same environment of nvidia software.

- Supported feature
  - [ x ] Check with specified version
  - [  ] Check with specified version in range
- Document
  - [crates.io](https://crates.io/crates/nvidia-checker)
  - [docs.rs](https://docs.rs/nvidia-checker)

## Get started
### install

- Install `cargo`
  - Please see [document](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Install `nvidia-checker`

```
cargo install nvidia-checker
```

### Print environment

```sh
nvidia-checker
```

### Check with desirable environment

- See [example config](./config/test.toml)

```sh
nvidia-checker --diff ./config/test.toml
```

- Output example

```
17:00:39 [INFO] ===== Start nvidia-checker =====
17:00:39 [INFO] command: uname -r

5.15.0-58-generic

17:00:39 [INFO] command: cat /etc/os-release

PRETTY_NAME="Ubuntu 22.04.1 LTS"
NAME="Ubuntu"
VERSION_ID="22.04"
VERSION="22.04.1 LTS (Jammy Jellyfish)"
VERSION_CODENAME=jammy
ID=ubuntu
ID_LIKE=debian
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
UBUNTU_CODENAME=jammy

17:00:39 [INFO] command: cat /proc/driver/nvidia/version

NVRM version: NVIDIA UNIX x86_64 Kernel Module  545.23.08  Mon Nov  6 23:49:37 UTC 2023
GCC version:  gcc version 11.4.0 (Ubuntu 11.4.0-1ubuntu1~22.04) 

17:00:39 [INFO] command: nvcc -V

nvcc: NVIDIA (R) Cuda compiler driver
Copyright (c) 2005-2023 NVIDIA Corporation
Built on Wed_Nov_22_10:17:15_PST_2023
Cuda compilation tools, release 12.3, V12.3.107
Build cuda_12.3.r12.3/compiler.33567101_0

17:00:39 [INFO] command: dpkg -l | grep cudnn

hi  libcudnn8                                         8.9.5.29-1+cuda12.2                     amd64        cuDNN runtime libraries
hi  libcudnn8-dev                                     8.9.5.29-1+cuda12.2                     amd64        cuDNN development libraries and headers
ii  ros-humble-cudnn-cmake-module                     0.0.1-3jammy.20230112.141335            amd64        Exports a CMake module to find cuDNN.

17:00:39 [INFO] command: dpkg -l | grep TensorRT

ii  libnvinfer-bin                                    8.6.1.6-1+cuda12.0                      amd64        TensorRT binaries
hi  libnvinfer-dev                                    8.6.1.6-1+cuda12.0                      amd64        TensorRT development libraries
ii  libnvinfer-dispatch8                              8.6.1.6-1+cuda12.0                      amd64        TensorRT dispatch runtime library
ii  libnvinfer-headers-dev                            8.6.1.6-1+cuda12.0                      amd64        TensorRT development headers
ii  libnvinfer-headers-plugin-dev                     8.6.1.6-1+cuda12.0                      amd64        TensorRT plugin headers
ii  libnvinfer-lean8                                  8.6.1.6-1+cuda12.0                      amd64        TensorRT lean runtime library
hi  libnvinfer-plugin-dev                             8.6.1.6-1+cuda12.0                      amd64        TensorRT plugin libraries
hi  libnvinfer-plugin8                                8.6.1.6-1+cuda12.0                      amd64        TensorRT plugin libraries
ii  libnvinfer-vc-plugin8                             8.6.1.6-1+cuda12.0                      amd64        TensorRT vc-plugin library
hi  libnvinfer8                                       8.6.1.6-1+cuda12.0                      amd64        TensorRT runtime libraries
hi  libnvonnxparsers-dev                              8.6.1.6-1+cuda12.0                      amd64        TensorRT ONNX libraries
hi  libnvonnxparsers8                                 8.6.1.6-1+cuda12.0                      amd64        TensorRT ONNX libraries
hi  libnvparsers-dev                                  8.6.1.6-1+cuda12.0                      amd64        TensorRT parsers libraries
hi  libnvparsers8                                     8.6.1.6-1+cuda12.0                      amd64        TensorRT parsers libraries
ii  ros-humble-tensorrt-cmake-module                  0.0.3-1jammy.20230112.141218            amd64        Exports a CMake module to find TensorRT.

17:00:39 [INFO] ===== Your environment =====
Environment {
    checked_time: "2024.01.27 02:00:39",
    os: "Ubuntu 22.04.1 LTS",
    kernel: "5.15.0-58-generic",
    nvidia_driver: "545.23.08",
    cuda: "cuda_12.3.r12.3",
    cudnn: "8.9.5.29-1+cuda12.2",
    tensorrt: "8.6.1.6-1+cuda12.0",
}
17:00:39 [INFO] ===== Check environment =====
17:00:39 [INFO] os: OK
17:00:39 [INFO] kernel: OK
17:00:39 [INFO] nvidia driver: OK
17:00:39 [INFO] cuda: OK
17:00:39 [INFO] cudnn: OK
17:00:39 [INFO] tensorrt: OK
```

### Check with the change from latest check

- If you want to check with the change for NVIDIA software environment as automatic `apt update`, you can use `nvidia-checker` as below.
  - The environment file at the last `nvidia-checker` running is at `$HOME/.local/nvidia-checker/latest.toml`.

```sh
nvidia-checker --latest
```

### Run from source files

- Clone repository

```sh
git clone https://github.com/scepter914/nvidia-checker-rs.git
```

- Run

```sh
cargo run --release --bin nvidia-checker -- --diff ./config/test.toml
```

## History

- Next release
  - Update README
- v0.1.0
  - Published crate
