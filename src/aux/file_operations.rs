// better-logger/src/aux/file_operations.rs

pub(crate) fn create_file(path: &str) -> std::io::Result<std::fs::File> {
    let path: &std::path::Path = std::path::Path::new(path);

    match path.parent() {
        Some(parent) => {
            std::fs::create_dir_all(parent)?;
        }
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Path is not valid"));
        }
    }

    let file: std::fs::File = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path)?;

    return Ok(file);    
}

pub(crate) fn write_log_line(level: &str, target: &str, message: &str) -> std::io::Result<()> {
    let mutex_file: &std::sync::Mutex<std::fs::File> = {
        match crate::aux::running_settings::LOG_FILE.get() {
            Some(file) => {
                file
            }
            None => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "log file not initialized"));
            }
        }
    };

    let mut file: std::sync::MutexGuard<'_, std::fs::File> = {
        match mutex_file.lock() {
            Ok(file) => {
                file
            },
            Err(error) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("file mutex is poisoned: {}", error)));
            } 
        }
    };

    let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let header: String = format!("[{} {} {}]", timestamp, level.to_uppercase(), target);
    let line: String   = format!("{} {}", header, message);

    use std::io::Write;
    writeln!(file, "{}", line)?;

    return Ok(());
}