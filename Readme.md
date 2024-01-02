# no-ecoqos

Remove process from EcoQos.

```
Usage: no-ecoqos.exe [OPTIONS]

Options:
  -i, --id <ID>      List of processes ID (comma separated)
  -n, --name <NAME>  List of processes names (comma separated)
  -v, --verbose      Set as verbose mode
  -q, --quiet        Dont print anything
  -h, --help         Print help
```

# install

```
cargo install --git https://github.com/milkyapps/no-ecoqos
```

# schedule to run periodically

```ps1
powershell> schtasks /create /sc minute /mo 1 /tn "No EcoQos" /tr (get-command no-ecoqos).Path
```

A terminal will "blink" in the screen.