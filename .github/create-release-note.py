#! /usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
import csv
import re

commit_messages = {'build': [], 'ci': [], 'feat': [], 'fix': [], 'docs': [], 'style': [], 'refactor': [], 'perf': [], 'test': [], 'revert': [], 'chore': []}

titles = {'build': 'Build Systems', 'ci': 'Continuous Integration', 'feat': 'Features', 'fix': 'Bug Fixes', 'docs': 'Documentation',
         'style': 'Styles', 'refactor': 'Code Refactoring', 'perf': 'Performance Improvements', 'test': 'Tests',
         'revert': 'Reverts', 'chore': 'Chores'}

cin = csv.reader(sys.stdin, delimiter="\t") # 補足: 開く対象がファイルのときは newline='' をパラメータに追加

def match_append(key, row):
    r = re.match(f"^{key}(.*)?\: (.*)", row[2])
    if r:
        commit_messages[key].append((r.group(2), row))

for row in cin:
    for key in commit_messages.keys():
        match_append(key, row)

for k,v in titles.items():
    if commit_messages[k]:
        print(f"### {v}\n")
    for messages in commit_messages[k]:
        print(f"* {messages[0]} ({messages[1][0]})")
    if commit_messages[k]:
        print()
