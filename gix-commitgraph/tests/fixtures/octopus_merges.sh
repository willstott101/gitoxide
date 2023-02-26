#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -q --orphan root
git commit -q --allow-empty -m root

git checkout -q -b parent1 root
git commit -q --allow-empty -m parent1

git checkout -q -b parent2 root
git commit -q --allow-empty -m parent2

git checkout -q -b parent3 root
git commit -q --allow-empty -m parent3

git checkout -q -b parent4 root
git commit -q --allow-empty -m parent4

git checkout -q -b three_parents parent1
git merge -q -m three_parents --no-ff parent2 parent3 >/dev/null

git checkout -q -b four_parents parent2
git merge -q -m four_parents --no-ff parent1 parent3 parent4 >/dev/null

git commit-graph write --no-progress --reachable
git repack -adq
