mod real;

mod logical {
    use pest::Span;
    use pest_ast::FromPest;

    use crate::parser::{span_into_str, Rule};
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromPest, Default)]
    #[pest_ast(rule(Rule::logical))]
    pub struct Logical(
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))] bool,
    );

    impl std::ops::Deref for Logical {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl From<Logical> for bool {
        fn from(value: Logical) -> Self {
            value.0
        }
    }

    impl From<bool> for Logical {
        fn from(value: bool) -> Self {
            Self(value)
        }
    }

    impl<'a> From<Span<'a>> for Logical {
        fn from(value: Span<'a>) -> Self {
            span_into_str(value)
                .to_lowercase()
                .parse::<bool>()
                .unwrap()
                .into()
        }
    }
}

pub use logical::Logical;
pub use real::Real;
