mod collector;
mod lexer;

pub use collector::TokenCollector;
pub use lexer::Lexer;

#[cfg(test)]
pub use collector::tests::MockTokenCollector;
