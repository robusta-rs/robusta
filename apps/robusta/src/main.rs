use std::process::ExitCode;

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "riscv64",
    miri,
    target_os = "freebsd",
    target_family = "wasm"
)))]
#[global_allocator]
static GLOBAL: mimalloc_safe::MiMalloc = mimalloc_safe::MiMalloc;

fn main() -> ExitCode {
    robusta_cli::run_cli()
}
