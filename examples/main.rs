use log::info;

fn main() {
  rmw_log::init()
    .level_for("surf", log::LevelFilter::Warn)
    .apply()
    .unwrap();
  info!("test");
}
