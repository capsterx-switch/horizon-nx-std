#!/bin/sh
set -e

function finish {
    git checkout master || true
}

trap finish EXIT

git branch -D squashed || true
git checkout --orphan squashed
git commit -m 'squash' >/dev/null
git push --force -u squashed HEAD:master
