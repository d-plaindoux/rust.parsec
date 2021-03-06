# Parser Combinator in Rust

[![Build Status](https://travis-ci.org/d-plaindoux/parsec.rust.svg?branch=master)](https://travis-ci.org/d-plaindoux/parsec.rust)
[![unstable](http://badges.github.io/stability-badges/dist/stable.svg)](http://github.com/badges/stability-badges)

# Status

This library is no more maintained. Please see [Celma generalized parsec](https://github.com/d-plaindoux/celma) instead!

# Objective 

A [parser combinator library](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/parsec-paper-letter.pdf)
implementation from scratch in [Rust](https://www.rust-lang.org/en-US/).

# Parsers

## Core definition

A parser is specified by the following `Trait`.

```rust
pub trait Parser<A> {}
```

Since the Parser size is not known Rust does not allow the Trait type to be returned and used as is. For this reason each parser is denoted by a specific
structure (`struct`) and the corresponding `Parser` trait implementation.

## Basic parsers

module `parsecute::parsers::core`

```rust
returns :: A  -> Parser<A> where A: Copy
fail    :: () -> Parser<A>
any     :: () -> Parser<u8>
eos     :: () -> Parser<()>
```

```rust
satisfy   :: self:Parser<A> -> Box<(Fn(&A) -> bool)> -> Parser<A>
do_try    :: Parser<A> -> Parser<A>
lookahead :: Parser<A> -> Parser<A>
```

### Monadic 

module `parsecute::parsers::monadics`

```rust
fmap :: self:Parser<A> -> (Fn(A) -> B) -> Parser<B>
bind :: self:Parser<A> -> (Fn(A) -> Parser<B>) -> Parser<B>
```

### Flow

module `parsecute::parsers::flow`

```rust
then       :: self:Parser<A> -> Parser<B> -> Parser<(A,B)>
or         :: self:Parser<A> -> Parser<A> -> Parser<A>
opt        :: self:Parser<A> -> Parser<Option<A>>
optrep     :: self:Parser<A> -> Parser<Vec<A>>
rep        :: self:Parser<A> -> Parser<Vec<A>>
take_while :: (Fn(&u8) -> bool) -> Parser<Vec<u8>>
take_one   :: (Fn(&u8) -> bool) -> Parser<Option<u8>>
```

## Literals

module `parsecute::parsers::literals`

`char` and `string` data types implement the `do_parse` method.

```rust
digit        :: () -> Parser<char>
letter       :: () -> Parser<char>
float        :: () -> Parser<FloatLiteral>
string_delim :: () -> Parser<StringLiteral>
char_delim   :: () -> Parser<char>
```

# Example

```rust
// item ::= [^,]*
// line ::= item (',' item)*

let atom = || take_while(|c| *c != ',' as u8);
let line = atom().then(','.then(atom()).fmap(|(_,b)| b).optrep());
```

# Benchmarks

The benchmarks were run on a 2016 Macbook Pro, quad core 2,7 GHz Intel Core i7.

## Brute force tests on basic parsers execution

```
test basic_and                ... bench:   7,971,699 ns/iter (+/- 1,005,128) = 131 MB/s
test basic_any                ... bench:     990,096 ns/iter (+/- 550,996) = 1059 MB/s
test basic_do_try             ... bench:     911,593 ns/iter (+/- 58,521) = 1150 MB/s
test basic_fmap               ... bench:   9,200,929 ns/iter (+/- 880,518) = 113 MB/s
test basic_or                 ... bench:  11,342,151 ns/iter (+/- 2,780,405) = 92 MB/s
test basic_skip               ... bench:  11,097,176 ns/iter (+/- 650,653) = 188 MB/s
test literal_delimited_string ... bench:      10,365 ns/iter (+/- 1,332) = 790 MB/s
test literal_float            ... bench:      15,928 ns/iter (+/- 3,338) = 771 MB/s
```

## Json benches

### The Parser 

````rust
fn json_parser<'a>() -> Parsec<'a, JsonValue<'a>> {
    #[inline]
    fn spaces<E, A>(p: E) -> FMap<And<Skip, (), E, A>, ((), A), A> where E: Parser<A> {
        seq!((skip(" \n\r\t".to_string())) ~> (p))
    }

    fn to_str(s: &[u8]) -> &str {
        std::str::from_utf8(s).unwrap()
    }

    #[inline]
    fn object<'a>() -> Parsec<'a, JsonValue<'a>> {
        let attribute = || seq!((seq!((spaces(delimited_string())) <~ (spaces(':')))) ~ (json::<'a>()));
        let attributes = seq!((attribute()) ~ (seq!((spaces(',')) ~> (attribute())).optrep())).opt();
        let parser = seq!(('{') ~> (attributes) <~ (spaces('}'))).fmap(|v| {
            let mut r = HashMap::default();
            if let Some(((k, e), v)) = v {
                r.insert(to_str(k), e);
                for (k, e) in v {
                    r.insert(to_str(k), e);
                }
            }
            JsonValue::Object(r)
        });

        parsec!('a, parser)
    }

    #[inline]
    fn array<'a>() -> Parsec<'a, JsonValue<'a>> {
        let elements = seq!((json::<'a>()) ~ (seq!((spaces(',')) ~> (json::<'a>())).optrep())).opt();
        let parser = seq!(('[') ~> (elements) <~ (spaces(']'))).fmap(|v| {
            if let Some((e, v)) = v {
                let mut r = v;
                r.insert(0, e);
                JsonValue::Array(r)
            } else {
                JsonValue::Array(Vec::default())
            }
        });

        parsec!('a, parser)
    }

    #[inline]
    fn json<'a>() -> Parsec<'a, JsonValue<'a>> {
        let parser = lazy!(
            // This trigger should be done automatically in the next version hiding this ugly parse type impersonation
            spaces(lookahead(any()).bind(|c| {
                match c as char {
                    '{' => object::<'a>(),
                    '[' => array::<'a>(),
                    '"' => parsec!('a, delimited_string().fmap(|v| JsonValue::Str(to_str(v)))),
                    'f' => parsec!('a, "false".fmap(|_| JsonValue::Boolean(false))),
                    't' => parsec!('a, "true".fmap(|_| JsonValue::Boolean(true))),
                    'n' => parsec!('a, "null".fmap(|_| JsonValue::Null())),
                    _   => parsec!('a, float().fmap(|v| JsonValue::Num(v.to_f64()))),
                }
            }))
        );

        parsec!('a, parser)
    }

    parsec!('a,json::<'a>().then_left(spaces(eos())))
}
````

### JSon benches based on Nom data set

Reference: [Nom & al. Benchmarks](https://github.com/Geal/parser_benchmarks/tree/master/json)

```
test json_apache      ... bench:   2,434,180 ns/iter (+/- 192,180) = 51 MB/s
test json_basic       ... bench:       3,960 ns/iter (+/- 374) = 20 MB/s
test json_canada_nom  ... bench:     135,969 ns/iter (+/- 23,376) = 75 MB/s
test json_data        ... bench:     170,537 ns/iter (+/- 103,725) = 74 MB/s
```

### JSon benches based on Pest data set

Reference: [Pest & al. Benchmarks](https://github.com/pest-parser/pest)

```
test json_canada_pest ... bench:  97,576,631 ns/iter (+/- 41,502,590) = 23 MB/s
```

Based on the throughput the referenced Json file is processed building the corresponding
AST in 86ms.

# License

Copyright 2018 D. Plaindoux.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
