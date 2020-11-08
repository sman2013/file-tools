# File-tools for file operation

## SUBCOMMANDS

### insert

#### USAGE:
```shell script
file-tools insert [FLAGS] [OPTIONS] --dir <dir> --str <str>
```

#### FLAGS:
```shell script
    -h, --help          Prints help information
        --rename-dir    
    -V, --version       Prints version information
```

#### OPTIONS:
```shell script
    -d, --dir <dir>              Directory for to be renamed
        --position <position>    Position about which to be inserted,can be `head`,`middle`,`tail` [default: middle]
        --str <str> 
```

### replace

### USAGE:
 ```shell script
file-tools replace [FLAGS] [OPTIONS] --dir <dir> --src <src>
```

#### FLAGS
```shell script
    -h, --help          Prints help information
        --rename-dir    Rename directory also
    -V, --version       Prints version information
```

#### OPTIONS
```shell script
OPTIONS:
    -d, --dir <dir>    Directory for to be renamed
        --dst <dst>    Replaced by `dst`
        --src <src>    To be replaced `src`
```