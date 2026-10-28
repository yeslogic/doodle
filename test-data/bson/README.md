# BSON test fixtures

Hand-built BSON documents used to exercise the BSON format definition
(`doodle-formats/src/format/bson.rs`). Every file is a complete, valid top-level
document (`int32` total length, `element*`, trailing `0x00`), and every element uses the
real `type_byte + e_name + value` shape. Each is decoded with `--as-format bson` and
checked against a snapshot in `tests/expected/decode/`.

Tag bytes below are the BSON element type bytes. All integers are little-endian, except
the `ObjectId` timestamp and counter, which are big-endian.

## Document structure

| File | Exercises |
|------|-----------|
| `test_document_empty.bson` | Smallest valid document: length `5` and no elements. |
| `test_document_mixed.bson` | Flat multi-element document mixing `null`, `MinKey`, `MaxKey`, two `bool`s, positive and negative `int32`, and an `ObjectId`. Checks that the element loop dispatches correctly across several tags in one document. |

## Singleton markers (no value payload)

| File | Exercises |
|------|-----------|
| `test_document_null.bson` | `0x0A` null: the element has a name and no value bytes. |
| `test_document_minkey.bson` | `0xFF` MinKey. The tag is `-1` as `i8`, which sorts last in the unsigned tag-ordered `alts`. |
| `test_document_maxkey.bson` | `0x7F` MaxKey. |

## Fixed-width scalars

| File | Exercises |
|------|-----------|
| `test_document_bool.bson` | `0x08` bool, with both `0x01` and `0x00`. |
| `test_document_int32.bson` | `0x10` int32, with a positive and a negative (two's-complement) value. |
| `test_document_int64.bson` | `0x12` int64, with a value that needs more than 32 bits. |
| `test_document_double.bson` | `0x01` double (`1.5`). Currently decoded as raw `u64le` bits, because core has no float support. |
| `test_document_datetime.bson` | `0x09` UTC datetime (int64 milliseconds). |
| `test_document_timestamp.bson` | `0x11` internal Timestamp (increment and seconds, both `u32le`). |
| `test_document_objectid.bson` | `0x07` ObjectId: 4-byte big-endian timestamp, 5 raw bytes, 3-byte big-endian counter. |

## Length-prefixed and string-like values

| File | Exercises |
|------|-----------|
| `test_document_string.bson` | `0x02` string: `int32` length, UTF-8 bytes, trailing NUL. |
| `test_document_string_embedded_null.bson` | `0x02` string whose payload contains an embedded NUL (`"hi\0lo"`). Checks the lenient behaviour: the decoder truncates and does not fail. |
| `test_document_symbol.bson` | `0x0E` Symbol (deprecated), with the same layout as string. |
| `test_document_code.bson` | `0x0D` JavaScript code without scope. |
| `test_document_regex.bson` | `0x0B` regex: two consecutive cstrings, pattern then options. |
| `test_document_dbpointer.bson` | `0x0C` DBPointer (deprecated): a namespace string plus 12 opaque bytes. |
| `test_document_binary.bson` | `0x05` binary, generic subtype `0x00`. |
| `test_document_binary_old.bson` | `0x05` binary, subtype `0x02` ("Binary (Old)"), which has a redundant inner length field. |

## Recursion (embedded documents, arrays, scoped code)

These exercise the `element` / `document` / `array` mutual-recursion batch.

| File | Exercises |
|------|-----------|
| `test_document_nested.bson` | `0x03` embedded document, one level deep. |
| `test_document_nested_deep.bson` | Three levels of `0x03` nesting (`l1` → `l2` → `l3`), with a scalar at every level. |
| `test_document_array.bson` | `0x04` array with index-string keys (`"0"`, `"1"`). The elements are an `int32` and a string, so it also checks that array elements may have mixed types. |
| `test_document_code_w_scope.bson` | `0x0F` code with scope: total length, code string, then a scope document (a second use of the `document` recursion). |

## Mixed-recursion stress (Phase 8)

These interleave all three recursive entry points (`document`, `array`, `code_w_scope`) at depth.
That is the shape that stresses the 3-way recursion batch.

| File | Exercises |
|------|-----------|
| `test_document_mixed_nesting.bson` | Six levels of nested bodies that go through every recursive kind: document → array → document → code_w_scope → scope document → array → document. |
| `test_document_array_nested.bson` | Arrays of arrays, three levels deep, with a document inside. Also includes an empty array nested in a document that is an array element. |
| `test_document_scope_nested.bson` | `code_w_scope` whose scope contains another `code_w_scope`, whose scope contains an embedded document. Checks nested `slice` boundaries on both total-length fields. |
