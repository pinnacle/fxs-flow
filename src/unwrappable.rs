pub enum Unwrappable<T, E = !> {
    Option(Option<T>),
    Result(Result<T, E>),
}

impl<T> From<Option<T>> for Unwrappable<T> {
    fn from(value: Option<T>) -> Self {
        Unwrappable::Option(value)
    }
}

impl<T, E> From<Result<T, E>> for Unwrappable<T, E> {
    fn from(value: Result<T, E>) -> Self {
        Unwrappable::Result(value)
    }
}
