# markdown parser

parsing markdown to html

### usage

```js
import { parse, parseMarkdownFile } from "markdown-parser";
const markdown =
  "Hello world, this is a ~~complicated~~ *very simple* example.";
const html = parse(markdown);

const htmlFromFile = parseMarkdownFile(pathToMd);
```
