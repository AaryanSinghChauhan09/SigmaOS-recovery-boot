//! SigmaOS-recovery-boot — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_recovery_boot_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-recovery-boot: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-recovery-boot: ok");
}
