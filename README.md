# jsonrpc-filter [![Build Status](https://travis-ci.com/CodeChain-io/jsonrpc-filter.svg?branch=master)](https://travis-ci.com/CodeChain-io/jsonrpc-filter) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
This is a proxy server that allows the only subsets of the RPCs.

## Usage
```
USAGE:
    jsonrpc-filter [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --allowed-list <allowed_list>    The path of the file. [default: allowed.txt]
        --bind <bind>                    The binding address [default: 0.0.0.0]
        --forward <forward>              The uri to forward the JSONRPC requests [default: http://127.0.0.1:8080]
        --port <port>                    The binding port
```

## allowed.txt
This file is a collection of the allowed RPCs.
Each line should have precisely one RPC name without any trailing characters.
