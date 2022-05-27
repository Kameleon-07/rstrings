## <p align="center">`rstrings`</p>



### <p align="center">`rstrings` is tool that allows you to print n long sequence of printable characters appearing side by side inside **any** file.</p>
<br>
<p align="center"><img src="https://github.com/Kameleon-07/rstrings/blob/main/previews/noflags.gif"></p>

## Usage

#### rstrings (file) (other-options)

|Option|Short|Explanation|
|------|-----|-----------|
|show-index|i|Shows index of the first character in the sequence|
|no-color|None|Choosing that will result printing output without colorizing|
|sequence-length|n|Specifies minimum length of the printable characters sequence (default 4)|

<br>

## Requirements

* any linux distro (tested on ArchLinux)
* rust compiler

## Installation
```
git clone https://github.com/Kameleon-07/rstrings.git
cd rstrings
cargo install --path .
strip ~/.cargo/bin/rstrings
```

#### Or more manually:

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

### Changing minimum sequence length
<p align="center"><img src="https://github.com/Kameleon-07/rstrings/blob/main/previews/n.gif"></p>

<br>

### Show index of first char in sequence
<p align="center"><img src="https://github.com/Kameleon-07/rstrings/blob/main/previews/showindex.gif"></p>
