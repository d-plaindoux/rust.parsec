use parsers::core::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Flow
// -------------------------------------------------------------------------------------------------

pub struct Seq<A, B> { p1: Parsec<A>, p2: Parsec<B> }

impl<A, B> ParserTrait<(A, B)> for Seq<A, B> {
    fn do_parse(&self, s: &str, o: usize) -> Response<(A, B)> {
        match self.p1.do_parse(s, o) {
            Response::Success(a1, i1, b1) => {
                match self.p2.do_parse(s, i1) {
                    Response::Success(a2, i2, b2) => Response::Success((a1, a2), i2, b1 || b2),
                    Response::Reject(i2, b2) => Response::Reject(i2, b1 || b2),
                }
            }
            Response::Reject(i1, b1) => Response::Reject(i1, b1)
        }
    }
}

#[inline]
pub fn seq<A, B>(p1: Parsec<A>, p2: Parsec<B>) -> Seq<A, B> {
    Seq { p1, p2 }
}

#[macro_export]
macro_rules! seq {
    ( $p1:expr ) => { $p1 };
    ( $p1:expr, $($p2:expr),+ )  => { seq(Box::new($p1), Box::new(seq!($($p2),*))) };
}

// -------------------------------------------------------------------------------------------------

pub struct Or<A> { p1: Parsec<A>, p2: Parsec<A> }

impl<A> ParserTrait<A> for Or<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<A> {
        match self.p1.do_parse(s, o) {
            Response::Success(a1, i1, b1) => Response::Success(a1, i1, b1),
            Response::Reject(_, b1) => {
                match self.p2.do_parse(s, o) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(i2, b2) => Response::Reject(i2, b1 || b2) // TODO max i1 o2
                }
            }
        }
    }
}

#[inline]
pub fn or<A>(p1: Parsec<A>, p2: Parsec<A>) -> Or<A> {
    Or { p1, p2 }
}

#[macro_export]
macro_rules! or {
    ( $p1:expr ) => { $p1 };
    ( $p1:expr, $($p2:expr),+ ) => { or(Box::new($p1), Box::new(or!($($p2),*))) };
}

// -------------------------------------------------------------------------------------------------
// Occurrences
// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! opt {
    ( $p:expr ) => {
        or!(fmap!(|a| Some(a), $p), returns(None))
    };
    ( $($p:expr),+ ) => {
        or!(fmap!(|a| Some(a), seq!($($p),*)), returns(None))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Repeat<A> { opt: bool, p: Parsec<A> }

impl<A> ParserTrait<Vec<A>> for Repeat<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<Vec<A>> {
        let mut result: Vec<A> = Vec::with_capacity(13);
        let mut offset = o;
        let mut consumed = false;
        let mut parsed = true;

        while parsed {
            match self.p.do_parse(s, offset) {
                Response::Success(a1, i1, b1) => {
                    result.push(a1);
                    offset = i1;
                    consumed = consumed || b1;
                }
                _ => {
                    parsed = false;
                }
            }
        }

        if self.opt || result.len() > 0 {
            return Response::Success(result, offset, consumed);
        }

        return Response::Reject(offset, consumed);
    }
}

#[inline]
pub fn optrep<A>(p: Parsec<A>) -> Repeat<A> {
    Repeat { opt: true, p }
}

#[macro_export]
macro_rules! optrep {
    ( $($p:expr),+ ) => { optrep(Box::new(seq!($($p),*))) };
}

#[inline]
pub fn rep<A>(p: Parsec<A>) -> Repeat<A> {
    Repeat { opt: false, p }
}

#[macro_export]
macro_rules! rep {
    ( $($p:expr),+ ) => { rep(Box::new(seq!($($p),*))) };
}

// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! take_while {
    ( $x:expr ) => { optrep!(do_try!(satisfy!(any(), $x))) };
}

#[macro_export]
macro_rules! take_one {
    ( $x:expr ) => { do_try!(satisfy!(any(), $x)) };
}
