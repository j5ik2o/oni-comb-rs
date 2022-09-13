#! /usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
import csv
import re

commit_messages = {'build': 0, 'ci': 0, 'feat': 0, 'fix': 0, 'docs': 0, 'style': 0, 'refactor': 0, 'perf': 0, 'test': 0, 'revert': 0, 'chore': 0}

rules = {'major': ['perf'], 'minor': ['feat', 'revert'], 'patch': ['build', 'ci', 'fix', 'docs', 'style', 'refactor', 'chore']}

cin = csv.reader(sys.stdin, delimiter="\t") # 補足: 開く対象がファイルのときは newline='' をパラメータに追加

def match_append(key, row):
    r = re.match(f"^{key}(.*)?\: (.*)", row[2])
    if r:
        commit_messages[key]+=1

for row in cin:
    for key in commit_messages.keys():
        match_append(key, row)

for k,v in rules.items():
    sum = 0
    for t in v:
        sum += commit_messages[t]
    if sum > 0:
        print(k)
        break