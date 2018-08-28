# Parser Combinator in Rust

[![Build Status](https://travis-ci.org/d-plaindoux/parsec.rust.svg?branch=master)](https://travis-ci.org/d-plaindoux/parsec.rust)
[![unstable](http://badges.github.io/stability-badges/dist/unstable.svg)](http://github.com/badges/stability-badges)

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
fmap :: self:Parser<A> -> Box<(Fn(A) -> B)> -> Parser<B>
bind :: self:Parser<A> -> Box<(Fn(A) -> Parser<B>)> -> Parser<B>
```

### Flow

module `parsecute::parsers::flow`

```rust
then       :: self:Parser<A> -> Parser<B> -> Parser<(A,B)>
or         :: self:Parser<A> -> Parser<A> -> Parser<A>
opt        :: self:Parser<A> -> Parser<Option<A>>
optrep     :: self:Parser<A> -> Parser<Vec<A>>
rep        :: self:Parser<A> -> Parser<Vec<A>>
take_while :: Box<(Fn(&u8) -> bool)> -> Parser<Vec<u8>>
take_one   :: Box<(Fn(&u8) -> bool)> -> Parser<Option<u8>>
```

## Literals

module `parsecute::parsers::literals`

`char` and `string` data types implement the `do_parse` method.

```rust
digit        :: () -> Parser<char>
letter       :: () -> Parser<char>
float        :: () -> Parser<f32>
string_delim :: () -> Parser<String>
char_delim   :: () -> Parser<char>
```

# Example

```rust
// item ::= [^,]*
// line ::= item (',' item)*

let atom = || take_while(Box::new(|c| *c != ',' as u8));
let line = atom().then(','.then(atom()).fmap(Box::new(|(_,b)| b)).optrep());
```

# Benchmarks

[Nom & al. Benchmarks](https://github.com/Geal/parser_benchmarks/tree/master/json)

## JSon benches

```
test basic  ... bench:      10,853 ns/iter (+/- 2,584) = 7 MB/s
test canada ... bench:     348,430 ns/iter (+/- 58,229) = 26 MB/s
test apache ... bench:   4,425,182 ns/iter (+/- 420,662) = 28 MB/s
test data   ... bench:     343,244 ns/iter (+/- 72,412) = 26 MB/s
```

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
