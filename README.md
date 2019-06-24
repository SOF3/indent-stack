# indent-stack
Rust library for parsing off-side syntax like YAML or Python

## Syntax specification
Although indents are typically spaces and tabs, this library does not specify any constraints on the characters used for
indentation. As long as the character sequences are identical (without equivalent conversions), they are considered as the same
indent level.

This library accepts indentation strings from callers; it is not the responsibility of this library to extract
indentations from a file buffer. Callers should extract the indentation characters from each line and iteratively pass
them to the `accept()` function.

The off-side syntax is defined by the following rules:
- If the indent of line `x + 1` is longer than and starts with that of line `x`, line `x + 1` is called a "child" of line `x`.
- If lines `x` and `y` have the same indent and lines `x+1..=y-1` are children of line `x`, lines `x` and `y` are "siblings".
- If line `y` is a child of line `x` and lines `z` and `y` are siblings, line `z` is also a child of line `x`.
- All valid lines are either descendents or siblings of line 0 (which is defined as a line without indent).

This leads to the following results:
- If the indent of a line is neither a prefix nor a continuation of that of the previous line, it is an invalid line.
- The longest common prefix of the indents of all siblings and descendents of a line is its own indent.

## Disallow inconsistent indents
By default, inconsistent indents are disallowed. That is, all indents must be repetitions of the first indent.

For example, if the first indent is 2 spaces, all indents must be 2 spaces; a line with 2-space indent followed by a
line with 4-space indent is syntax error.
