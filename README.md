# Nexis Kernel

```
 .S_sSSs      sSSs   .S S.    .S    sSSs 
.SS~YS%%b    d%%SP  .SS SS.  .SS   d%%SP 
S%S   `S%b  d%S'    S%S S%S  S%S  d%S'   
S%S    S%S  S%S     S%S S%S  S%S  S%|    
S%S    S&S  S&S     S%S S%S  S&S  S&S    
S&S    S&S  S&S_Ss   SS SS   S&S  Y&Ss   
S&S    S&S  S&S~SP    S_S    S&S  `S&&S  
S&S    S&S  S&S      SS~SS   S&S    `S*S 
S*S    S*S  S*b     S*S S*S  S*S     l*S 
S*S    S*S  S*S.    S*S S*S  S*S    .S*P 
S*S    S*S   SSSbs  S*S S*S  S*S  sSS*S  
S*S    SSS    YSSP  S*S S*S  S*S  YSS'   
SP                  SP       SP          
Y                   Y        Y           

```
**Nexis** is a Rust-based kernel built for **safety, modularity, and experimentation**.  
It prevents unsafe memory access by default and includes:  
- **Preemptive multitasking** with a custom scheduler  
- **Safe memory management** via a physical memory manager  
- **Task context switching** using Rust + inline assembly  
- **Basic VGA + serial output**  
- **PS/2 keyboard input**  
- **System calls & syscall dispatcher**  
- **Early filesystem support**  

---

## Development Status 🚧
All major subsystems (scheduler, filesystem, syscalls, VGA, keyboard, memory manager) are implemented.  
Currently, the project faces **build system issues** (`cargo`, `bootimage`, and target configs), which are being debugged.  
Despite this, the core kernel code is in place and actively evolving.  

---

## Project Structure
---

## Project Structure
```
├── LICENSE
├── README.md
├── Cargo.toml
├── Nexis/                  # Rust-based kernel (Nexis)
│   ├── .cargo/
│   ├── asm/                # Assembly boot and low-level routines
│   │   ├── boot.S
│   │   └── context.S
│   ├── config/             # Kernel configuration & constants
│   │   ├── defs.rs
│   │   └── consts.rs
│   ├── core/               # Core kernel logic and scheduling
│   │   ├── memory.rs
│   │   ├── scheduler.rs
│   │   ├── tasks.rs
│   │   ├── process.rs
│   │   ├── syscall.rs
│   │   └── syscall_dispatch.rs
│   ├── cortex/             # CPU, architecture, and init routines
│   │   ├── boot.rs
│   │   ├── context.rs
│   │   └── mod.rs
│   ├── drivers/            # Hardware drivers
│   │   ├── vga.rs
│   │   ├── pit.rs
│   │   ├── kb.rs
│   │   ├── fs.rs
│   │   └── interrupts.rs
│   ├── userland/           # Programs & services in ring 3
│   │   ├── init.rs
│   │   ├── shell.rs
│   │   └── mod.rs
│   ├── utils/              # Logging, macros, debugging helpers
│   │   ├── macros.rs
│   │   ├── logging.rs
│   │   └── mod.rs
│   ├── scripts/            # Build & QEMU scripts
│   │   └── qemu-run.sh
│   ├── build.rs
│   ├── Cargo.toml
│   ├── linker.ld
│   ├── lib.rs
│   ├── main.rs
│   ├── x86_64-nexis.json
│   └── .gitignore
└── IronVeil/               # OS shell & higher-level interface
    ├── Cargo.toml
    ├── README.md
    ├── assets/
    ├── ui/
    └── src/
        ├── main.rs
        ├── tui.rs
        ├── cli.rs
        ├── network.rs
        ├── storage.rs
        └── mod.rs

```

---

## Build & Run

### Requirements:
- Rust nightly toolchain
- `bootimage`
- `qemu-system-x86_64`

### Install & Build:
```bash
cargo install bootimage
rustup override set nightly
rustup component add rust-src
cargo bootimage
```

### Run in QEMU:
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-nexis/debug/bootimage-nexis.bin
```

---

## Commands (VGA Shell)
| Command         | Description                          |
|-----------------|--------------------------------------|
| `help`          | Show available commands              |
| `clear` / `cls` | Clear the screen                     |
| `genpass`       | Generate a 16-char password          |
| `ip`            | Generate a fake IPv4 address         |
| `mac`           | Generate a fake MAC address          |
| `fs ls`         | List available files                 |
| `fs cat <file>` | Print file contents                  |
| `reboot`        | Halt the kernel (restart in QEMU)    |

---

## License
This project is licensed under the MIT License – see the LICENSE file for details.
