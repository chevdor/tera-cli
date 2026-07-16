# Install

## Via Cargo

```bash
cargo install --git https://github.com/chevdor/tera-cli
```

## Via Docker

A `tera` Docker image is available at `chevdor/tera` — very small, under
8MB.

```bash
docker run --rm -it chevdor/tera --version
```

This image isn't yet built by CI, so it may lag behind the very latest
version.

### Build the image yourself

```bash
docker build --tag tera-cli .
```

### Run from the container

```bash
# Help
docker run -it --rm tera-cli --help

# Parse a template
docker run -it --rm \
    --volume="$(pwd)/templates:/templates" \
    --read-only \
    --env=FOO=BAR \
    tera-cli --template /templates/env-debug.txt --env-only --env-key env
```

## Hot reload

Watch a folder of templates and re-run `tera` whenever one changes. Name
your template e.g. `foobar.md.tera` if it expands into a markdown file, then
use [fswatch](https://github.com/emcrisostomo/fswatch):

```bash
fswatch templates -e ".*\.md$" | \
    xargs -n1 -I{} \
    tera --include-path templates \
        --template templates/template.md.tera context.json
```
