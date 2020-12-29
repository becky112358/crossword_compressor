# Crossword Compressor

This code takes a selection of words and compresses them into a crossword.

## Technologies

Rust

## Limitations

* In src/crossword.rs, `crossable_letters()` returns a vector. It should return an iterator.
* Code is not fully unit tested.
* Code cannot find crossword overlaps with side-by-side interactions. Examples:

```
      w
      examples
example
      e

   d
   d
abcd
  cdef
  c
  c

```

## Usage

Words must be written directly into the code.
If the user has a formation which they wish to be included in the final crossword, this can be set within
`initialise_crossword()`.

