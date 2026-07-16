# tera-cli

![tera-cli logo](/media/logo.png ':size=128')

[tera](https://github.com/Keats/tera) is a template engine written in Rust
and inspired by Jinja2. It merges some data (called **context data**) into a
template and produces a new output. `tera-cli` is a command line for the
[tera template engine](https://github.com/Keats/tera).

This project is called `tera-cli` but the command installed on your system
is simply `tera`.

`tera-cli` offers powerful features related to your environment variables,
letting you control the output **both** from the context data you pass and
from the ENV variables set on your system.

## What can I do with this?

If you have **data** and want to format it, this tool is a great companion:

- Generate beautiful changelogs in markdown, asciidoc, restructured text, etc.
- Generate more human-readable views of your data
- Make a blog with it
- Generate k8s config files

The tera engine itself supports variables & expressions (including math),
comments, control structures & loops, filters, formatting functions (file
size, dates, ...), inheritance, includes, and built-ins (capitalize,
replace, trim, ...) — see the
[tera docs](https://tera.netlify.app/docs) for the full template syntax.

## Where to start

- [Install](/getting-started/install.md)
- [A basic example](/guide/example.md)
- [CLI reference](/reference/cli.md)
