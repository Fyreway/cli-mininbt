use std::{fmt::Display, process::exit};

fn display_error<E: Display>(msg: &str, err: E) -> ! {
    eprintln!("{msg}: {err}");
    exit(1);
}

fn str_error<E: ToString>(msg: &str, err: &E) -> ! {
    eprintln!("{msg}: {}", err.to_string());
    exit(1);
}

pub trait UnwrapOrDisplayErr<E: Display> {
    type Type;

    fn unwrap_or_err(self, msg: &str) -> Self::Type;
}

impl<T, E: Display> UnwrapOrDisplayErr<E> for Result<T, E> {
    type Type = T;

    fn unwrap_or_err(self, msg: &str) -> Self::Type {
        match self {
            Ok(v) => v,
            Err(e) => display_error(msg, e),
        }
    }
}

pub trait UnwrapOrStrErr<E: ToString> {
    type Type;

    fn unwrap_or_err(self, msg: &str) -> Self::Type;
}

impl<T, E: ToString> UnwrapOrStrErr<E> for Result<T, E> {
    type Type = T;

    fn unwrap_or_err(self, msg: &str) -> Self::Type {
        match self {
            Ok(v) => v,
            Err(e) => str_error(msg, &e),
        }
    }
}
