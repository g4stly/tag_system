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
taken from `./examples/elementary_arithmetic` which either adds or subtracts two signed 3 bit integers
```json
{
	"init":"-      3      2      ",
	"token_size": 7,
	"halt_symbol": " ",
	"productions": [
		{"t":"+", "w":"_     "},
		{"t":"-", "w":"_    "},

		{"t":"1", "w":"1[]    "},
		{"t":"[", "w":" Aa   "},
		{"t":"]", "w":"  Ba  "},

		{"t":"A", "w":""},
		{"t":"B", "w":"    _   "},
		{"t":"a", "w":"vwxtsru"},

		{"t":"2", "w":"2{}    "},
		{"t":"{", "w":" Cb   "},
		{"t":"}", "w":"  Db  "},

		{"t":"C", "w":"   _     "},
		{"t":"D", "w":"    _  "},
		{"t":"b", "w":"wxyutsv"},

		{"t":"3", "w":"3<>    "},
		{"t":"<", "w":" Ec   "},
		{"t":">", "w":"  Fc  "},

		{"t":"E", "w":"   _    "},
		{"t":"F", "w":"    _ "},
		{"t":"c", "w":"xyzvutw"},

		{"t":"0", "w":"0()    "},
		{"t":"(", "w":" Gd   "},
		{"t":")", "w":"  Hd  "},

		{"t":"G", "w":"   _    "},
		{"t":"H", "w":"    _"},
		{"t":"d", "w":"uvwsrqt"},

		{"t":"4", "w":"4!@    "},
		{"t":"!", "w":" Ie   "},
		{"t":"@", "w":"  Je  "},

		{"t":"I", "w":"   _   "},
		{"t":"J", "w":"    "},
		{"t":"e", "w":"tuvrqps"},

		{"t":"5", "w":"5#$    "},
		{"t":"#", "w":" Kf   "},
		{"t":"$", "w":"  Lf  "},

		{"t":"K", "w":"   _  "},
		{"t":"L", "w":"    _     "},
		{"t":"f", "w":"stuqpor"},

		{"t":"6", "w":"6%^    "},
		{"t":"%", "w":" Mg   "},
		{"t":"^", "w":"  Ng  "},

		{"t":"M", "w":"   _ "},
		{"t":"N", "w":"    _    "},
		{"t":"g", "w":"rstponq"},

		{"t":"n", "w":" -6"},
		{"t":"o", "w":" -5"},
		{"t":"p", "w":" -4"},
		{"t":"q", "w":" -3"},
		{"t":"r", "w":" -2"},
		{"t":"s", "w":" -1"},
		{"t":"t", "w":" 0"},
		{"t":"u", "w":" 1"},
		{"t":"v", "w":" 2"},
		{"t":"w", "w":" 3"},
		{"t":"x", "w":" 4"},
		{"t":"y", "w":" 5"},
		{"t":"z", "w":" 6"},

		{"t":"_", "w":""}
	]
}

```
* `init`: initial word (`String`)
* `token_size`: number of symbols to delete each cycle (`usize`)
* `halt_symbol`: symbol that will halt execution (`char`)
* `productions`: array of production rules
  * `t`: the symbol to recognize (`char`)
  * `w`: word to append when symbol is recognized (`String`)
  
note that the initial input to this tag system (`./examples/elementary_arithmetic`) must conform to the grammar
```
| operator |         | add / sub this much |        | to / from this|
{+|-}<six characters>{0,1,2,3,4,5,6}<six characters>{0,1,2,3,4,5,6}<six characters>
```
where 0, 1, 2, and 3 represent themselves, and 4, 5, and 6 represent -1, -2, and -3 respectively.
  
## output
`cat ./examples/elementary_arithmetic | cargo run` out of the box will yeild
```
000: -      3      2      
001: 3      2      _    
002: 2      _    3<>    
003: _    3<>    2{}    
004: >    2{}    
005: }      Fc  
006: Fc    Db  
007: b      _ 
008: _ wxyutsv
009: sv
00A:  -1

```
