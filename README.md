# clean_subs

This program tries to clean SDH, closed captions and font coloring from .srt files.
The cleaned file will be named the same as the original file and the original is renamed with .bak suffix

## Quick start
```
cargo run [subfile]
```

TODO:
- make default not "in-place"
- implement flags
    - i: in-place
    - q: quiet
