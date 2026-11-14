use std::fmt;
use std::fmt::Write as _;

use nu_ansi_term::Color;
use nu_ansi_term::Style;
use tracing_core::Event;
use tracing_core::Level;
use tracing_core::Subscriber;
use tracing_core::field::Field;
use tracing_core::field::Visit;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::registry::LookupSpan;

/// Width the message is padded to, so the fields start in the same column.
const MESSAGE_WIDTH: usize = 20;

/// Installs the global subscriber: local time with milliseconds, level, padded message and fields, separated by dim pipes, filtered by `RUST_LOG` or at INFO when it is not set.
pub fn init() {
    let filter = EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
    tracing_subscriber::fmt().event_format(Line).with_env_filter(filter).init();
}

/// Formats an event as one line of pipe-separated columns.
struct Line;

impl<S, N> FormatEvent<S, N> for Line
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    /// Writes time, level, message and fields, with the level and message padded to a fixed width.
    fn format_event(&self, _ctx: &FmtContext<'_, S, N>, mut writer: Writer<'_>, event: &Event<'_>) -> fmt::Result {
        let mut columns = Columns::default();
        event.record(&mut columns);

        let pipe = Style::new().dimmed().paint("|");
        let level = *event.metadata().level();
        let level_color = match level {
            Level::TRACE => Color::Purple,
            Level::DEBUG => Color::Blue,
            Level::INFO => Color::Green,
            Level::WARN => Color::Yellow,
            Level::ERROR => Color::Red,
        };
        ChronoLocal::new("%H:%M:%S%.3f".to_owned()).format_time(&mut writer)?;
        write!(writer, " {pipe} {} {pipe} ", level_color.paint(format!("{:<5}", level.as_str())))?;

        if columns.fields.is_empty() {
            return writeln!(writer, "{}", columns.message);
        }
        writeln!(writer, "{:<MESSAGE_WIDTH$} {pipe} {}", columns.message, columns.fields)
    }
}

/// Message and formatted fields of one event.
#[derive(Default)]
struct Columns {
    /// Event message as plain text.
    message: String,
    /// Fields as bold names and magenta values, separated by spaces.
    fields: String,
}

impl Visit for Columns {
    /// Stores the message as plain text and appends every other field as `name=value`.
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{value:?}");
            return;
        }
        if !self.fields.is_empty() {
            self.fields.push(' ');
        }
        let _ = write!(self.fields, "{}={}", Style::new().bold().paint(field.name()), Color::Magenta.paint(format!("{value:?}")));
    }
}
