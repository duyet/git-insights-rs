#!/bin/bash

main=README.md
temp=/tmp/README.md
temp2=/tmp/temp2.md
demo_repo=https://github.com/duyet/git-insights-rs
demo_repo_dir=/tmp/git-insights-rs

BEGIN_GEN=$(cat $main | grep -n '<!-- BEGIN INSTALLATION -->' | sed 's/\(.*\):.*/\1/g')
END_GEN=$(cat $main | grep -n '<!-- END INSTALLATION -->' | sed 's/\(.*\):.*/\1/g')

cat <(head -n $(expr $BEGIN_GEN) $main)                                 > $temp
echo '```bash'                                                          >> $temp
echo '$ cargo install --git https://github.com/duyet/git-insights-rs'   >> $temp
echo '$ insights --help'                                                >> $temp
echo ''                                                                 >> $temp
cargo run -q -- --help                                                  >> $temp
echo '```'                                                              >> $temp
cat <(tail -n +$(expr $END_GEN) $main)                                  >> $temp


BEGIN_GEN=$(cat $temp | grep -n '<!-- BEGIN DEMO -->' | sed 's/\(.*\):.*/\1/g')
END_GEN=$(cat $temp | grep -n '<!-- END DEMO -->' | sed 's/\(.*\):.*/\1/g')

cat <(head -n $(expr $BEGIN_GEN) $temp)                                            > $temp2
echo '```bash'                                                                     >> $temp2
echo '$ git clone https://github.com/duyet/git-insights-rs /tmp/git-insights-rs'   >> $temp2
echo '$ insights $demo_repo_dir'                                                   >> $temp2
echo '```'                                                                         >> $temp2
echo ''                                                                            >> $temp2
echo 'Output:'                                                                     >> $temp2
echo ''                                                                            >> $temp2
echo '```'                                                                         >> $temp2
git clone $demo_repo $demo_repo_dir &>/dev/null
cargo run -q -- $demo_repo_dir                                                     >> $temp2
echo '```'                                                                         >> $temp2
cat <(tail -n +$(expr $END_GEN) $temp)                                             >> $temp2

cat $temp2
cat $temp2 > $main
