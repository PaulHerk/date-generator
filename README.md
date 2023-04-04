# adp - append dates
Add dates to your sources now!!!

<!--toc:start-->
- [1. Usage](#1-usage)
  - [Date](#date)
  - [Format](#format)
  - [Day Range](#day-range)
- [2. Examples](#2-examples)
- [3. Features](#3-features)
<!--toc:end-->


## 1. Usage
adp --input [INPUT_FILE] --start-date [[DATE](#date)] --end-date [[DATE](#date)] --format [[FORMAT](#format)] [OUTPUT_FILE]

Command short/long | Description | Default Value
---|---|---
-i / --input | Input file | REQUIRED
-s / --start-date | The date when to start | REQUIRED
-e / --end-date | The date when to end | 0 (today), [see more here](#date)
-d / --day_range | The range in the day from when to pick values (not inclusive) | "9-20", [see more here](#day-range)
-f / --format | The format for the output | "[day].[month].[year] [hour]:[minute]", [see more here](#format)
-h / --help | Print help
-v / --verstion | Print version

### Date
There are two different options to represent dates.

1. **First**
`YYYY-MM-DD_HH`
- _Example:_ `-s 2023-4-01_18`

2. **Second**
`Days from now (signed integers)`
- _Example 1:_ `-s -5` (5 days ago)
- _Example 2:_ `-e 4` (in 5 days)


### Format
You can customize the format as you want with these arguments:

- [year]
- [month]
- [day]
- [hour]
- [minute]
- [second]

For more Information read [this](https://time-rs.github.io/book/api/format-description.html) and you can also find the complete argument list [here](https://docs.rs/time/0.3.20/time/format_description/modifier/index.html#structs). You can put basically anything in the string. <br>

_Example:_ `-o "[second]:[minute]_[unix_timestamp] [weekday repr:short],[day]"`


### Day Range
You can customize the range when to pick the hours with this. 24 is no valid hour.

- _Example 1:_ `-d "5-12"`
 > Choose values from 5:00 to 11:59
- _Example 2:_ `-d "0-0"`
 > If you put the same values twice, it means that every hour is possible
- _Example 3:_ `-d "3-1"`
 > Will fail, because start is bigger than end


## 2. Examples
1. `apd -i input.txt -s -7`
  > start last week, end now
2. `apd -i sources.txt -s 2023-02-01_00 -e -1 output.txt`
  > start on the 01.02.2023 at 00:00, end yesterday, output is output.txt


## 3. Features
- [x] ~~Append random dates~~
- [x] ~~Format customization~~
- [x] ~~Relative dates~~
- [ ] Remove all .unwrap()
- [ ] No night mode
- [ ] Verbose mode
- [ ] Create docker image
- [ ] Create homebrew tab?
