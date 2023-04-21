# Binary format compilation experiment

An experiment into compiling binary format descriptions to decoders with a
fixed amount of lookahead. This is related work on our data description
language, [Fathom](https://github.com/yeslogic/fathom).

## Usage

Decoding files using the CLI:

```sh
cargo run test2.jpg
```

Viewing decoded data on the web frontend (requires Python):

```sh
cargo run -- --output=json test2.jpg > frontend/test.json
python3 -m http.server --directory frontend
```
