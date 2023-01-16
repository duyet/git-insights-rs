echo "Input: $@"

if [[ "$@" =~ ^\@duyetbot[[:space:]]https://github.com/[^[:space:]]*$ ]]; then
    echo "Runnning on $2"

    # https://trstringer.com/github-actions-multiline-strings/
    echo "OUTPUT<<EOF" >> $GITHUB_ENV
    cargo run --release -q $2 >> $GITHUB_ENV
    echo "EOF" >> $GITHUB_ENV
else
    echo "Bad input"
fi
