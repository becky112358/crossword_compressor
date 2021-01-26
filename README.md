# Crossword Compressor

This code takes a selection of words and compresses them into a crossword.

## Technologies

Rust

## Method

This code considers every possible way that the selection of words can be put together in a crossword. It outputs the 'best' crossword options. Crossword A is considered to be better than crossword B if the size of crossword A can fit inside the size of crossword B.

## Limitations

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

