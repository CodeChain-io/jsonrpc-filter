#!/usr/bin/env python3
import sys
import os
import json

if len(sys.argv) != 3:
    print("Reqruie 2 arguments", file=sys.stderr)
    sys.exit(-2)

try:
    obj1 = json.loads(sys.argv[1])
    obj2 = json.loads(sys.argv[2])
except:
    print("Invalid arguments", file=sys.stderr)
    print("argv[1]:", sys.argv[1], file=sys.stderr)
    print("argv[2]:", sys.argv[2], file=sys.stderr)
    sys.exit(-2)

if obj1 != obj2:
    print("json is different", file=sys.stderr)
    print("left:", obj1, file=sys.stderr)
    print("right:", obj2, file=sys.stderr)
    sys.exit(-1)
