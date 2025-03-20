# Revamped Record-Kinded `Format` Definitions

After the removal of the `Format::Record` primitive (#239), the approach taken to
specifying Record-kinded `Format` definitions has changed fundamentally.

This document is intended to be half-documentation, half-guide, as to how to approach
Record-formats after the redesign.

## Changes In #239

> Removes `Format::Record` entirely, and introduces two new `Format` variants to serve for record-construction purposes:
>
> - `MonadSeq`: Variation of `LetFormat` that explicitly discards one parse and yields the result of the other (i.e. without binding to a variable)
> - `Hint`: Used to associate a particular informational bread-crumb with a `Format` node, so that deep introspection and complex heuristics are not needed to identify the meta-structural properties of a `Format`. Currently used to wrap the first bind-or-sequence of a Record construction, so that it can be identified as being Record-style far more simply.
>
> Several supporting types/methods have been added, and some existing methods modified, in light of this redesign:
>
> - Added type: `FieldLabel`, which is used to represent the capture-level details of a pseudo-field in a new-style (including old-style) record format
> - Added type: `RecordFormat`, which is used to encapsulate the list of pseudo-field (as labels and formats) of any new-style record format>

## Old-Style vs New-Style

This shift introduces a slight distinction to Record-kinded Formats, between **old-style** and **new-style**.

### Old-Style

Old-style records bind (and persist, at value-level) every pseudo-field (i.e. each LHS-format of a `LetFormat`) with an associated name, and the names/order of the individual pseudo-fields dictate the names/order of the persisted record-fields.

### New-Style

New-style records are a super-set of old-style-records, in which any of the following are permitted (and the presence of at least one means that the record is new-style)

- Unbound pseudo-fields (i.e. any use of `MonadSeq` in the right-associative spine)
- Ephemeral pseudo-fields (those which are bound in order to be used in computations or dependent-parses later on, but which are not stored in the final Record)
- Record field re-ordering or re-naming (the field-names and order-of-definition of the fields of the record vary, in any way, from the binding-names and nesting-order).

(All old-style records are new-style records, but in some cases 'new-style' is used to refer to cases that do not fall into old-style.)

## Hinting

A side-effect of the change in Record specification, is a new complexity introduced
in format-processing:

```rust
let f = Format::record([...]);
process_format(f)
```

If `process_format` is a function that previously had custom handling of `Format::Record`,
there is now no directly analogous pattern-match that can identify its argument as being
constructed via `Format::record` (i.e. as an old-style record). Even if we are able,
through recursive introspection, to determine that some node is the minimal ancestor containing all `LetFormat`-bound identifiers above a `Format::Compute(Expr::Record(...))`
node, even that is not good enough to determine what the 'true' root is intended to be;
there is no practical distinction between the following two constructions:

```rust
let f0 = chain(f, "not_part_of_record", Format::record(xs));
// the line below uses theoretical methods and conventions that aren't part of the source (yet)
let f1 = Format::record_custom([(Ephemeral("part_of_record"), f), ..xs]);
```

In other words, we can no longer reliably answer the question "is a given node `f` the conceptual root of a record-producing subtree".

To combat this, `Format::Hint` is tobe wrapped around each and every Format representing a record we have just constructed, whether old- or new-style, along with a `StyleHint` storing all salient properties that are difficult or impossible to reverse-engineer after-the-fact.

## Usage Patterns

The basic construction used for parsing Records under this model is lightly inspired by Haskell `do` syntax, where the record-construction itself is a pure computation (`Expr`) that uses the value-bindings produced via a sequence of  impure computations (`Format`):

### Example

Before:

```rust
Format::Record(vec![
    ("__x", x_format), // Semi-anonymous
    ("_y", y_format), // Ephemeral
    ("z", z_format(var("_y"))), // Permanent
])
```

Stylized:

```haskell
oldStyle = do
    __x <- xFormat
    _y <- yFormat
    z <- zFormat _y
   compute $ Record { __x = __x, _y = _y, z = z }

newStyle = do
    xFormat
    y <- yFormat
    z <- zFormat y
   compute $ Record { z = z }
```

Verbatim

```rust
// old-style
chain(x_format, "__x",
    chain(y_format, "_y",
        chain(z_format(var("_y")), "z",
            compute(Expr::Record(vec![
                ("__x", var("__x")),
                ("_y", var("_y")),
                ("z", var("z"))
            ])),
        ),
    ),
)

// new-style
monad_seq(x_format,
    chain(y_format, "y",
        chain(z_format(var("y")), "z",
            compute(Expr::Record(vec![
                ("z", var("z"))
            ])),
        )
    )
)
```
