use super::parser::*;

#[derive(Copy, Clone)]
pub struct Count<'a, P: Parser<'a>>(i32, P, Lifetime<'a>);

impl<'a, P: Parser<'a>> Parser<'a> for Count<'a, P> {
    type O = &'a [u8];
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, &'a [u8]> {
        let ref p = self.1;
        let mut rest = s;
        for _ in 0..self.0 {
            match p.parse_b(s) {
                Ok((_, next)) => {
                    rest = next;
                }
                Err(e) => return Err(e),
            }
        }
        // Calculate offset
        let offset = s.as_ptr() as usize - rest.as_ptr() as usize;
        let parsed = &s[..offset];
        ok(parsed, rest)
    }
}

pub fn count<'a, P: Parser<'a>>(i: i32, p: P) -> Count<'a, P> {
    Count(i, p, PhantomData)
}

// Matches 0/1 or more instances of P and returns the slice.
// Matching 0 results in an empty slice, so be careful.
// The bool represents if 0 matches is valid.
#[derive(Copy, Clone)]
pub struct ManySlice<'a, P: Parser<'a>>(P, bool, Lifetime<'a>);

impl<'a, P: Parser<'a>> Parser<'a> for ManySlice<'a, P> {
    type O = &'a [u8];
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, &'a [u8]> {
        let ref p = self.0;
        let mut rest = s;
        while let Ok((_, next)) = p.parse_b(rest) {
            rest = next;
        }

        if rest.as_ptr() == s.as_ptr() {
            if !self.1 {
                match p.parse_b(s) { // This will retry the first parse and return the same Error
                    Err(e) => return Err(e),
                    _ => unreachable!(),
                }
            }
        }

        // Calculate offset
        let offset = s.as_ptr() as usize - rest.as_ptr() as usize;
        let parsed = &s[..offset];
        ok(parsed, rest)
    }
}

// Matches zero or more occurrences of P.
pub fn many<'a, P: Parser<'a>>(p: P) -> ManySlice<'a, P> {
    ManySlice(p, true, PhantomData)
}

// Matches one or more occurrences of P. Zero matches is an Err.
pub fn many1<'a, P: Parser<'a>>(p: P) -> ManySlice<'a, P> {
    ManySlice(p, false, PhantomData)
}

// Matches 0/1 or more instances of P and returns the vector of results.
// The bool represents if 0 matches is valid.
#[derive(Copy, Clone)]
pub struct ManyVec<'a, P: Parser<'a>>(P, bool, Lifetime<'a>);

impl<'a, P: Parser<'a>> Parser<'a> for ManyVec<'a, P> {
    type O = Vec<P::O>;
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, Self::O> {
        let ref p = self.0;
        let mut rest = s;
        let mut results = Vec::new();
        while let Ok((v, next)) = p.parse_b(rest) {
            results.push(v);
            rest = next;
        }

        if rest.as_ptr() == s.as_ptr() {
            if !self.1 {
                match p.parse_b(s) { // This will retry the first parse and return the same Error
                    Err(e) => return Err(e),
                    _ => unreachable!(),
                }
            }
        }

        ok(results, rest)
    }
}

pub fn collect_many<'a, P: Parser<'a>>(p: P) -> ManyVec<'a, P> {
    ManyVec(p, true, PhantomData)
}

pub fn collect_many1<'a, P: Parser<'a>>(p: P) -> ManyVec<'a, P> {
    ManyVec(p, false, PhantomData)
}

// Succeeds on F one or more times, returning the entire slice.
// Succeeding on 0 is an error. Use OptionSlice.
#[derive(Copy, Clone)]
pub struct TakeWhile<'a, F: Fn(u8) -> bool>(F, Lifetime<'a>);

impl<'a, F: Fn(u8) -> bool> Parser<'a> for TakeWhile<'a, F> {
    type O = &'a [u8];
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, &'a [u8]> {
        let mut offset: usize = 0;
        let ref f = self.0;
        for c in s {
            if !f(*c) {
                break;
            }
            offset = offset + 1;
        }

        if offset == 0 {
            return Err("No match for take-while.".to_string());
        }

        return ok(&s[..offset], &s[offset..]);
    }
}

pub fn take_while<'a, P: Fn(u8) -> bool>(p: P) -> TakeWhile<'a, P> {
    TakeWhile(p, PhantomData)
}

// Will always succeed. Wraps a parser of type T and returns None(T) if it fails.
#[derive(Copy, Clone)]
pub struct Optional<'a, P: Parser<'a>>(P, Lifetime<'a>);

impl<'a, P: Parser<'a>> Parser<'a> for Optional<'a, P> {
    type O = Option<P::O>;
    fn parse_b(&self, s: &'a [u8]) -> ParserResult<'a, Self::O> {
        match self.0.parse_b(s) {
            Ok((v, rest)) => Ok((Some(v), rest)),
            Err(_) => Ok((None, s))
        }
    }
}

pub fn option<'a, P: Parser<'a>>(p: P) -> Optional<'a, P> {
    Optional(p, PhantomData)
}
