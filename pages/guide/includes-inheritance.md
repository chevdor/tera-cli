# Includes &amp; inheritance

## External files

With `--include`, the command recursively scans for files that can be used
as [includes](https://tera.netlify.app/docs/#include),
[macros](https://tera.netlify.app/docs/#macros), or for
[inheritance](https://tera.netlify.app/docs/#inheritance). By default it
scans the folder containing the main template, unless `--include-path` is
given.

Hidden files/directories (name starting with `.`, e.g. `.DS_Store`,
`.git`) are skipped during that scan. Pass `--include-hidden` to parse them
too.

From this repository, try the **include** feature:

```bash
USER="[YOURNAME]" tera --template data/include/hello.txt --include --env-only
```

and the **inheritance** feature:

```bash
USER="[YOURNAME]" tera --template data/inheritance/child.txt --inherit --env-only
```

(`--inherit` / `--inherit-path` are aliases for `--include` / `--include-path`.)

## Content escaping

Pass `-a` / `--escape` to auto-escape rendered content — useful for HTML
output.
