# HMTL editor cli

> `cargo install html-editor-cli`

A command line tool to edit HTML files [written in Rust](https://github.com/mothepro/html-editor-cli).

| alias | --option     | Description |
|-------|-------------|------------------------------------|
| `-q` | `--query`     | Document query for the element to replace |
| `-a` | `--attribute` | The attribute to be given the new content <br /> If unset, the content will become the element's only child |
| `-c` | `--content`   | The content to use in the replacement     |
| `-h` | `--help`      | Print help                                |
| `-V` | `--version`   | Print version                             |

## Example

Imagine you have `sample.html` which you want to be modified during build step.

```html
<!DOCTYPE html>
<html>
<body>

  <h1>My First Heading</h1>
  <p>
    My first paragraph.
    <a id="home">go home</a>
  </p>
  <span class="build-time">some time</span>

</body>
</html>
```

This command will put a time in the `<span>`'s attribute

`html-editor-cli --query .build-time --content "Now" < sample.html > build.html`

On unix you can nest commands for good results

`html-editor-cli --query .build-time --content $(time) < sample.html > build.html`

Use the attribute option to modify HTML in some other ways

`html-editor-cli --query #home --attribute href --content "http://localhost/" < sample.html > build.html`
