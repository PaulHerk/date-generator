# adp - append dates
Add dates to your sources now!!!

<!--toc:start-->
- [adp - append dates](#adp-append-dates)
  - [1. Usage](#1-usage)
  - [2. Arguments](#2-arguments)
  - [3. Options](#3-options)
    - [Date](#date)
    - [Format](#format)
    - [Day Range](#day-range)
  - [4. Examples](#4-examples)
  - [5. Features](#5-features)
<!--toc:end-->


## 1. Usage
apd [[OPTIONS]](#3-options) --input <INPUT> --start-date <START_DATE> [[OUTPUT]](#2-arguments)

## 2. Arguments
[OUTPUT]  Name of the output file [default: sources.txt]

## 3. Options
Command short/long | Description | Default Value
---|---|---
-i / --input | Input file | REQUIRED
-s / --start-date | The date when to start | REQUIRED
-e / --end-date | The date when to end | 0 (today), [see more here](#date)
-d / --day_range | The range in the day from when to pick values (not inclusive) | "9-20", [see more here](#day-range)
-f / --format | The format for the output | "[day].[month].[year] [hour]:[minute]", [see more here](#format)
-h / --help | Print help
-v / --version | Print version

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


## 4. Examples
1. `apd -i input.txt -s -7`
  > start last week, end now
2. `apd -i sources.txt -s 2023-02-01_00 -e -1 output.txt`
  > start on the 01.02.2023 at 00:00, end yesterday, output is output.txt


## 5. Features
- [x] ~~Append random dates~~
- [x] ~~Format customization~~
- [x] ~~Relative dates~~
- [ ] Remove all .unwrap()
- [x] ~~No night mode~~
- [ ] create a website for it
- [x] read input so a external file isn't needed
- [x] optimise links
