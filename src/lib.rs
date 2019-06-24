/*
 * indent-stack
 *
 * Copyright (C) 2019 chankyin
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Stores the indentation state of an off-side parser.
/// The default state is where
#[derive(Debug, Default)]
pub struct IndentStack {
    stack: Vec<String>,
    std_indent: Option<String>,
    pub allow_inconsistent_indents: bool,
}

impl IndentStack {
    pub fn default_inconsistent_indents() -> Self {
        Self { allow_inconsistent_indents: true, ..Default::default() }
    }

    /// Mutates the state of the stack by applying an indent level.
    /// There is no restriction on the input characters, as long as they are identical.
    ///
    /// The initial state is an empty indent. Passing a non-empty string the first time would
    /// immediately result in an indent.
    ///
    /// # Returns
    /// - If an indentation error occurred, `None` is returned.
    /// - `Ok(1)` implies that an indent is detected.
    /// - `Ok(0)` implies that no indentation change is detected.
    /// - `Ok(x)` where `x < 0` implies that `-x` levels of dedents are detected.
    pub fn accept(&mut self, input: &str) -> Result<isize, IndentError> {
        let mut offset = 0;
        for i in 0..self.stack.len() {
            if offset == input.len() {
                let ret = self.stack.len() - i;
                self.stack.drain(i..);
                return Ok(-(ret as isize));
            }

            let indent = self.stack[i].as_str();
            if input.len() - offset < indent.len() || &input[offset..(offset + indent.len())] != indent {
                return Err(IndentError::MixedIndent);
            }

            offset += indent.len();
        }

        if offset == input.len() {
            return Ok(0);
        }

        let indent = &input[offset..];
        if !self.allow_inconsistent_indents {
            match &self.std_indent {
                Some(std) => {
                    if indent != std {
                        return Err(IndentError::InconsistentIndent);
                    }
                }
                None => {
                    self.std_indent = Some(indent.into());
                }
            }
        }

        self.stack.push(input[offset..].into());
        Ok(1)
    }
}

#[derive(Debug, PartialEq)]
pub enum IndentError {
    /// Not all indentations use the same character sequence.
    ///
    /// Only returned if allow_inconsistent_indent is false.
    InconsistentIndent,

    /// The current indentation is not a continuation nor substring of the previous indentation.
    MixedIndent,
}

impl Display for IndentError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            IndentError::InconsistentIndent => write!(f, "Not all indentations use the same character sequence."),
            IndentError::MixedIndent => write!(f, "The current indentation is not a continuation nor substring of the previous indentation."),
        }
    }
}

impl Error for IndentError {}

#[cfg(test)]
mod tests;
