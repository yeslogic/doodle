# Daedalus Notes

## 3.1 (Primitive Parsers)

* `^<value>` is achievable through `compute`
* `Fail <msg>` requires a refactor we have discussed but never implemented
  * This could be a modification of `Fail` itself, or a wrapper using `StyleHint`
* We don't have a notion of character classes for the parser `$[c]`, but we could emulate this with `ByteSet` constants
* The closest thing to `Match <string>` is `is_bytes`, which stores an N-Tuple ([issue #260](https://github.com/yeslogic/doodle/issues/260) discusses this)

## 3.2 (Sequential Composition)

The syntax `{ P1; ...; Pn }` can be emulated with `LetFormat`/`MonadSeq`.

### Example Translations

```ddl
def Add2 =
  block
    let x = BEUInt64
    let y = BEUInt64
    ^ x + y

def StatementSemi =
  block
    $$ = Statement
    $[';']
```

```rust
let add2 =
    chain(
        base.u64be(), "x",
        chain(
            base.u64be(),
            "y",
            compute(add(var("x"), var("y")))
        )
    );

let statement_semi =
    chain(
        statement, // defined elsewhere,
        "ret",
        monad_seq(
            is_byte(b';'),
            compute(var("ret"))
        )
    );
```

## 3.3 (Parallel Composition)

There is currently no way of performing *unbiased* composition in `doodle`; all parallel compositions
are first-come-first-served and will bias towards the first non-error result.

`<|` is therefore supported, while `|` is not.

## 3.4 (Repetition)

* Kleene Star - `Many <P>` is just `Repeat`, while `Many (1..) <P>` is `Repeat`
* Kleene Star with State - At least some cases of `many (x = s) <P>` can be emulated with `Map(Repeat, LeftFold)`, while `AccumUntil` might be usable in other cases; there may be cases where neither are applicable, in which case a more bespoke `Format` may be required.
* `map (k,v in c) <P>` and `for (x = s; k,v in c) <P>` could be emulated with `ForEach`, at least in certain instances.

## 3.5 (Branching)

* `case-of` (and, by extension, `if-then-else`) appear one-to-one with `Match`

## 3.6 (Coercions)

* There are no format-level types per-se, but various `Expr`s like `AsU64` and family accomplish much-the-same, albeit in a closed class rather with a constructive syntax.
  * `as?` (dynamic safe coercion) is closest to what we have, since `AsU{8,16,32,64}`/`AsChar` are runtime-checked; we currently have nothing of the sort of `as` (static safe) or `as!` (static lossy) coercion

## 5.1 (Bitdata)

Through helpers like `bit_fields_u8` and so forth, which can be defined as-needed, we have a plausible
analogue to the `bitdata` declarations in Daedalus.

However, the current implementation of `BitFieldKind` is somewhat restrictive, in the following ways, compared to `bitdata`:

* It does not support type-coercions (e.g. u8 packed in a u16)
* It does not support fixed-bits checking other than all-zero

These are features that could be added with various caveats, if necessary.

### Examples

```ddl
bitdata Packed where
  x: uint 8
  0xFFFF
  y: uint 8

bitdata Uni where
  value = { get: Packed }
  null  = 0
```

```rust
use doodle::helper::BitFieldKind::*;

let packed = bit_fields_u32([ // <- this is not yet defined
    BitsField { field_name: "x", bit_width: 8 },
    Reserved { bit_width: 16, check_zero: false },
    BitsField { field_name: "y", bit_width: 8 },
]);

let uni = union_nondet([
    ("null", is_bytes(&[0; 4])),
    ("value", packed),
]);
```

This is more of a parse-level directive than a data-type declaration, however,
as while in Daedalus the two are implicitly specified with the same declaration,
in `doodle` the data-type is a synthetic implication of the parse declaration,
and cannot be used in coercions; for that, we would need a separate declaration
of a dependent `u32 -?-> Packed` computation that could then be fed in arbitrary
arguments to interpret as `Packed`.

```rust
use doodle::helper::BitFieldKind::*;
let as_packed = module.define_format_args(
    "Packed-Coerce",
    [(Label::Borrowed("raw"), ValueType::Base(BaseType::U32))],
    cast_u32_bit_fields( // <- also not defined, but furthermore has no archetype
        var("raw"),
        [
            BitsField { field_name: "x", bit_width: 8 },
            Reserved { bit_width: 16, check_zero: false },
            BitsField { field_name: "y", bit_width: 8 },
        ]
    )
);
```

## 5.2 (Automatic ADT Synthesis)

We have no first-class types in the specification language of `doodle`, and all
types are implied through synthesis over the declared formats and expressions.
As a result, type-ascriptions are syntactically unavailable.

Even currently, we can still at least ensure that two parsers have mutually
compatible types, by defining a declaration-check marker-format that we run
through type-checking but discard afterwards:

```rust
let point = module.define_format("types.point", record([("x", base.u8()), ("y", base.u8())]));
let point_x = module.define_format("types.point_x", record([("x", base.u8()), ("y", is_byte(0))]));

let __type_proof = module.define_format(
    "__TYPE_PROOF",
    monad_seq(
        union([point.call(), point_x.call()]),
        /* we can sequence other type-compatibility assertion-formats here as well */
        Format::EMPTY,
    )
);
```

Because every format needs a reified type for the module to be usable, but these
type-ascriptions need not be a bijection, there would be no implicitly-defined
alias `type PointX = Point` as would be synthesized by the corresponding
Daedalus declarations; instead, whichever type-name is preferred would win, and
both formats would receive verbatim-identical type-ascriptions.

While tagged unions in general are supportable, the example given for tagged
unions cannot be constructed in `doodle` because of a lack of support for
auto-recursive and mutually-recursive format-constructs. Implementing these is
not *a priori* impossible, but would require a noticeable investment of effort
into a design to support this, which would most notably require a
termination-rule for otherwise infinitely-recursive type-checking.

## 6 (Lookahead and Stream Manipulation)

The concept of a `Stream` is not first-class within the `Format` model of `doodle`,
though there are various combinators that interact with it.

* `GetStream` does not obviously have a one-to-one equivalent in `doodle`, though constructs that use it may be replicable in other ways
* `Drop n s` does not properly exist as a first-class construction but can be emulated with some degree of ingenuity
* `Take n s` itself is not quite analogous to anything in `doodle`
* `SetStream` does not properly exist as a first-class construction but can be emulated with some degree of ingenuity
* `Chunk n P` is equivalent to `Slice`
* `Lookahead P` is equivalent to `Peek`
* `WithStream` can be emulated using `DecodeBytes` up to a certain degree, where if the parser itself is responsible for determining where the stream ends, there may be issues in capturing the stream into a suitable buffer.

### Example

```ddl
block
  let base = GetStream
  Many block
    let offset = Word64
    let here = GetStream
    SetStream (Drop offset base)
    $$ = ParseObject
    SetStream here
```

```rust
chain(
    Format::Pos,
    "base",
    repeat(
        chain(base.u64be(), "offset",
            with_relative_offset(Some(var("base")), var("offset"), parse_object)
        )
    )
)
```

## 7 (Eager vs. Lazy)

There is currently no support, at any layer, for parse-level laziness in the
`doodle` processing model; there is some value-level laziness involving
constructed sequences, but that is more of an representation-level optimization
than a feature of the processing model, and aside from performance concerns,
nothing would change if it were eliminated.

Multiple paths cannot be explored in parallel, both for unbounded/indefinite-length
repetitions as well as for more explicit alternations over N branches. There are currently only two places where nondeterministic unions are used:

* At the top-level, for alternating between distinct data-formats (e.g. png, OpenType, gzip); and secondly, to allow fallback to uninterpreted bytes during speculative
parsing of zlib-compressed UTF-8 in an `iTXt` PNG-chunk.

The latter usage is more of a band-aid against unwanted parse-failure, and due
to the limitations of the error-propagation model whereby high-confidence
partial-parses of a given top-format are nevertheless rejected altogether when
even a single, possibly trivial component encounters uncaught parse-failure.
Aside from mitigation within the operational model that would allow
determinations such as 'correct format, malformed data', there are ways to get
around this locally without adjusting the model, by using a separate construct
from `UnionNondet` to avoid coupling one form of speculative parsing to the only
version of something like that in the current implementation of `doodle`:

```rust
fn try_with_fallback(f0: Format, f1: Format) -> Format {
    TryWithFallback(Box::new(f0), Box::new(f1))
}

/* .. */

let zlib_utf8txt_or_raw =
    try_with_fallback(
        fmt_variant("valid", zlib_utf8text),
        fmt_variant("invalid", repeat(base.u8())),
    );
```

This can be used in a broader sense, as a more generic 'permit local failure gracefully'
construct:

```rust
fn permit_fail(f: Format, dummy: Expr) -> Format {
    TryWithFallback(Box::new(f), Box::new(compute(dummy)))
}

// vvv Usage Patterns vvv
fn opt_success(f: Format) -> Format {
    permit_fail(fmt_some(f), expr_none())
}

fn try_or_invalid(f: Format) -> Format {
    permit_fail(fmt_variant("valid", f), Expr::Variant("invalid".into(), Expr::UNIT))
}
```
