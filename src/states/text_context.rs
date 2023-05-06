use crate::data::span::Span;
use crate::errors::error_context::ErrorContext;
use crate::states::TextContext;

impl TextContext {
    pub fn source(&self) -> &str {
        &self.0
    }

    pub fn error_context(&self, span: Option<Span>) -> ErrorContext {
        if self.source().is_empty() {
            return "".into();
        }

        let span = match span {
            Some(span) => span,
            None => return "".into(),
        };

        let extract: String = self
            .source()
            .split('\n')
            .skip(span.start.line - 1)
            .take(span.end.line - span.start.line + 1)
            .enumerate()
            .map(|(i, line)| {
                let cur_line = i + span.start.line;
                let start = if cur_line == span.start.line {
                    span.start.col
                } else {
                    1
                };
                let end = if cur_line == span.end.line {
                    span.end.col
                } else {
                    line.len()
                };

                let underline = " ".repeat(start - 1) + &"^".repeat(end - start + 1);
                format!("{:>4} | {}\n     | {}\n", cur_line, line, underline)
            })
            .collect();

        extract.into()
    }
}
