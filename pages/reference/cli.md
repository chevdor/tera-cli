# CLI reference

```
Usage: tera [OPTIONS] --template <TEMPLATE> [CONTEXT]
```

## Arguments

| Argument | Description |
|---|---|
| `[CONTEXT]` | Location of the context data. Supports `json`, `toml`, `yaml`. Use `--stdin` to pass it via stdin instead |

## Options

| Flag | Description |
|---|---|
| `-t, --template <TEMPLATE>` | Location of the template |
| `-i, --include` (alias `--inherit`) | Parse all templates found in the same path as the given template |
| `--include-path <INCLUDE_PATH>` (alias `--inherit-path`) | Search/parse templates from a different path |
| `--include-hidden` | Also parse hidden files/directories (name starting with `.`) found in the include path |
| `-s, --stdin` | Pass context data via stdin |
| `-e, --env` | Append the current ENV to the context data under `--env-key` |
| `--env-key <ENV_KEY>` | Nest ENV variables under this key instead of the context root |
| `--env-first` | Apply ENV to the context before the passed data (so your data overrides ENV) |
| `--fail-on-collision` | Exit `1` on an ENV/context key collision instead of silently resolving it |
| `--env-only` | Use only the ENV as context (no context error) |
| `-o, --out <OUT>` | Write output to this file (default: stdout) |
| `-a, --escape` | Auto-escape rendered content (useful for HTML output) |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

See [ENV variables](/guide/env-variables.md) and
[Includes &amp; inheritance](/guide/includes-inheritance.md) for details and
examples of these flags in use.
