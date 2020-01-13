Datacrumbs is command line utility and conventions to help you track data provenance. The main use
case is when repeatedly running simple commands that generate data. For  more complex flow tracking
you should evaluate a system like [mlflow](https://mlflow.org/)


## Overview

The use case where `datacrumbs` is most useful is when the output of your command is a new folder
during every command execution. The command is providing a very simple chaining capability by creating a `datacrumbs.json` file with the following
properties:

- `run_time`: UTC time when the command was executed
- `cwd`: the work folder where the command was executed
- `git_hash`: current git hash


## Example

Let's assume you have an existing command `generate.sh` with the following usage pattern:

```
generate.sh --input /data/some --use_foo --output /data/out/
```

To integrate `datacrumbs` you defer the output generation to it

```
generate.sh --input /data/some --use_foo --output `datacrumbs /data/out`
```

