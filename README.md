# `rxd`

This is just an exploratory repo to play with rust concepts. Inspired by tools 
like `hexdump`, `xxd` and `hexyl` this simply dumps hex values from a file
and (TODO) std input.

## Build

Create a binary by running `cargo build --release` and then place the file in
`target/release/rxd` in your path to run. 

## Usage 

Currently only supports a single file as the first argument like:

```
$ rxd Cargo.toml
00000000 | 5b70 6163 6b61 6765 5d0a 6e61 6d65 203d | [package].name =
00000001 | 2022 7278 6422 0a76 6572 7369 6f6e 203d |  "rxd".version =
00000002 | 2022 302e 312e 3022 0a65 6469 7469 6f6e |  "0.1.0".edition
00000003 | 203d 2022 3230 3231 220a 0a23 2053 6565 |  = "2021"..# See
00000004 | 206d 6f72 6520 6b65 7973 2061 6e64 2074 |  more keys and t
00000005 | 6865 6972 2064 6566 696e 6974 696f 6e73 | heir definitions
00000006 | 2061 7420 6874 7470 733a 2f2f 646f 632e |  at https://doc.
...
```

Also supports piping via std in like `cat somefiles.txt | rxd`

## TODO
- [ ] improve arg parsing - flags, multiple files
- [x] support std input
- [ ] dynamic formatter options - binary not hex, byte groupings (currently 2 bytes wide), line length (default is 16 bytes per line)

As a reminder these are the options supported  by `xxd` which I would like to explore;

```
Usage:
       xxd [options] [infile [outfile]]
    or
       xxd -r [-s [-]offset] [-c cols] [-ps] [infile [outfile]]
Options:
    -a          toggle autoskip: A single '*' replaces nul-lines. Default off.
    -b          binary digit dump (incompatible with -ps,-i,-r). Default hex.
    -C          capitalize variable names in C include file style (-i).
    -c cols     format <cols> octets per line. Default 16 (-i: 12, -ps: 30).
    -E          show characters in EBCDIC. Default ASCII.
    -e          little-endian dump (incompatible with -ps,-i,-r).
    -g          number of octets per group in normal output. Default 2 (-e: 4).
    -h          print this summary.
    -i          output in C include file style.
    -l len      stop after <len> octets.
    -o off      add <off> to the displayed file position.
    -ps         output in postscript plain hexdump style.
    -r          reverse operation: convert (or patch) hexdump into binary.
    -r -s off   revert with <off> added to file positions found in hexdump.
    -s [+][-]seek  start at <seek> bytes abs. (or +: rel.) infile offset.
    -u          use upper case hex letters.
    -v          show version: "xxd V1.10 27oct98 by Juergen Weigert".
  ```
