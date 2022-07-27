use colored::Colorize;

pub fn init() -> fern::Dispatch {
  fern::Dispatch::new()
    .format(move |out, message, record| {
      let line = record.line().unwrap_or(0);
      let level = record.level();
      let tip = (format_args!(
        "{} {}{} {}",
        level,
        //record.target(),
        record.file().unwrap_or(""),
        if line > 0 {
          format!(":{}", line)
        } else {
          "".to_string()
        },
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
      ))
      .to_string();
      {
        use log::Level::{Debug, Error, Info, Trace, Warn};
        out.finish(format_args!(
          "{}\n{}\n",
          match level {
            Error => tip.bright_red(),
            Warn => tip.bright_yellow(),
            Info => tip.bright_black(),
            Debug => tip.green(),
            Trace => tip.purple(),
          },
          message,
        ))
      }
    })
    .chain(std::io::stdout())
    .level(log::LevelFilter::Info)
  // .chain(fern::log_file("output.log")?)
}
