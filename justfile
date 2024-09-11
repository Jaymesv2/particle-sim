default:
    @just --list

run *ARGS:
    cargo run {{ARGS}}

watch *ARGS:
    cargo watch -x "run -- {{ARGS}}"
