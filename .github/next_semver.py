#! /usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
import semver
import re

for s in sys.stdin:
    r = re.match('.*v?(\d\.\d\.\d)', s)
    print(r)
    if r:
        cur_ver = semver.VersionInfo.parse(r.group(1))
        next_ver = cur_ver.bump_patch()
        print(next_ver)