use core::marker::PhantomData;
use parsers::basic::*;
use parsers::core::Executable;
use parsers::core::Parser;
use parsers::response::*;

// -------------------------------------------------------------------------------------------------
// Parser type definition
// -------------------------------------------------------------------------------------------------

pub struct Or<E, R, A>(pub E, pub R, PhantomData<A>) where E: Parser<A>, R: Parser<A>;

impl<E, R, A> Parser<A> for Or<E, R, A> where E: Parser<A>, R: Parser<A> {}

pub trait OrOperation<E, R, A> where E: Parser<A>, R: Parser<A> {
    fn or(self, R) -> Or<E, R, A>;
}

impl<E, R, A> OrOperation<E, R, A> for E where E: Parser<A>, R: Parser<A> {
    fn or(self, a: R) -> Or<E, R, A> {
        Or(self, a, PhantomData)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct And<E, A, R, B>(pub E, pub R, PhantomData<A>, PhantomData<B>) where E: Parser<A>, R: Parser<B>;

impl<E, A, R, B> Parser<(A, B)> for And<E, A, R, B> where E: Parser<A>, R: Parser<B> {}

pub trait AndOperation<E, A, R, B> where E: Parser<A>, R: Parser<B> {
    fn then(self, R) -> And<E, A, R, B>;
}

impl<E, A, R, B> AndOperation<E, A, R, B> for E where E: Parser<A>, R: Parser<B> {
    fn then(self, b: R) -> And<E, A, R, B> {
        And(self, b, PhantomData, PhantomData)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Opt<E, A>(E, PhantomData<A>) where E: Parser<A>;

impl<E, A> Parser<Option<A>> for Opt<E, A> where E: Parser<A> {}

pub fn opt<E, A>(p: E) -> Opt<E, A> where E: Parser<A> {
    Opt(p, PhantomData)
}

//  -------------------------------------------------------------------------------------------------

pub struct Repeat<E, A>(bool, E, PhantomData<A>) where E: Parser<A>;

impl<E, A> Parser<Vec<A>> for Repeat<E, A> where E: Parser<A> {}

pub fn optrep<E, A>(p: E) -> Repeat<E, A> where E: Parser<A> {
    Repeat(true, p, PhantomData)
}

pub fn rep<E, A>(p: E) -> Repeat<E, A> where E: Parser<A> {
    Repeat(false, p, PhantomData)
}

//  -------------------------------------------------------------------------------------------------

pub trait RepeatOperation<E, A> where E: Parser<A> {
    fn opt(self) -> Opt<E, A>;
    fn rep(self) -> Repeat<E, A>;
    fn optrep(self) -> Repeat<E, A>;
}

impl<E, A> RepeatOperation<E, A> for E where E: Parser<A> {
    fn opt(self) -> Opt<E, A> {
        opt(self)
    }

    fn rep(self) -> Repeat<E, A> {
        rep(self)
    }

    fn optrep(self) -> Repeat<E, A> {
        optrep(self)
    }
}

//  -------------------------------------------------------------------------------------------------

pub type TypeWhile = Repeat<Satisfy<Try<Any, char>, char>, char>;

pub fn take_while(f: Box<(Fn(&char) -> bool)>) -> TypeWhile {
    optrep(do_try(any()).satisfy(f))
}

pub type TakeOne = Satisfy<Try<Any, char>, char>;

pub fn take_one(f: Box<(Fn(&char) -> bool)>) -> TakeOne {
    do_try(any()).satisfy(f)
}

// -------------------------------------------------------------------------------------------------
// Parser execution
// -------------------------------------------------------------------------------------------------

impl<E, R, A> Executable<A> for Or<E, R, A>
    where E: Executable<A> + Parser<A>,
          R: Executable<A> + Parser<A>
{
    fn execute(&self, s: &str, o: usize) -> Response<A> {
        let Or(p1, p2, _) = self;

        match p1.execute(s, o) {
            Response::Success(a1, i1, b1) => Response::Success(a1, i1, b1),
            Response::Reject(_, b1) => {
                match p2.execute(s, o) {
                    Response::Success(a2, i2, b2) => Response::Success(a2, i2, b1 || b2),
                    Response::Reject(i2, b2) => Response::Reject(i2, b1 || b2)
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<E, A, R, B> Executable<(A, B)> for And<E, A, R, B>
    where E: Executable<A> + Parser<A>,
          R: Executable<B> + Parser<B>
{
    fn execute(&self, s: &str, o: usize) -> Response<(A, B)> {
        let And(p1, p2, _, _) = self;

        match p1.execute(s, o) {
            Response::Success(a1, i1, b1) => {
                match p2.execute(s, i1) {
                    Response::Success(a2, i2, b2) => Response::Success((a1, a2), i2, b1 || b2),
                    Response::Reject(i2, b2) => Response::Reject(i2, b1 || b2),
                }
            }
            Response::Reject(i1, b1) => Response::Reject(i1, b1)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<E, A> Executable<Option<A>> for Opt<E, A>
    where E: Executable<A> + Parser<A>
{
    fn execute(&self, s: &str, o: usize) -> Response<Option<A>> {
        let Opt(p, _) = self;

        match p.execute(s, o) {
            Response::Success(a, o, b) => Response::Success(Some(a), o, b),
            Response::Reject(_, false) => Response::Success(None, o, false),
            Response::Reject(o, true) => Response::Reject(o, true)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<E, A> Executable<Vec<A>> for Repeat<E, A>
    where E: Executable<A> + Parser<A>
{
    fn execute(&self, s: &str, o: usize) -> Response<Vec<A>> {
        let Repeat(opt, p, _) = self;

        let mut result: Vec<A> = Vec::with_capacity(13);
        let mut offset = o;
        let mut consumed = false;
        let mut parsed = true;

        while parsed {
            match p.execute(s, offset) {
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

        if *opt || result.len() > 0 {
            return Response::Success(result, offset, consumed);
        }

        return Response::Reject(offset, consumed);
    }
}

// -------------------------------------------------------------------------------------------------
