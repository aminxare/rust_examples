import test from "ava";
import { mkdirSync, rmdirSync, writeFileSync } from "node:fs";

import { parse, parseMarkdownFile } from "../index.js";

test("parse function", (t) => {
  const markdown =
    "Hello world, this is a ~~complicated~~ *very simple* example.";
  const expected_html =
    "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";

  const actualHtml = parse(markdown);
  t.is(actualHtml, expected_html);
});

test("parseMarkdownFile", (t) => {
  const tmpDir = "/tmp/markdown_parser_test";
  const tmpFile = tmpDir.concat("/test.md");
  const markdown =
    "Hello world, this is a ~~complicated~~ *very simple* example.";
  const expected_html =
    "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";

  mkdirSync(tmpDir);
  writeFileSync(tmpFile, markdown);

  const actualHtml = parseMarkdownFile(tmpFile);
  t.is(actualHtml, expected_html);

  rmdirSync(tmpDir, { recursive: true });
});
