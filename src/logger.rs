use tracing::Level;

pub fn init(debug: bool) {
    let level = if debug { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt().with_max_level(level).init();
}
