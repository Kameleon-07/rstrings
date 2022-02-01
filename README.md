## <p align="center">`rstrings`</p>



### <p align="center">`rstrings` is tool that allows you to print n long set of printable characters appearing side by side inside **any** file.</p>
<br>
<p align="center"><img src="https://github.com/Kameleon-07/rstrings/blob/main/previews/noflags.gif"></p>

## Usage

#### rstrings (file) (other-options)

|Option|Short|Explanation|
|------|-----|-----------|
|--show-index|o|Prints index of first char in sequence|
|no-color|None|Output will not be colorized|
|n|n|Specifies how long should be sequence of characters (default 4)|

<br>

## Requirements

* any linux distro (tested on arch)
* rust compiler

## Installation

```
git clone https://github.com/Kameleon-07/rstrings.git
cd rstrings
cargo build --release
(optional but recommended) strip target/release/rstrings
sudo cp target/release/rstrings /usr/local/bin
```

## Previews

### No color
<p align="center"><img src="https://github.com/Kameleon-07/rstrings/blob/main/previews/nocolor.gif"></p>

<br>

### Changing default length of printable characters sequence
<p align="center"><img src="https://github.com/Kameleon-07/rstrings/blob/main/previews/n.gif"></p>

<br>

### See index of first char in sequence
<p align="center"><img src="https://github.com/Kameleon-07/rstrings/blob/main/previews/showindex.gif"></p>