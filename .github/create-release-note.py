#! /usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
import csv
import re

d = {'build': [], 'ci': [], 'feat': [], 'fix': [], 'docs': [], 'style': [], 'refactor': [], 'perf': [], 'test': [], 'revert': [], 'chore': []}

tiles = {'build': 'Build Systems', 'ci': 'Continuous Integration', 'feat': 'Features', 'fix': 'Bug Fixes', 'docs': 'Documentation',
         'style': 'Styles', 'refactor': 'Code Refactoring', 'perf': 'Performance Improvements', 'test': 'Tests',
         'revert': 'Reverts', 'chore': 'Chores'}

cin = csv.reader(sys.stdin, delimiter="\t") # 補足: 開く対象がファイルのときは newline='' をパラメータに追加

def match_append(key, row):
    r = re.match(f"^{key}(.*)?\: (.*)", row[2])
    if r:
        d[key].append((r.group(2), row))

for row in cin:
    for key in d.keys():
        match_append(key, row)

for k,v in tiles.items():
    if d[k]:
        print(f"### {v}\n")
    for messages in d[k]:
        print(f"* {messages[0]} ({messages[1][0]})")
    if d[k]:
        print()
