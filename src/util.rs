use std::{fmt::Display, process::exit};

fn error<E: Display>(err: E) -> ! {
    eprintln!("{err}");
    exit(1);
}

pub trait Unwrap<T, E: Display> {
    type Type;

    fn unwrap_or_err(self) -> Self::Type;
}

impl<T, E: Display> Unwrap<T, E> for Result<T, E> {
    type Type = T;

    fn unwrap_or_err(self) -> Self::Type {
        match self {
            Ok(v) => v,
            Err(e) => error(e),
        }
    }
}
