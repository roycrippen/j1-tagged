# J1 CPU emulator

[J1](https://excamera.com/sphinx/fpga-j1.html) CPU emulator written in Rust. <br>
Ported from [j1](https://github.com/dim13/j1) written in go. <br>
requires [rustup](https://rustup.rs/) <br>

### build and install
```shell
$ cargo install --path . 
```
### executables
Help with executable arguments `<executable> -h` or `<executable> --help` <br>

| Name                        |Description |
| :-------------------------  | :------ |
| j1_tagged                          | j1 emulator |
| j1_tagged_dump                     | dump j1 cpu memory in assembly or instruction AST format |

### test
```shell
$ cargo test
```

### document
```shell
$ cargo doc
# open j1-cpu/target/doc/j1/index.html with browser
```

### run j1_tagged eforth repl
```shell
# option -r or --repl
$ j1_tagged --repl
```

### run j1_tagged eforth repl with a script
```shell
# from j1-tagged directory
$ j1_tagged --repl --script resources/simple.4th
```

### j1_tagged options
```shell
# help -h or --help
# Note: results saved to <script_file>-log.txt if not running repl
$ j1_tagged -h

j1_tagged 1.0
Roy Crippen
J1_tagged cpu emulator

USAGE:
    j1_tagged [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -r, --repl       Run the J1 forth repl
    -V, --version    Prints version information

OPTIONS:
    -b, --bin <bin_file>          Binary J1 forth image to load, default is j1e_bytes::J1E_BYTES
    -s, --script <script_file>    Forth script file to load and execute
```


### todo
| Task                           | Done |
| :---------------------------   | :------: |
| stack                          | &#x2714; |
| instructions                   | &#x2714; |
| cpu                            | &#x2714; |
| dump memory to asm and ast     | &#x2714; |
| j1 emulator                    | &#x2714; |
| arguments for j1 executable    | &#x2714; |
| j1 gRPC service                | &#x2714; |
| move R->PC to bit 4            | &#x2714; |
| allow 5 bit opcodes            |  |
| implement 32 bit tagged memory |  |
| add verbosity levels           |  |
