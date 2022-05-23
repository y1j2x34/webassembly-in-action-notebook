const rust = import('./pkg/hello.js');

rust
    .then(m => m.add(100, 100))
    .then(result => console.log(result));
