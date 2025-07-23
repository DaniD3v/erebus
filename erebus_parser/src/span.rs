use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use ariadne::Span as AriadneSpan;
use chumsky::span::{SimpleSpan, Span as ChumskySpan};

type SpanContext = ();

/// A re-implementation of the builtin SimleSpan
///
/// This allows implementing PartialOrd and Ord
/// so that Ast Nodes capturing Spans can be used in BTrees
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,

    file_ref: SpanContext,
}

impl Span {
    pub(crate) const TEST_VALUE: Self = Self {
        start: 0,
        end: 0,

        file_ref: (),
    };

    pub fn into_range(self) -> Range<usize> {
        Range {
            start: self.start,
            end: self.end,
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.start..self.end).fmt(f)
    }
}

impl From<SimpleSpan> for Span {
    fn from(value: SimpleSpan) -> Self {
        Self {
            start: value.start,
            end: value.end,

            file_ref: (),
        }
    }
}

impl ChumskySpan for Span {
    type Context = SpanContext;
    type Offset = usize;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        Self {
            start: range.start,
            end: range.end,

            file_ref: context,
        }
    }

    fn context(&self) -> Self::Context {
        self.file_ref
    }

    fn start(&self) -> Self::Offset {
        self.start
    }

    fn end(&self) -> Self::Offset {
        self.end
    }
}

impl AriadneSpan for Span {
    type SourceId = ();

    fn source(&self) -> &Self::SourceId {
        &()
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}
