# General BASH Notes
## Common Code
### Run BASH script from anywhere
```sh
#!/usr/bin/env bash

# Obtains the absolute path to the parent directory of this script
PROJECT_BASE_DIR="$(builtin cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

set -e # enables "exit on error" mode
function finish {
  popd
}

pushd "${PROJECT_BASE_DIR}"
trap finish EXIT # Runs function finish when EXIT signal received
```

## Syntax
- `$(...)`: This is a command substitution. The command inside the parentheses is executed, and its output is substituted in place

## Commands
- `set -e`: enables "exit on error" mode.  If any command in the script exits with a non-zero status, the script itself will also exit immediately
- `pushd` and `popd`:
  - "pushd" : pushes current dir onto stack and switches to new directory 
  - "popd" : returns to the directory at the top of the stack and removes it from the stack (no params)
  - Ex:
  ```sh
  pushd "${PROJECT_BASE_DIR}"
  trap popd EXIT # Runs function finish when EXIT signal received
  ```
- `trap`: sets up signal handling
