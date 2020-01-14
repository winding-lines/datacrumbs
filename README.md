Datacrumbs is command line utility and conventions to help you track data provenance. The main use
case is when repeatedly running simple commands that generate data. For  more complex flow tracking
you should evaluate a system like [mlflow](https://mlflow.org/)


## Overview

The use case where `datacrumbs` is most useful is when the output of your command is a new folder
during every command execution. The command is providing a very simple chaining capability 
by creating the following files in the output folder:
 
 - `datacrumbs.json` properties about the run
 - `datacrumbs.patch` git diff in the current folder
 
 In order to increase the usefulness of your patch you should save complex shell commands
 into a script and then execute the script.


## Example

Let's assume you have an existing command `generate.sh` with the following usage pattern:

```
generate.sh --input /data/some --use_foo --output /data/out/
```

To integrate `datacrumbs` you defer the output generation to it and save the whole command into a script.

File `do.sh`
``` 
generate.sh --input /data/some --use_foo --output `datacrumbs --base /data/out`
```

Then run 

```
sh ./do.sh
```


## Install

Install [rust](https://rustup.rs) and then 

```
cargo install --git https://github.com/winding-lines/datacrumbs.git
```
