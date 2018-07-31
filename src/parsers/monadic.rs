use parsers::core::*;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Monadic
// -------------------------------------------------------------------------------------------------

pub struct Join<A> { p: Parsec<Parsec<A>> } // How this Box of Box can be simplified ?

impl<A> ParserTrait<A> for Join<A> {
    fn do_parse(&self, s: &str, o: usize) -> Response<A> {
        match self.p.do_parse(s, o) {
            Response::Success(a1, i1, b1) => {
                match a1.do_parse(s, i1) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2),
                }
            }
            Response::Reject(b1) => Response::Reject(b1)
        }
    }
}

#[inline]
pub fn join<A>(p: Parsec<Parsec<A>>) -> Join<A> {
    Join { p }
}

#[macro_export]
macro_rules! join {
    ( $x:expr ) => {
        join(Box::new($x))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct FMap<A, B> { f: Box<Fn(A) -> B>, p: Parsec<A> } // Can we remove this Box

impl<A, B> ParserTrait<B> for FMap<A, B> {
    fn do_parse(&self, s: &str, o: usize) -> Response<B> {
        match self.p.do_parse(s, o) {
            Response::Success(a, i, b) => Response::Success((self.f)(a), i, b),
            Response::Reject(b) => Response::Reject(b)
        }
    }
}

#[inline]
pub fn fmap<A, B>(f: Box<Fn(A) -> B>, p: Parsec<A>) -> FMap<A, B> {
    FMap { f, p }
}

#[macro_export]
macro_rules! fmap {
    ( $f:expr , $x:expr ) => {
        fmap(Box::new($f), Box::new($x))
    };
}

// -------------------------------------------------------------------------------------------------

pub struct Bind<A, B> { f: Box<Fn(A) -> Parsec<B>>, p: Parsec<A> } // Can we remove this Box

impl<A, B> ParserTrait<B> for Bind<A, B> {
    fn do_parse(&self, s: &str, o: usize) -> Response<B> {
        match self.p.do_parse(s, o) {
            Response::Reject(b1) => Response::Reject(b1),
            Response::Success(a1, i1, b1) => {
                match (self.f)(a1).do_parse(s, i1) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(b2) => Response::Reject(b1 || b2),
                }
            }
        }
    }
}

#[inline]
pub fn bind<A, B>(f: Box<Fn(A) -> Parsec<B>>, p: Parsec<A>) -> Bind<A, B> {
    Bind { f, p }
}

#[macro_export]
macro_rules! bind {
    ( $f:expr , $p:expr ) => {
        bind(Box::new($f), Box::new($p))
    };
}
