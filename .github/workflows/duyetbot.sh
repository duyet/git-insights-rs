echo "Input: $@"

if [[ "$@" =~ ^\@duyetbot[[:space:]]https://github.com/[^[:space:]]*$ ]]; then
    echo "Runnning on $2"
    cargo run --release $2
else
    echo "Bad input"
fi
