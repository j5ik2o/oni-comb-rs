#! /usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
import semver

for s in sys.stdin:
    cur_ver = semver.VersionInfo.parse(s)
    next_ver = cur_ver.bump_patch()
    print(next_ver)