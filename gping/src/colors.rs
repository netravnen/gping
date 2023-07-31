use std::{iter::Iterator, ops::RangeFrom, str::FromStr};

use anyhow::{anyhow, Result};
use tui::style::Color;

pub struct Colors<T> {
    already_used: Vec<Color>,
    color_names: T,
    indices: RangeFrom<u8>,
}

impl<T> From<T> for Colors<T> {
    fn from(color_names: T) -> Self {
        Self {
            already_used: Vec::new(),
            color_names,
            indices: 2..,
        }
    }
}

impl<'a, T> Iterator for Colors<T>
where
    T: Iterator<Item = &'a String>,
{
    type Item = Result<Color>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.color_names.next() {
            // TODO: As of v0.21.0, Ratatui only accepts color patterns
            // formatted as "lightred" or "light red". We replace any '-' in
            // the color name with a space for compatibility.
            //
            // Note: Revisit this section when Ratatui supports patterns such
            // as "light-red". At that point, this replacement operation
            // will be unnecessary and can be removed.
            //
            // See https://github.com/tui-rs-revival/ratatui/issues/305
            Some(name) => match Color::from_str(&name.replace('-', " ")) {
                Ok(color) => {
                    if !self.already_used.contains(&color) {
                        self.already_used.push(color);
                    }
                    Some(Ok(color))
                }
                error => Some(error.map_err(|err| {
                    anyhow!(err).context(format!("Invalid color code: `{}`", name))
                })),
            },
            None => loop {
                let index = unsafe { self.indices.next().unwrap_unchecked() };
                let color = Color::Indexed(index);
                if !self.already_used.contains(&color) {
                    self.already_used.push(color);
                    break Some(Ok(color));
                }
            },
        }
    }
}
