#!/Users/quan/.pyenv/versions/script/bin/python3
import os
from pathlib import Path

import requests
import argparse

CURRENT_PATH = Path(".")

def download_aoc_input():
    parser = argparse.ArgumentParser()
    parser.add_argument("--day")
    args = parser.parse_args()

    day = str(args.day)
    session = os.environ.get("SESSION")
    num = "".join(n for n in day if n.isdigit())
    response = requests.get(
        f"https://adventofcode.com/2024/day/{num}/input",
        cookies={
            "session": session
        }
    )
    with open(CURRENT_PATH / day / "src" / "input.txt", "w+") as f:
        f.write(response.text)


if __name__ == '__main__':
    download_aoc_input()