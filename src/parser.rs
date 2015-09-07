#![allow(unused_variables)]
use std::ops::*;
pub use ::std::marker::PhantomData;

pub type ParserResult<'a, T> = Result<(T, &'a [u8]), String>;
pub type Lifetime<'a> = ::std::marker::PhantomData<&'a ()>;

pub trait Parser<'a>: Sized {
    type O;
    fn parse_b(&self, &'a [u8]) -> ParserResult<'a, Self::O>;

    fn parse(&self, s: &'a str) -> ParserResult<'a, Self::O> {
        self.parse_b(s.as_bytes())
    }

    fn bind<Q, F>(self, f: F) -> Bind<'a, Self, Q, F>
        where Q: Parser<'a>, F: Fn(Self::O) -> Q
    {
        Bind(self, f, PhantomData)
    }

    fn map<F, Output>(self, f: F) -> Map<'a, Self, F, Output>
        where F: Fn(Self::O) -> Output
    {
        Map(self, f, PhantomData)
    }

    fn seq<P: Parser<'a>>(self, p: P) -> Seq<'a, Self, P> {
        Seq(self, p, PhantomData)
    }
}

impl<'a, 'b, T: Parser<'a>> Parser<'a> for &'b T {
    type O = T::O;
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, Self::O> {
        (*self).parse_b(s)
    }
}

pub fn ok<T>(t:T, s:&[u8]) -> ParserResult<T> {
    Ok((t, s))
}

#[derive(Copy, Clone)]
pub struct Return<T>(T);

impl<'a, T: Clone> Parser<'a> for Return<T> {
    type O = T;
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, T> {
        ok(self.0.clone(), s)
    }
}

pub fn ret<T: Clone>(t: T) -> Return<T> {
    Return(t)
}

#[derive(Copy, Clone)]
pub struct Char(u8);

impl<'a> Parser<'a> for Char{
    type O = ();
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, Self::O> {
        match s.first() {
            None             => Err(format!("Expected char {}, but ran out of input.", self.0)),
            Some(&c) if c == self.0  => ok((), &s[1..]),
            Some(&c) => Err(format!("Expected char {}, but found {}.", self.0, c))
        }
    }
}

pub fn char(c: u8) -> Char {
    Char(c)
}

#[derive(Copy, Clone)]
pub struct Chars(&'static [u8]);

impl<'a> Parser<'a> for Chars {
    type O = ();
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, Self::O> {
        let len = self.0.len();
        if len != s.len() {
            return Err(format!("Expeted {}, but not enough input.", ::std::str::from_utf8(self.0).unwrap()));
        }

        for i in 0..len {
            if (self.0)[i] != s[i] {
                return Err(format!("Expected {}, but found {}", ::std::str::from_utf8(self.0).unwrap(),
                                                                ::std::str::from_utf8(&s[0..i+1]).unwrap()))
            }
        }
        ok((), &s[len..])
    }
}

pub fn string(s: &'static str) -> Chars {
    Chars(s.as_bytes())
}

#[derive(Copy, Clone)]
pub struct Seq<'a, T, S>(T, S, Lifetime<'a>)
    where T: Parser<'a>,
          S: Parser<'a>;

impl<'a, T, S> Parser<'a> for Seq<'a, T, S>
    where T: Parser<'a>, S: Parser<'a>
{
    type O = S::O;
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, Self::O> {
        match self.0.parse_b(s) {
            Ok((_, remaining)) => self.1.parse_b(remaining),
            Err(s) => Err(s)
        }
    }
}

#[derive(Copy, Clone)]
pub struct Bind<'a, P, Q, F>(P, F, Lifetime<'a>)
    where P: Parser<'a>,
          Q: Parser<'a>,
          F: Fn(P::O) -> Q;

impl<'a, P, Q, F> Parser<'a> for Bind<'a, P, Q, F>
    where P: Parser<'a>,
          Q: Parser<'a>,
          F: Fn(P::O) -> Q
{
    type O = Q::O;
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, Self::O> {
        let ref p = self.0;
        let ref f = self.1;
        match p.parse_b(s) {
            Ok((o, rest)) => {
                let q = f(o);
                q.parse_b(rest)
            }
            Err(e) => Err(e)
        }
    }
}

pub struct Map<'a, P, F, O> (P, F, Lifetime<'a>)
    where P: Parser<'a>,
          F: Fn(P::O) -> O;

impl<'a, P, F, Output> Parser<'a> for Map<'a, P, F, Output>
    where P: Parser<'a>,
          F: Fn(P::O) -> Output
{
    type O = Output;
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, Self::O> {
        let ref p = self.0;
        let ref f = self.1;
        match p.parse_b(s) {
            Ok((t, rest)) => {
                ok(f(t), rest)
            }
            Err(e) => Err(e)
        }
    }
}

pub struct Map2<'a, P, Q, F, O>(P, Q, F, Lifetime<'a>)
    where P: Parser<'a>,
          Q: Parser<'a>,
          F: Fn(P::O, Q::O) -> O;

impl<'a, P, Q, F, Output> Parser<'a> for Map2<'a, P, Q, F, Output>
    where P: Parser<'a>,
          Q: Parser<'a>,
          F: Fn(P::O, Q::O) -> Output
{
    type O = Output;
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, Self::O> {
        let ref p = self.0;
        let ref q = self.1;
        let ref f = self.2;

        let mut p_o;
        let mut q_o;
        let mut remaining;

        match p.parse_b(s) {
            Ok((t, rest)) => {
                p_o = t;
                remaining = rest;
            },
            Err(e) => return Err(e)
        }

        match q.parse_b(remaining) {
            Ok((t, rest)) => {
                q_o = t;
                remaining = rest;
            },
            Err(e) => return Err(e)
        }

        ok(f(p_o, q_o), remaining)
    }
}

pub trait Applicative<'a, F, O> {
    type M;
    fn map(self, F) ->  Self::M;
}

impl<'a, P, Q, F, O> Applicative<'a, F, O> for (P, Q)
    where P: Parser<'a>,
          Q: Parser<'a>,
          F: Fn(P::O, Q::O) -> O
{
    type M = Map2<'a, P, Q, F, O>;

    fn map(self, f: F) -> Map2<'a, P, Q, F, O>
        where F: Fn(P::O, Q::O) -> O
    {
        Map2(self.0, self.1, f, PhantomData)
    }
}

impl<'a, T, R, S> Shr<T> for Seq<'a, R, S>
    where T: Parser<'a>,
          R: Parser<'a>,
          S: Parser<'a>
{
    type Output = Seq<'a, Self, T>;
    fn shr(self, rhs: T) -> Seq<'a, Self, T> {
        Seq(self, rhs, PhantomData)
    }
}

#[macro_export]
macro_rules! seq {
    // Binding
    ($i:ident = $val:expr; $($t:tt)*) => ({
        $val.bind(move |$i|{ seq!($($t)*) })
    });

    // Sequencing, for effects
    ($v:expr; $($t:tt)*) => ({$v.seq(seq!($($t)*))});

    // Final value
    ($v:expr) => ($v)
}

fn main() {
}
