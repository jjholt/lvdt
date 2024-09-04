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


# Making the validation data manageable
The following snippet uses awk to take every 10th line of each file. The code goes through all `.csv` in the current directory, finds a number in the title and uses that as a name for the new file.
The original data is moved into a `raw` folder.
```bash
mkdir -p raw && ls *.csv | xargs -I {} sh -c '
  file={}
  number=$(echo "$file" | rg -oP "\d+")
  awk "NR == 10" "$file" > "${number}.csv"
 mv $file raw/'
```
