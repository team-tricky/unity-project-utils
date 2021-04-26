#!/usr/bin/env python
"""
This script removes trash *.meta and *.orig files recursively.
Works with python2+ and any os.

Using as a precomit hook:

[hooks]
pretxncommit.whitespace = python Utils/clean_project.py Assets/Scripts
"""

import os
import sys


IGNORE_DIRNAMES = ('.git', '.hg', '.hgcheck', 'Library', 'obj')
EXTENTIONS_TO_REMOVE = ('.meta',)


def clean_meta(path):
    for child_path in os.listdir(path):
        if child_path in IGNORE_DIRNAMES:
            continue

        full_path = os.path.join(path, child_path)

        if os.path.isdir(full_path):
            clean_meta(full_path)
        elif os.path.isfile(full_path):
            if child_path.endswith(EXTENTIONS_TO_REMOVE):
                dirname, _ = os.path.splitext(full_path)
                if not os.path.exists(dirname):
                    print("Remove meta file '{}'".format(full_path))
                    os.remove(full_path)

    if len(os.listdir(path)) == 0:
        print("Empty path: {}".format(path))
        os.rmdir(path)


def remove_orig(path):
    for root, dirs, fnames in os.walk(path):
        for fname in fnames:
            if fname.endswith('.orig'):
                full_path = os.path.join(root, fname)
                print("Remove orig file '{}'".format(full_path))
                os.remove(full_path)


if __name__ == '__main__':
    root = os.path.abspath(sys.argv[1])
    remove_orig(root)
    clean_meta(root)
