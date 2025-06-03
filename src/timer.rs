use std::process::Command;
use std::time::Instant;
use colored::*;

pub fn run_and_time(command_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    #[cfg(target_family = "unix")]
    let mut command = Command::new("sh");

    #[cfg(target_family = "windows")]
    let mut command = Command::new("cmd");

    #[cfg(target_family = "unix")]
    command.arg("-c").arg(command_str);

    #[cfg(target_family = "windows")]
    command.arg("/C").arg(command_str);

    let status = command.status()?;
    let duration = start.elapsed();

    let exit_code = status.code().unwrap_or(-1);
    let exit_display = if exit_code == 0 {
        exit_code.to_string().green()
    } else {
        exit_code.to_string().red()
    };

    // Threshold in milliseconds
    let threshold_ms = 500;
    let duration_ms = duration.as_millis();
    let duration_str = format!("{:.3?}", duration);

    let duration_display = if duration_ms > threshold_ms {
        duration_str.red()
    } else {
        duration_str.green()
    };

    println!("\nCommand: {}", command_str);
    println!("Exit code: {}", exit_display);
    println!("Time elapsed: {}", duration_display);

    Ok(())
}
