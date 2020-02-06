use crate::SpanTrace;
use std::error::Error;
use std::fmt::{self, Debug, Display};

///
pub struct TracedError {
    spantrace: SpanTrace,
    inner: Box<dyn Error + Send + Sync + 'static>,
}

impl TracedError {
    fn new<E>(error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            spantrace: SpanTrace::capture(),
            inner: Box::new(error),
        }
    }
}

impl Error for TracedError {
    fn source<'a>(&'a self) -> Option<&'a (dyn Error + 'static)> {
        self.inner.source()
    }
}

impl Debug for TracedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for TracedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

///
pub trait IntoTracedError<E> {
    ///
    fn in_current_span(self) -> TracedError;
}

impl<E> IntoTracedError<E> for E
where
    E: Error + Send + Sync + 'static,
{
    fn in_current_span(self) -> TracedError {
        TracedError::new(self)
    }
}

///
pub trait Instrument<T, E> {
    ///
    fn in_current_span(self) -> Result<T, TracedError>;
}

impl<T, E> Instrument<T, E> for Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    fn in_current_span(self) -> Result<T, TracedError> {
        self.map_err(TracedError::new)
    }
}

///
pub trait SpanTraceExt {
    ///
    fn spantrace(&self) -> Option<&SpanTrace>;
}

impl SpanTraceExt for &(dyn Error + 'static) {
    fn spantrace(&self) -> Option<&SpanTrace> {
        self.downcast_ref::<TracedError>().map(|e| &e.spantrace)
    }
}
