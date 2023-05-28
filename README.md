A version of cmp which displays the verbose option in byte, hex or ascii. No octal what so ever :).

Work in progress!

```
cmpr 0.1.0
Steven Lalewicz 05-2023
cmpr with ascii, byte or hex output

USAGE:
    cmpr [FLAGS] [OPTIONS] <file1> [file2]

FLAGS:
    -c, --char       list all differences shown as characters
        --help       Prints help information
    -h, --hex        list all differences shown in hex
    -l, --list       list all differences shown in bytes
    -V, --version    Prints version information

OPTIONS:
    -i, --ignore <skip>    skip first n bytes

ARGS:
    <file1>    first file to compare
    <file2>    second file to compare
```
