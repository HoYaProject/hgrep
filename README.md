# hgrep
HoYa's grep program

# Usage

> $ hgrep [OPTIONS] \<PATTERN> [PATH]

### Display Help
> $ hgrep [-h | --help]

### Search in Directory Only
> $ hgrep [-d | --dir] \<PATTERN>

### Search in File Only
> $ hgrep [-f | --file] \<PATTERN>

### Search in File Contents
> $ hgrep [-n | --name] \<PATTERN>

### Search with Ignoring Case
> $ hgrep [-i | --ignorecase] \<PATTERN>

### Search Recursively
> $ hgrep [-r | --recursive] \<PATTERN>

### Search with the Whole Word
> $ hgrep [-w | --wholeword]

### Search with the All Options
> $ hgrep [-a | --all] \<PATTERN>

> $ hgrep \<PATTERN>

### Configure: Exclude Directory
> $ hgrep [-c | --config] ex_dir \<PATTERN>

### Configure: Exclude File Extension
> $ hgrep [-c | --config] ex_ext \<PATTERN>

### Configure: Include Directory
> $ hgrep [-c | --config] in_dir \<PATTERN>

### Configure: Include File Extension
> $ hgrep [-c | --config] in_ext \<PATTERN>

### Configure: Clear
> $ hgrep [-c | --config] clear \<ANY CHARACTER>

### Display Version
> $ hgrep [-V | --version]
