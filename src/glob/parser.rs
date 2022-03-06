mod alt;
mod charclass;
mod main;
mod numrange;

pub use main::parse as parse;

type Chars<'a> = std::iter::Peekable<std::str::Chars<'a>>;
