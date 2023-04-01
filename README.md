# adp - append dates
Add dates to your sources now!!!
## Usage:
adp -i [IN_FILE] -s [[DATE](#date)] -e [[DATE](#date)] -o [[FORMAT](#format)] [OUT_FILE]

Command short/long | Description | Default Value
---|---|---
-i / --input | Input file | REQUIRED
-s / --start-date | The date when to start | REQUIRED
-e / --end-date | The date when to end | 0 (today)
-o / --out-format | The format for the output | "[day].[month].[year] [hour]:00"
-h / --help | Print help
-v / --verstion | Print version

### Date
There are two different options to represent dates.

#### First
- `YYYY-MM-DD_HH`
- _Example:_ `-s 2023-4-01_18`

#### Second
- `Days from now (signed integers)`
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

For more Information read [this](https://time-rs.github.io/book/api/format-description.html) and you can also find more arguments [here](https://docs.rs/time/0.3.20/time/format_description/modifier/index.html#structs). You can put basically anything in the string.
_Example:_ `-o "[second]:[minute]_[unix_timestamp] [weekday repr:short],[day]"`
