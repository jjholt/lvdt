# Usage
Input can either come from stdin by piping 
```bash
cat input/1.csv | lvdt
```
or by specificing the data folder with the `-d` flag:
```bash
lvdt -d=input/1.csv
```

Output defaults to stdout, which means it can be placed in a folder using the typical methods:
```bash
cat input1.csv | lvdt > output/1.csv
```
