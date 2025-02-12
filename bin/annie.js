#!/usr/bin/env node

const { run } = require("../index.js");
const path = require("path");
const [_bin, script, ...args] = process.argv;

try {
  run([path.parse(script).name, ...args]);
} catch (e) {
  console.error(e);
  process.exit(1);
}
