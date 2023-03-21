#!/usr/bin/python
import sys,os

import json

harness="time {} < ../testlines.txt"

if not os.path.exists("testlines.txt"):
    print("Test input not found, making it")
    os.system("./make_data.py")
else:
    print("Using existing test data file...")

for lang in ["c", "cpp", "java", "python", "rust"]:
    print("======================")
    print(f"Testing {lang}")
    os.chdir(lang)
    rules = json.load(open("test.json",'rb'))
    for bench, cmds in rules.items():
        print(f"----- Running bench {bench} ------")
        print(f"Using build command: {cmds['build']}")
        os.system(cmds['build'])
        bcmd = harness.format(cmds['run'])
        print(f"Running bench command {bcmd}")
        os.system(bcmd)
    os.chdir('..')
    
print("All done!")
