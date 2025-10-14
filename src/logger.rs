use flexi_logger::{FileSpec, LoggerHandle};

pub fn setup_logger_handle() -> LoggerHandle {
    let file_spec = FileSpec::default().directory(".");

    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .append()
        .log_to_file(FileSpec::suppress_timestamp(file_spec))
        .start()
        .unwrap()
}
