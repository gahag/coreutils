//! CFG constants

// TODO: Add illumos target once it's done
#[cfg(target_os = "linux")]
pub const HOST_OS: &str = "Linux";
#[cfg(target_os = "freebsd")]
pub const HOST_OS: &str = "FreeBSD";
#[cfg(target_os = "netbsd")]
pub const HOST_OS: &str = "NetBSD";
#[cfg(target_os = "openbsd")]
pub const HOST_OS: &str = "OpenBSD";
#[cfg(target_os = "dragonfly")]
pub const HOST_OS: &str = "DragonflyBSD";
#[cfg(target_os = "illumos")]
pub const HOST_OS: &str = "Illumos";
#[cfg(target_os = "solaris")]
pub const HOST_OS: &str = "Solaris";
#[cfg(target_os = "macos")]
pub const HOST_OS: &str = "Darwin";
#[cfg(target_os = "haiku")]
pub const HOST_OS: &str = "Haiku";
#[cfg(target_os = "fuchsia")]
pub const HOST_OS: &str = "Fuchsia";
#[cfg(target_os = "redox")]
pub const HOST_OS: &str = "Redox";

#[cfg(target_arch = "aarch64")]
pub const MACHINE_ARCH: &str = "aarch64";
#[cfg(target_arch = "arm")]
pub const MACHINE_ARCH: &str = "arm";
#[cfg(target_arch = "armebv7r")]
pub const MACHINE_ARCH: &str = "armebv7r";
#[cfg(target_arch = "armv5te")]
pub const MACHINE_ARCH: &str = "armv5te";
#[cfg(target_arch = "armv7")]
pub const MACHINE_ARCH: &str = "armv7";
#[cfg(target_arch = "asmjs")]
pub const MACHINE_ARCH: &str = "asmjs";
#[cfg(target_arch = "i386")]
pub const MACHINE_ARCH: &str = "i386";
#[cfg(target_arch = "i586")]
pub const MACHINE_ARCH: &str = "i586";
#[cfg(target_arch = "i686")]
pub const MACHINE_ARCH: &str = "i686";
#[cfg(target_arch = "mips")]
pub const MACHINE_ARCH: &str = "mips";
#[cfg(target_arch = "mips64")]
pub const MACHINE_ARCH: &str = "mips64";
#[cfg(target_arch = "mips64el")]
pub const MACHINE_ARCH: &str = "mips64el";
#[cfg(target_arch = "mipsel")]
pub const MACHINE_ARCH: &str = "mipsel";
#[cfg(target_arch = "powerpc")]
pub const MACHINE_ARCH: &str = "powerpc";
#[cfg(target_arch = "powerpc64")]
pub const MACHINE_ARCH: &str = "powerpc64";
#[cfg(target_arch = "powerpc64le")]
pub const MACHINE_ARCH: &str = "powerpc64le";
#[cfg(target_arch = "riscv32imac")]
pub const MACHINE_ARCH: &str = "riscv32imac";
#[cfg(target_arch = "riscv32imc")]
pub const MACHINE_ARCH: &str = "riscv32imc";
#[cfg(target_arch = "riscv64gc")]
pub const MACHINE_ARCH: &str = "riscv64gc";
#[cfg(target_arch = "riscv64imac")]
pub const MACHINE_ARCH: &str = "riscv64imac";
#[cfg(target_arch = "s390x")]
pub const MACHINE_ARCH: &str = "s390x";
#[cfg(target_arch = "sparc64")]
pub const MACHINE_ARCH: &str = "sparc64";
#[cfg(target_arch = "sparcv9")]
pub const MACHINE_ARCH: &str = "sparcv9";
#[cfg(target_arch = "thumbv6m")]
pub const MACHINE_ARCH: &str = "thumbv6m";
#[cfg(target_arch = "thumbv7em")]
pub const MACHINE_ARCH: &str = "thumbv7em";
#[cfg(target_arch = "thumbv7m")]
pub const MACHINE_ARCH: &str = "thumbv7m";
#[cfg(target_arch = "thumbv7neon")]
pub const MACHINE_ARCH: &str = "thumbv7neon";
#[cfg(target_arch = "wasm32")]
pub const MACHINE_ARCH: &str = "wasm32";
#[cfg(target_arch = "x86")]
pub const MACHINE_ARCH: &str = "x86";
#[cfg(target_arch = "x86_64")]
pub const MACHINE_ARCH: &str = "amd64";
#[cfg(target_arch = "hexagon")]
pub const MACHINE_ARCH: &str = "hexagon";
#[cfg(target_arch = "mipsisa32r6el")]
pub const MACHINE_ARCH: &str = "mipsisa32r6el";
#[cfg(target_arch = "mipsisa64r6")]
pub const MACHINE_ARCH: &str = "mipsisa64r6";
#[cfg(target_arch = "mipsisa64r6el")]
pub const MACHINE_ARCH: &str = "mipsisa64r6el";
#[cfg(target_arch = "msp430")]
pub const MACHINE_ARCH: &str = "msp430";
#[cfg(target_arch = "nvptx")]
pub const MACHINE_ARCH: &str = "nvptx";
#[cfg(target_arch = "nvptx64")]
pub const MACHINE_ARCH: &str = "nvptx64";
#[cfg(target_arch = "riscv32i")]
pub const MACHINE_ARCH: &str = "riscv32i";
#[cfg(target_arch = "thumbv8m.base")]
pub const MACHINE_ARCH: &str = "thumbv8m.base";
#[cfg(target_arch = "thumbv8m.main")]
pub const MACHINE_ARCH: &str = "thumbv8m.main";
