create day:
    cargo generate --path ./daily-template --name {{day}}
    just get-input {{day}}

get-input day:
    python scripts/get_aoc_input.py --day {{day}}

set dotenv-load := true