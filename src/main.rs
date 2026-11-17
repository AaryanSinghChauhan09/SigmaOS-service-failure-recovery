//! SigmaOS-service-failure-recovery — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_service_failure_recovery_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-service-failure-recovery: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-service-failure-recovery: ok");
}
