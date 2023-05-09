```js
let orig = {
  name: "hello",
  items: ["apple", "cherry"]
};
let dest = {
  name: "hello2",
  items: ["apple", "banana"]
};

diff(orig, dest)
//[
//  { op: 'replace', path: '/items/1', value: 'banana' },
//  { op: 'replace', path: '/name', value: 'hello2' }
//]
```
