pub macro unwrap_or_match {
    ($expr:expr, { $(if $guard:expr => $result:expr),*, _ => $default:expr $(,)? }) => {{
        #[allow(unreachable_patterns)]
        match crate::Unwrappable::from($expr) {
            $(crate::Unwrappable::Option(None) if $guard => $result),*,
            $(crate::Unwrappable::Result(Err(..)) if $guard => $result),*,
            crate::Unwrappable::Option(Some(v)) | crate::Unwrappable::Result(Ok(v)) => v,
            crate::Unwrappable::Option(None) | crate::Unwrappable::Result(Err(..)) => $default,
        }
    }},
    ($expr:expr, { $(if $guard:expr => $result:expr),* $(,)? }) => {{
        #[allow(unreachable_patterns)]
        match crate::Unwrappable::from($expr) {
            $(crate::Unwrappable::Option(None) if $guard => $result),*,
            $(crate::Unwrappable::Result(Err(..)) if $guard => $result),*,
            crate::Unwrappable::Option(Some(v)) | crate::Unwrappable::Result(Ok(v)) => v,
            crate::Unwrappable::Option(None) | crate::Unwrappable::Result(Err(..)) => Default::default(),
        }
    }},
    ($expr:expr, { $($pat:pat $(if $guard:expr)? => $result:expr),* $(,)? }) => {{
        #[allow(unreachable_patterns)]
        match crate::Unwrappable::from($expr) {
            crate::Unwrappable::Option(Some(v)) | crate::Unwrappable::Result(Ok(v)) => v,
            $(crate::Unwrappable::Option(None) $(if $guard)? => $result),*,
            $(crate::Unwrappable::Result(Err($pat)) $(if $guard)? => $result),*,
        }
    }},
}

#[cfg(test)]
mod test {
    use super::unwrap_or_match;

    #[test]
    fn works_with_value() {
        assert_eq!(unwrap_or_match!(Some(1), { _ => 0 }), 1);
        assert_eq!(unwrap_or_match!(None, { if true => 1, _ => 0 }), 1);
        assert_eq!(unwrap_or_match!(None, { if false => 0, _ => 1 }), 1);

        assert_eq!(unwrap_or_match!(Ok::<i32, !>(1), { _ => 0 }), 1);
        assert_eq!(unwrap_or_match!(Err::<i32, i32>(1), { _ => 1 }), 1);
        assert_eq!(unwrap_or_match!(Err::<i32, i32>(1), { 1 => 1, _ => 0 }), 1);
    }

    #[test]
    fn works_with_expr() {
        let mut value = 0;
        let _ = unwrap_or_match!(Some(()), { _ => value += 1 });
        let _ = unwrap_or_match!(Some(()), { if true => value += 1 });
        let _ = unwrap_or_match!(Some(()), { if false => value += 0 });
        assert_eq!(value, 0);

        let mut value = 0;
        let _ = unwrap_or_match!(None, { _ => value += 1 });
        let _ = unwrap_or_match!(None, { if true => value += 1 });
        let _ = unwrap_or_match!(None, { if false => value += 1 });
        assert_eq!(value, 2);

        let mut value = 0;
        let _ = unwrap_or_match!(Ok::<(), !>(()), { _ => value += 1 });
        let _ = unwrap_or_match!(Ok::<(), !>(()), { if true => value += 1 });
        let _ = unwrap_or_match!(Ok::<(), !>(()), { if false => value += 0 });
        assert_eq!(value, 0);

        let mut value = 0;
        let _ = unwrap_or_match!(Err::<!, ()>(()), { _ => value += 1 });
        let _ = unwrap_or_match!(Err::<!, ()>(()), { if true => value += 1 });
        let _ = unwrap_or_match!(Err::<!, ()>(()), { if false => value += 1 });
        assert_eq!(value, 2);
    }

    #[test]
    fn works_with_loop_control_stmt() {
        let mut value = 0;
        for _ in 0..5 {
            let _ = unwrap_or_match!(Some(()), { _ => continue });
            let _ = unwrap_or_match!(Ok::<(), !>(()), { _ => continue });
            value += 1;
        }
        assert_eq!(value, 5);

        let mut value = 0;
        for _ in 0..5 {
            let _ = unwrap_or_match!(None, { _ => continue });
            value += 1;
        }
        assert_eq!(value, 0);

        let mut value = 0;
        for _ in 0..5 {
            let _ = unwrap_or_match!(Err::<!, ()>(()), { () => continue, _ => {} });
            value += 1;
        }
        assert_eq!(value, 0);
    }

    #[test]
    fn works_with_return_stmt() {
        let mut value = 0;
        let mut mut_fn = || {
            let _ = unwrap_or_match!(Some(()), { _ => return });
            let _ = unwrap_or_match!(Ok::<(), !>(()), { _ => return });
            value += 1
        };
        for _ in 0..5 {
            mut_fn();
        }
        assert_eq!(value, 5);

        let mut value = 0;
        let mut mut_fn = || {
            let _ = unwrap_or_match!(None, { _ => return });
            value += 1
        };
        for _ in 0..5 {
            mut_fn();
        }
        assert_eq!(value, 0);

        let mut value = 0;
        let mut mut_fn = || {
            let _ = unwrap_or_match!(Err::<!, ()>(()), { () => return, _ => {} });
            value += 1
        };
        for _ in 0..5 {
            mut_fn();
        }
        assert_eq!(value, 0);
    }
}
