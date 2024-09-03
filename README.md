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

## Definition of implant and screw positions
Relies on `config.yaml` found in the current folder. The structure is as follows:
```yaml
# Screw positions
screws: [
  3.0, 0.0, #x, y for point 1
  0.0, 10.0, #x, y for point 2
  10.0, 0.0 #x, y for point 3
]

# Implant positions of interest. Suggest the AP and medial extremes.
implant: [
  0.0, 0.0,
  0.0, 5.0,
  5.0, 0.0
]
```
