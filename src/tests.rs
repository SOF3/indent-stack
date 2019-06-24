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

use crate::{IndentError, IndentStack};

#[test]
fn plain() {
    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept(""), Ok(0));
    assert_eq!(is.accept(""), Ok(0));
    assert_eq!(is.accept(""), Ok(0));
}

#[test]
fn indent() {
    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept(""), Ok(0));
    assert_eq!(is.accept("  "), Ok(1));
    assert_eq!(is.accept("  "), Ok(0));
    assert_eq!(is.accept("  "), Ok(0));

    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept("  "), Ok(1));
    assert_eq!(is.accept("  "), Ok(0));
}

#[test]
fn dedent() {
    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept(""), Ok(0));
    assert_eq!(is.accept("  "), Ok(1));
    assert_eq!(is.accept(""), Ok(-1));
    assert_eq!(is.accept(""), Ok(0));
    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept("  "), Ok(1));
    assert_eq!(is.accept("  \t"), Ok(1));
    assert_eq!(is.accept(""), Ok(-2));
    assert_eq!(is.accept(""), Ok(0));
}

#[test]
fn wrong_indent() {
    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept("  "), Ok(1));
    assert_eq!(is.accept("\t"), Err(IndentError::MixedIndent));

    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept("\t"), Ok(1));
    assert_eq!(is.accept("  "), Err(IndentError::MixedIndent));
}

#[test]
fn two_inconsistent_indents() {
    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept("  "), Ok(1));
    assert_eq!(is.accept("  \t"), Ok(1));
    assert_eq!(is.accept("  "), Ok(-1));
    assert_eq!(is.accept(""), Ok(-1));
    assert_eq!(is.accept(""), Ok(0));
}

#[test]
fn multi_wrong_indent() {
    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept("  "), Ok(1));
    assert_eq!(is.accept("  \t"), Ok(1));
    assert_eq!(is.accept("\t"), Err(IndentError::MixedIndent));

    let mut is = IndentStack::default_inconsistent_indents();
    assert_eq!(is.accept("\t"), Ok(1));
    assert_eq!(is.accept("\t  "), Ok(1));
    assert_eq!(is.accept("  "), Err(IndentError::MixedIndent));
}

#[test]
fn inconsistent_indents_mixed() {
    let mut is = IndentStack::default();
    assert_eq!(is.accept("  "), Ok(1));
    assert_eq!(is.accept("  \t"), Err(IndentError::InconsistentIndent));
}
#[test]
fn inconsistent_indents_prefix() {
    let mut is = IndentStack::default();
    assert_eq!(is.accept("  "), Ok(1));
    assert_eq!(is.accept("      "), Err(IndentError::InconsistentIndent));
}
#[test]
fn inconsistent_indents_suffix() {
    let mut is = IndentStack::default();
    assert_eq!(is.accept("    "), Ok(1));
    assert_eq!(is.accept("      "), Err(IndentError::InconsistentIndent));
}
