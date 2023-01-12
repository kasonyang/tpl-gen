# Tpl-gen

A command tool for generating file from template.

# Usage

```
tpl-gen [-d DATA_FILE] [-o OUT_FILE] [-v:VAR_NAME VAR_VALUE]... TEMPLATE_FILE
```

For example:

With template file:`template.tpl`
```
Hello, {{name}}
```
Run
```
tpl-gen -v:name world -o output.txt template.tpl
```
A `output.txt` that contains the following content will be generated.
```
Hello, world
```

For more complex variables, you could save your data to a data file.

For example, with the data file `data.toml`

```
name = "world"
```
Run

```
tpl-gen -d data.toml -o output.txt template.tpl
```


# Template syntax

[Jinja2](http://jinja.pocoo.org/)

# Data syntax

[Toml](https://toml.io/)

# Licence

MIT