use tracing::info;

#[test]
fn init_installs_subscriber() {
    tracing_dinhani::init();

    info!(scanned = 2, saved_mb = 0.6, "Finished");

    assert!(tracing::dispatcher::has_been_set());
}
