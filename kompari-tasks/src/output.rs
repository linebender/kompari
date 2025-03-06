use humansize::{format_size, DECIMAL};
use kompari::OptimizationResult;
use std::io::Write;
use termcolor::{Color, ColorSpec, WriteColor};

pub fn print_size_optimization_results(results: &[OptimizationResult]) -> kompari::Result<()> {
    if results.is_empty() {
        println!("Nothing to optimize");
        return Ok(());
    }
    let stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
    let mut stdout = stdout.lock();
    let mut total_size = 0;
    let mut total_diff = 0;
    for result in results {
        let diff = result.old_size - result.new_size;
        stdout.set_color(ColorSpec::new().set_fg(None))?;
        write!(stdout, "{}: ", result.path.display(),)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, "{} ", format_size(result.new_size, DECIMAL))?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(stdout, "(-{})", format_size(diff, DECIMAL))?;
        total_size += result.new_size;
        total_diff += diff;
    }
    stdout.set_color(ColorSpec::new().set_fg(None))?;
    write!(stdout, "----------------------------\nTotal size: ",)?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    write!(stdout, "{} ", format_size(total_size, DECIMAL))?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(stdout, "(-{})", format_size(total_diff, DECIMAL))?;
    Ok(())
}
