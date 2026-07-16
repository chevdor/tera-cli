# ENV variables

Context data can be passed as a file or via **stdin** (JSON only for stdin).

## Enable ENV injection

By default, environment variables are not merged in. Turn this on with
`--env`.

## Collisions

Once ENV merging is enabled, your ENV may collide with your context data —
convenient if you want ENV to override the context data.

## ENV injection priority

Use `--env-first` to apply ENV first and let your context data override it
(the opposite of the default).

## Failing on collisions

Use `--fail-on-collision` to treat a collision as an error: exits with
status `1` and a message, instead of silently letting one side win.

## ENV only

Use `--env-only` to use solely the ENV as context data (skips the "no
context passed" error).

## ENV sub key

By default ENV variables load at the root of the context — e.g. `HOME`
becomes `{{ HOME }}` in the template. To avoid collisions, nest ENV under a
sub key with `--env-key <name>` — e.g. `--env-key env` makes `HOME`
available as `{{ env.HOME }}`.

More verbose, but combined with `--fail-on-collision` it guarantees nothing
happens behind your back.
