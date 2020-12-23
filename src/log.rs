/*
 * log.rs
 *
 * ftml - Library to parse Wikidot code
 * Copyright (C) 2019-2020 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! Helper module to add filenames and newlines to slog output.
//!
//! See https://github.com/slog-rs/slog/issues/123

macro_rules! slog_filename {
    () => {
        slog::PushFnValue(|r: &slog::Record, ser: slog::PushFnValueSerializer| {
            ser.emit(r.file())
        })
    };
}

macro_rules! slog_lineno {
    () => {
        slog::PushFnValue(|r: &slog::Record, ser: slog::PushFnValueSerializer| {
            ser.emit(r.line())
        })
    };
}

#[cfg(test)]
mod loggers {
    #[inline]
    fn build_logger() -> slog::Logger {
        build_null_logger()
    }

    #[allow(dead_code)]
    fn build_null_logger() -> slog::Logger {
        slog::Logger::root(slog::Discard, o!())
    }

    #[allow(dead_code)]
    fn build_console_logger() -> slog::Logger {
        use sloggers::terminal::TerminalLoggerBuilder;
        use sloggers::types::Severity;
        use sloggers::Build;

        TerminalLoggerBuilder::new()
            .level(Severity::Trace)
            .build()
            .expect("Unable to initialize logger")
    }

    #[allow(dead_code)]
    fn build_json_logger() -> slog::Logger {
        use slog::Drain;
        use std::io;
        use std::sync::Mutex;

        // For writing to a file:
        // + .add_default_keys()
        // + .set_pretty(false)
        let drain = slog_bunyan::with_name("ftml", io::stdout())
            .set_newlines(true)
            .set_pretty(true)
            .set_flush(true)
            .build();

        slog::Logger::root(Mutex::new(drain).fuse(), o!("env" => "test"))
    }
}

#[cfg(test)]
pub use self::loggers::{
    build_console_logger, build_json_logger, build_logger, build_null_logger,
};
