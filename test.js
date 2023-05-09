#!/usr/bin/env node
const wasm = require('./build/node/jsonpatch_rs')


function testing() {
  let orig = {
    name: "hello",
    items: ["apple", "cherry"]
  };
  let dest = {
    name: "hello2",
    items: ["apple", "banana"]
  };

  let patch = wasm.diff(orig, dest);

  console.log(patch);
}

testing();
