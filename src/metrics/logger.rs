use crate::models::SessionMetrics;
use std::fs::{create_dir_all, File};
use std::io::{Write, BufWriter};
use std::path::Path;

pub fn write_to_csv(metrics: &[SessionMetrics], file_path: &str) -> std::io::Result<()> {
    let path = Path::new(file_path);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }

    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "timestamp,bitrate_kbps,buffer_level_secs,stalled,switch")?;

    for m in metrics {
        writeln!(
            writer,
            "{},{},{:.2},{},{}",
            m.timestamp,
            m.bitrate_kbps,
            m.buffer_level_secs,
            m.stalled,
            m.switch
        )?;
    }

    Ok(())
}

#[test]
fn test_logger_creates_csv() {
    use crate::abr::{create_strategy, ABRType};
    use crate::metrics::logger::write_to_csv;

    let mut abr = create_strategy(ABRType::Fixed);
    let metrics = crate::playback::engine::run_simulation(&mut *abr);

    let result = write_to_csv(&metrics, "data/test_metrics.csv");
    assert!(result.is_ok());
}