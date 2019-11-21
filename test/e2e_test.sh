#!/usr/bin/env bash
set -e

SERVER_PORT=9991
FILTER_PORT=9992

cargo build

function tag {
  local TAG=$1
  while read LINE
  do
    echo "$TAG" "$LINE"
  done
}

python3 -u test-server.py 127.0.0.1 "$SERVER_PORT" 2>&1 | tag "[SERVER]" &

RUST_BACKTRACE=1 \
RUST_LOG='jsonrpc_filter=trace' \
../target/debug/jsonrpc-filter \
  --allowed-list ./test_allowed.txt \
  --bind 127.0.0.1 --port $FILTER_PORT \
  --forward "http://127.0.0.1:$SERVER_PORT" 2>&1 | tag "[FILTER]" &

function finish {
  set +e
  kill $(jobs -p)
  wait
}
trap finish EXIT

function req {
  curl -s -H "Content-Type: application/json" -d "$1" localhost:$FILTER_PORT
}

function testcase {
  local METHOD=$1
  local EXPECTED=$2
  local RESULT
  echo "running" "$METHOD"
  RESULT=$($METHOD)
  if python3 ./json_cmp.py "$RESULT" "$EXPECTED"
  then
    echo "success"
  else
    exit 255
  fi
}

sleep 1

function method_ping {
  req '{"jsonrpc":"2.0","id":1,"method": "ping","params":[]}'
}
testcase method_ping '{"jsonrpc":"2.0","result":"pong","id":1}'

function method_add {
  req '{"jsonrpc": "2.0","id":1,"method":"add","params":[1,2,3,4,5]}'
}
testcase method_add '{"jsonrpc":"2.0","result":15.0,"id":1}'

function method_echo {
  req '{"jsonrpc":"2.0","id":1,"method":"echo","params":["hello, ","world!"]}'
}
testcase method_echo '{"jsonrpc":"2.0","result":["hello, ","world!"],"id":1}'


function method_concat {
  req '{"jsonrpc":"2.0","id":1,"method":"concat","params":["hello, ","world!"]}'
}
testcase method_concat '{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":1}'
