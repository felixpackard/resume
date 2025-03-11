# Web package

Before attempting to build this package directly, make sure to run the `copy-files` task from the Makefile so that resume.json is present in the packages/web directory.

To install dependencies:

```sh
bun install
```

To render to an HTML file:

```sh
bun run build
```

To watch and continuously render to an HTML file whenever the contents of resume.json changes:

```sh
bun run watch
```

The rendered resume will be output to [resume.html](./resume.html).
