# Date Time Helper

## About this Tool

Tries to parse an input from standard input or from first parameter as 
ISO 8601 date or as epoch (milli)seconds and prints the parsed date as
- ISO 8601 datetime
- epoch seconds and
- epoch milliseconds

NOTE:
Numeric values will be handled as epoch seconds if the year of the result is less than 3000.
Otherwise, they will be handled as epoch milliseconds.

## Usage 

```shell
datetime [DATE_TIME]

Arguments:
  [DATE_TIME]  Input to be parsed. If omitted standard input is used

Options:
  -h, --help  Print help
```

### Example

* Parse date time parameter 
```text
$ datetime 1676140630                                                                                              ✔ 
┌────────────────────┬──────────────────────────┐
│ ISO 8601 timestamp │ 2023-02-11T18:37:10.000Z │
├────────────────────┬──────────────────────────┤
│ Epoch seconds      │               1676140630 │
├────────────────────┬──────────────────────────┤
│ Epoch milliseconds │            1676140630000 │
└────────────────────┴──────────────────────────┘
```

* Parse date time from standard input:
```text
$ datetime                                                                                                     127 ✘ 
1676140630
┌────────────────────┬──────────────────────────┐
│ ISO 8601 timestamp │ 2023-02-11T18:37:10.000Z │
├────────────────────┬──────────────────────────┤
│ Epoch seconds      │               1676140630 │
├────────────────────┬──────────────────────────┤
│ Epoch milliseconds │            1676140630000 │
└────────────────────┴──────────────────────────┘
2023-02-11T18:37:10.000Z
┌────────────────────┬──────────────────────────┐
│ ISO 8601 timestamp │ 2023-02-11T18:37:10.000Z │
├────────────────────┬──────────────────────────┤
│ Epoch seconds      │               1676140630 │
├────────────────────┬──────────────────────────┤
│ Epoch milliseconds │            1676140630000 │
└────────────────────┴──────────────────────────┘
```
Press Ctrl-C to exit.
