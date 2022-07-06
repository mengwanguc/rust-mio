# rust_motr
The repository contains a RUST API of CORTX Motr.

# Quick Start Guide

__This guide assumes you run all the commands as root user:__ ``sudo -s``

## Get Motr sources
To use the API, you first need to compile CORTX Motr from source.

You can get the sources by running: 

``git clone --recursive https://github.com/Seagate/cortx-motr.git``

## Modify Motr source codes

Note that after downloading the motr source codes, you will have to do a few modifications to the source codes before compiling.

(The following commands assumes that you have `cortx-motr` and `rust-motr` both located in your `/root/` folder.)

* Copy ``motr-changes/client_rust`` into motr folder: 

  ``cp /root/rust_motr/motr-changes/client_rust.c /root/cortx-motr/motr/client_rust.c``


* Go to ``cortx-motr`` folder: ``cd /root/cortx-motr``

  Edit ``motr/client.h``. At the end of `client.h`, above the line ``#endif /* __MOTR_CLIENT_H__ */``, add:

  ```C
  struct read_result {
          char* data;
          size_t len;
          int rc;
  };

  int m0_init_instance(const char* ha_addr, const char* local_addr,
                  const char* profile_fid, const char* process_fid);
  struct read_result * m0_object_read(uint64_t obj_hi, uint64_t obj_low, uint64_t start, uint64_t len);
  int m0_object_create(uint64_t obj_hi, uint64_t obj_low);
  int m0_object_write(uint64_t obj_hi, uint64_t obj_low, uint64_t start, uint64_t len, char* d);
  ```

* Edit ``motr/Makefile.sub``. Search for ``motr/client.c`` (should be at line 30). Below that line, add:

  ```
                             motr/client_rust.c \
  ```

* Edit ``motr/motr-pub.api``. At the end of the file, add:

  ```
  m0_init_instance
  m0_object_read
  m0_object_create
  m0_object_write
  ```

## Compile Motr and setup cluster

Now you can follow this guide to compile Motr from source: https://github.com/Seagate/cortx-motr/blob/main/doc/Quick-Start-Guide.rst

After that, please follow this guide to compile Hare and set up Motr cluster: https://github.com/Seagate/cortx-hare/blob/main/README.md

### If you already have a running motr cluster

If you already have a running motr cluster using hare, you will first need to stop the cluster:

``hctl shutdown``

Then you can modify motr source codes. After that, run ``make`` under ``cortx-motr`` folder to recompile motr. 
Once successfully compiled, you can restart the cluster by running ``hctl bootstrap --mkfs ~/CDF.yaml`` (or use your own CDF configuration file).

## Install RUST and Cargo

``curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``

Use the default installation. After that, please run ``source $HOME/.cargo/env``.

You can run ``cargo --version`` to confirm that RUST and Cargo has been successfully installed on your machine.

## Install clang

Our RUST API requires clang version higher than clang 5. To install it, run:

```shell
sudo yum install -y centos-release-scl 
sudo yum install -y llvm-toolset-7.0
source /opt/rh/llvm-toolset-7.0/enable
```

You can check clang version by running ``clang --version``. Now ``clang version 7.0.1`` should have been installed.

## Build Motr Rust API

```shell
cd /root/rust_motr
cargo build
```

## Test Motr Rust API

The tests are written in ``src/client.rs``. Before running the tests, you first need to edit ``src/client.rs`` modify the cluster parameters in the tests.
(In ``src/client.rs``, search for ``ha_addr_str``)

For example, here is my cluster parameters after running ``hctl status``:

```shell
[root@motr rust_motr]# hctl status
Bytecount:
    critical : 0
    damaged : 0
    degraded : 0
    healthy : 0
Data pool:
    # fid name
    0x6f00000000000001:0x0 'the pool'
Profile:
    # fid name: pool(s)
    0x7000000000000001:0x0 'default': 'the pool' None None
Services:
    localhost  (RC)
    [started]  hax                 0x7200000000000001:0x0          inet:tcp:10.140.82.80@22001
    [started]  confd               0x7200000000000001:0x1          inet:tcp:10.140.82.80@21002
    [started]  ioservice           0x7200000000000001:0x2          inet:tcp:10.140.82.80@21003
    [unknown]  m0_client_other     0x7200000000000001:0x3          inet:tcp:10.140.82.80@22501
    [unknown]  m0_client_other     0x7200000000000001:0x4          inet:tcp:10.140.82.80@22502
```

Therefore, I will modify the cluster parameters in ``src/client.rs`` into:

```Rust
    static ha_addr_str: &str = "10.140.82.80@22001";
    static local_addr_str: &str = "10.140.82.80@22501";
    static profile_fid_str: &str = "0x7000000000000001:0x0";
    static process_fid_str: &str = "0x7200000000000001:0x3";
```






