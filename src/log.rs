use crate::App;
use chrono::Local;

pub enum LogLevel{
    Info,
    Warn,
    Error,
    Debug,
}

impl LogLevel{
    pub fn as_str(&self) -> &'static str{
        match self {
            LogLevel::Info => "Info",
            LogLevel::Warn => "Warn",
            LogLevel::Error => "Error",
            LogLevel::Debug => "Debug",
        }
    }
}

pub fn log_message(message: String, level: LogLevel,app: &mut App){
    let date = Local::now().format("%Y-%m-%d %H:%M:%S");
    let content = format!("{0} : [{1}] - {2}\n", date, level.as_str(), message);
    app.debug_console_content.push_str(content.as_str());
}

pub fn info(message: &str, app: &mut App){
    log_message(message.to_string(), LogLevel::Info,app);
}
pub fn warn(message: &str,app: &mut App){
    log_message(message.to_string(), LogLevel::Warn,app);
}
pub fn error(message: &str,app: &mut App){
    log_message(message.to_string(), LogLevel::Error,app);
}
pub fn debug(message: &str,app: &mut App){
    log_message(message.to_string(), LogLevel::Debug,app);
}