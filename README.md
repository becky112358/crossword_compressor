# Crossword Compressor

This code takes a selection of words and compresses them into a crossword.

## Technologies

Rust

## Limitations

* In src/crossword.rs, `crossable_letters()` returns a vector. It should return an iterator.
* Code is not fully unit tested.
* Code cannot find crossword overlaps of size 2x2 or larger. The code can find overlaps which can be constructed such that at each insertion of one word, the full crossword remains valid. However, for overlaps of size 2x2 or larger, simultaneous insertion of words is required. In future versions this could be resolved with some initial code which specifically checks for square/rectangle overlaps. Example of a 2x2 overlap:

```

     t
     r
relate
    remember
    u
    t
    h

```

## Usage

Words must be written directly into the code.
If the user has a formation which they wish to be included in the final crossword, this can be set within
`initialise_crossword()`.

