#![allow(unused_variables)]
use std::marker::PhantomData;
use std::ops::*;
// The following is old code.
#[derive(Copy,Clone)]
pub struct Parser<T>(PhantomData<T>);

impl <T> Parser<T> {
    fn new() -> Parser<T> {
        Parser(PhantomData)
    }
}

pub struct Applicative<T>(T);

impl<T,S> Mul<Parser<S>> for Parser<T> {
    type Output = Applicative<(Parser<T>,Parser<S>)>;
    fn mul(self, rhs:Parser<S>) -> Applicative<(Parser<T>,Parser<S>)> {
        Applicative((self, rhs))
    }
}

impl<T,S,U> Mul<Parser<U>> for Applicative<(Parser<T>,Parser<S>)> {
    type Output = Applicative<(Parser<T>,Parser<S>,Parser<U>)>;
    fn mul(self, rhs: Parser<U>) -> Applicative<(Parser<T>,Parser<S>,Parser<U>)> {
        Applicative(((self.0).0, (self.0).1, rhs))
    }
}

impl<T,S,U,V> Mul<Parser<V>> for Applicative<(Parser<T>,Parser<S>,Parser<U>)> {
    type Output = Applicative<(Parser<T>,Parser<S>,Parser<U>,Parser<V>)>;
    fn mul(self, rhs: Parser<V>) -> Applicative<(Parser<T>,Parser<S>,Parser<U>,Parser<V>)> {
        Applicative(((self.0).0, (self.0).1, (self.0).2, rhs))
    }
}

impl<T,S,F:Fn(T)->S> BitXor<F> for Parser<T> {
    type Output = Parser<S>;
    fn bitxor(self, rhs:F) -> Parser<S> {
        Parser(PhantomData)
    }
}

impl<T,S,U,F:Fn(T,S)->U> BitXor<F> for Applicative<(Parser<T>,Parser<S>)> {
    type Output = Parser<U>;
    fn bitxor(self, rhs:F) -> Parser<U> {
        Parser(PhantomData)
    }
}

impl<T,S,U,V,F:Fn(T,S,U)->V> BitXor<F> for Applicative<(Parser<T>,Parser<S>,Parser<U>)> {
    type Output = Parser<V>;
    fn bitxor(self, rhs:F) -> Parser<V> {
        Parser(PhantomData)
    }
}

impl<T,S> Shr<Parser<S>> for Parser<T> {
    type Output = Parser<S>;
    fn shr(self, rhs: Parser<S>) -> Parser<S> {
        rhs
    }
}
