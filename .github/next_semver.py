#! /usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
import semver
import re

for s in sys.stdin:
    r = re.match('.*v?(\d+\.\d+\.\d+)', s)
    if r:
        cur_ver = semver.VersionInfo.parse(r.group(1))
        print(f"cur_ver={cur_ver}")
        next_ver = cur_ver.bump_patch()
        print(f"next_ver={next_ver}")