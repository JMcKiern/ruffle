#!/usr/bin/env python3

import argparse
import re
import sys

PREFIXES = "(core|desktop|web|avm1|docs|chore|tests)"
COMMIT_MSG = PREFIXES + ": .*?"

def check_commit_message(msg: str):
    return re.match(COMMIT_MSG, msg) is not None

if __name__=="__main__":
    parser = argparse.ArgumentParser(description="Check commit message matches expected pattern")
    parser.add_argument("commit_message")
    args = parser.parse_args()
    if check_commit_message(args.commit_message):
        sys.exit(0)
    else:
        sys.exit("Error: \"" + msg + "\" does not match pattern")
