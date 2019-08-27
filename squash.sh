#!/bin/sh
set -e
git branch -d squashed || true
git checkout --orphan squashed
git commit -m 'squash'
git push --force -u squashed master
git checkout master
