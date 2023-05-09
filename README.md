## Usage

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

## Performance

It's about 10x slower than fast-json-patch. I imagine it's because copying values js <-> rust is slow. But I'm not sure.

![image](https://github.com/ankarhem/jsonpatch-rs/assets/14110063/3536b3b8-6413-4994-ac38-5aaa3d6927df)
