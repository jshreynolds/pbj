## 0.3.0 (2025-11-23)

### Feat

- **templates/python.toml**: change project tool to 'uv'

### Fix

- fixed name for default config contents lookup
- **metadat**: change url scheme to http to support temporary redirect to github of main project domain
- **README.adoc**: add generated markdown version of documentation for crates.io

### Refactor

- **README.adoc**: Add space to force bump
- shifting responsibility to config, refactoring for clarity
- add slightly clearer and better string handling for defaults and the like.  move logic for figuring out settings
- add modules and rename for clarity

## 0.2.0 (2024-05-08)

### Feat

- add significant documentation. add `generate` subcommand in preparation for template generation. be tter messaging
- initial working implementation

### Fix

- update Cargo.toml with required data for publishing
- fixed testfile name. pytest won't run on a test file named 'test.py'

### Refactor

- clean up builtins with better written leet code
- ignore vscode file and add an explore directory for non-working experimentation
