use crate::parsers::parser::Parser;
use crate::parsers::response::Response;

// -------------------------------------------------------------------------------------------------
// Executable type definition
// -------------------------------------------------------------------------------------------------

pub trait Executable<'a, A>
where
    Self: Parser<A>,
{
    fn execute(&self, s: &'a [u8], o: usize) -> Response<A>;
}

// -------------------------------------------------------------------------------------------------
// Parse type definition
// -------------------------------------------------------------------------------------------------

pub trait Parsable<'a, A>
where
    Self: Parser<A>,
{
    fn parse_only(&self, s: &'a [u8], o: usize) -> Response<()>;
}
