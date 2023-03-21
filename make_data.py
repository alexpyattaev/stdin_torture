#!/usr/bin/python
f=open('testlines.txt','w')
for i in range(10000000):
    f.write('Hello world\n')
f.close()
