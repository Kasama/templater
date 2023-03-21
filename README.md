Templater
=========

A simple program that takes advantage of the Handlebars rust package to template a file using a json as the context.

```
Usage: templater <TEMPLATE> <SOURCE>
```

So if you have a template file like:

```yaml
hello: {{hello}}
world: {{world}}
```

and a source file like
```json
{
  "hello": "Thing",
  "world": "my world"
}
```

Running the templater would yield
```sh
$ templater template.yaml values.json
hello: Thing
world: my world
```
