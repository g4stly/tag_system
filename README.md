# tag_system
[tag system](https://en.wikipedia.org/wiki/Tag_system) emulator written in rust

## usage
tag systems are defined in JSON.
the program accepts as an argument the filename to read the definition in; 
in the abscence of an argument it'll read the definition from stdin. 
both of the following are valid:

- `cargo run ./examples/collatz`
- `cat ./examples/collatz | cargo run`
## input
taken from `./examples/collatz`
```json
{
	"init":"aaa",
	"token_size": 2,
	"halt_symbol": "H",
	"productions": [
		{"t":"a", "w":"bc"},
		{"t":"b", "w":"a"},
		{"t":"c", "w":"aaa"}
	]
}
```
* `init`: initial word (`String`)
* `token_size`: number of symbols to delete each cycle (`usize`)
* `halt_symbol`: symbol that will halt execution (`char`)
  * execution will also halt if there are < `token_size` characters left
* `productions`: array of production rules
  * `t`: the symbol to recognize (`char`)
  * `w`: word to append when symbol is recognized (`String`)
  
## output
`cat ./examples/collatz | cargo run` will yeild
```
000: aaa
001: abc
002: cbc
003: caaa
004: aaaaa
005: aaabc
006: abcbc
007: cbcbc
008: cbcaaa
009: caaaaaa
00A: aaaaaaaa
00B: aaaaaabc
00C: aaaabcbc
00D: aabcbcbc
00E: bcbcbcbc
00F: bcbcbca
010: bcbcaa
011: bcaaa
012: aaaa
013: aabc
014: bcbc
015: bca
016: aa
017: bc
018: a
```
