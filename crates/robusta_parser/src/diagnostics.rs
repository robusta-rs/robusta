use robusta_diagnostics::{RobustaDiagnostic, primary_labeled_span};
use robusta_span::Span;

macro_rules! parser_diagnostics {
    (
        $(
            $name:ident($($parameter:ident : $ty:ty),* $(,)?)
                @ $primary:ident => $body:expr
        );* $(;)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[expect(non_camel_case_types)]
        pub enum ParserDiagnostic {
            $(
                $name { $($parameter: $ty),* },
            )*
            /// A diagnostic materialized by an earlier pipeline stage.
            Eager(Box<RobustaDiagnostic>),
        }

        impl ParserDiagnostic {
            /// Materialize the diagnostic at parse exit.
            #[cold]
            pub fn into_diagnostic(self) -> RobustaDiagnostic {
                match self {
                    $(
                        Self::$name { $($parameter),* } => $body,
                    )*
                    Self::Eager(diagnostic) => *diagnostic,
                }
            }

            /// Project the source span without allocating diagnostic strings.
            pub fn primary_span(&self) -> Span {
                match self {
                    $(
                        Self::$name { $primary, .. } => *$primary,
                    )*
                    Self::Eager(diagnostic) => diagnostic.primary_span(),
                }
            }

            pub fn eager(diagnostic: RobustaDiagnostic) -> Self {
                Self::Eager(Box::new(diagnostic))
            }
        }

        $(
            #[inline]
            pub fn $name($($parameter: $ty),*) -> ParserDiagnostic {
                ParserDiagnostic::$name { $($parameter),* }
            }
        )*
    };
}

parser_diagnostics! {
    expected(what: &'static str, span: Span) @ span => {
        RobustaDiagnostic::error(format!("expected {what}"), span)
            .with_label(primary_labeled_span(span, format!("{what} expected here")))
    };

    message(message: &'static str, span: Span) @ span => {
        RobustaDiagnostic::error(message, span)
    };

    message_with_help(message: &'static str, span: Span, help: &'static str) @ span => {
        RobustaDiagnostic::error(message, span).with_help(help)
    };
}

impl ParserDiagnostic {
    /// Compare messages and spans without allocation. Ignore help and labels.
    pub fn same_identity(&self, other: &Self) -> bool {
        if self.primary_span() != other.primary_span() {
            return false;
        }
        match (self.identity_message(), other.identity_message()) {
            (Some(message), _) => other.has_message(message),
            (_, Some(message)) => self.has_message(message),
            (None, None) => match (self, other) {
                (Self::expected { what: left, .. }, Self::expected { what: right, .. }) => {
                    left == right
                }
                _ => false,
            },
        }
    }

    fn identity_message(&self) -> Option<&str> {
        match self {
            Self::message { message, .. } | Self::message_with_help { message, .. } => {
                Some(message)
            }
            Self::Eager(diagnostic) => Some(&diagnostic.message),
            Self::expected { .. } => None,
        }
    }

    fn has_message(&self, message: &str) -> bool {
        match self {
            Self::expected { what, .. } => message.strip_prefix("expected ") == Some(*what),
            Self::message { message: own, .. } | Self::message_with_help { message: own, .. } => {
                message == *own
            }
            Self::Eager(diagnostic) => message == diagnostic.message,
        }
    }
}

pub type Result<T> = std::result::Result<T, ParserDiagnostic>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deferred_diagnostics_expose_identity_and_span_before_materialization() {
        let span = Span::new(3, 5);
        let first = expected("`;`", span);
        let duplicate = expected("`;`", span);
        let different = expected("`)`", span);

        assert_eq!(first.primary_span(), span);
        assert_eq!(first, duplicate);
        assert_ne!(first, different);
        assert!(first.same_identity(&message("expected `;`", span)));
        assert!(!first.same_identity(&message("expected `;`", Span::new(4, 6))));
        assert_eq!(first.into_diagnostic().message, "expected `;`");
    }
}
