#! /usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
import semver
import re

args = sys.argv

for s in sys.stdin:
    r = re.match('.*v?(\d+\.\d+\.\d+)', s)
    if r:
        cur_ver = semver.VersionInfo.parse(r.group(1))
        if args[1] == "major":
            next_ver = cur_ver.bump_major()
        elif args[1] == "minor":
            next_ver = cur_ver.bump_minor()
        else:
            next_ver = cur_ver.bump_patch()
        print(next_ver)