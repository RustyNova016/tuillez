/// Trait to format an element with a custom formatter.
///
/// See [`FormatWithAsync`] for an async version of this trait.
pub trait FormatWith<T> {
    type Error;

    /// Format the current element with a custom formatter.
    fn format_with(&self, ft: &T) -> Result<String, Self::Error>;
}

/// A trait for formatting an element with a custom formatter asynchronously.
///
/// This trait is similar to [`FormatWith`], but it allows for asynchronous formatting.
pub trait FormatWithAsync<T> {
    type Error;

    /// Format the current element with a custom formatter, asynchronously.
    fn format_with_async(
        &self,
        ft: &T,
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send;
}

/// A trait for formatting an element with a custom formatter asynchronously.
///
/// This trait is similar to [`FormatWith`], but it allows for asynchronous formatting.
/// This trait is similar to [`FormatWithAsync`], but it is dyn compatible. Add `#[async_trait]` on the implementation of this trait for easier impl
#[async_trait::async_trait]
pub trait FormatWithAsyncDyn<T> {
    type Error;

    /// Format the current element with a custom formatter, asynchronously.
    async fn format_with_async(&self, ft: &T) -> Result<String, Self::Error>;
}

impl<T, F> FormatWithAsync<F> for T
where
    T: FormatWithAsyncDyn<F> + Sync,
    F: Sync,
{
    type Error = <Self as FormatWithAsyncDyn<F>>::Error;

    async fn format_with_async(&self, ft: &F) -> Result<String, Self::Error> {
        FormatWithAsyncDyn::format_with_async(self, ft).await
    }
}
