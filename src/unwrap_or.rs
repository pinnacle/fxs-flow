enum Unwrappable<T, E = ()> {
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

pub macro unwrap_or($expr:expr, $default:expr) {{
    match Unwrappable::from($expr) {
        Unwrappable::Option(Some(v)) | Unwrappable::Result(Ok(v)) => v,
        _ => $default,
    }
}}

#[cfg(test)]
mod unwrap_or_test {
    use super::unwrap_or;

    #[test]
    fn works_with_value() {
        assert_eq!(unwrap_or!(Some(2), 1), 2);
        assert_eq!(unwrap_or!(None, 1), 1);
    }

    #[test]
    fn works_with_expr() {
        let mut value = 0;
        let _ = unwrap_or!(Some(()), value += 1);
        assert_eq!(value, 0);

        let mut value = 0;
        let _ = unwrap_or!(None, value += 1);
        assert_eq!(value, 1);
    }

    #[test]
    #[allow(unused_braces)]
    fn works_with_block_expr() {
        let mut value = 0;
        let _ = unwrap_or!(Some(()), { value += 1 });
        assert_eq!(value, 0);

        let mut value = 0;
        let _ = unwrap_or!(None, { value += 1 });
        assert_eq!(value, 1);
    }

    #[test]
    fn works_with_loop_control_stmt() {
        let mut value = 0;
        for _ in 0..5 {
            let _ = unwrap_or!(Some(()), continue);
            value += 1;
        }
        assert_eq!(value, 5);

        let mut value = 0;
        for _ in 0..5 {
            let _ = unwrap_or!(None, continue);
            value += 1;
        }
        assert_eq!(value, 0);

        let mut value = 0;
        for _ in 0..5 {
            let _ = unwrap_or!(None, break);
            value += 1;
        }
        assert_eq!(value, 0);
    }

    #[test]
    fn works_with_return_stmt() {
        let mut value = 0;
        let mut mut_fn = || {
            let _ = unwrap_or!(Some(()), return);
            value += 1
        };
        for _ in 0..5 {
            mut_fn();
        }
        assert_eq!(value, 5);

        let mut value = 0;
        let mut mut_fn = || {
            let _ = unwrap_or!(None, return);
            value += 1
        };
        for _ in 0..5 {
            mut_fn();
        }
        assert_eq!(value, 0);
    }
}
