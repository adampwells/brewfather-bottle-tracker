<div align="center">

  <h1><code>brewfather-bottle-tracker</code></h1>

This project is based on the [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template)
[**📚 Read the template tutorial! 📚**][template-docs]
</div>

I followed this [tutorial that combines wasm and Vue](https://blog.stackademic.com/webassembly-simplified-a-beginners-guide-to-creating-your-first-wasm-project-with-rust-458091fdde57).

## About
This project is a combination of a wasm library that 'hides' bottle ids inside the numbers we are allowed to edit in each Brewfather batch, and a basic Vue single page web app plus some local storage.

The function of the app is to associate QR coded bottle ids with a given batch, so tou can scan the bottle in your phone and get a description of the beer inside.

When you are re-using your bottles, just scan the bottle and associate it with a different batch!

You will need to create a Brewfather API key and enter it into the app. We don't record the credentials and they stay on your browser only.

## 🛠️ Build
The build command creates the wasm library and copies it to the web app.
```
wasm-pack build
```
You need to clean out the vite cache after building the wasm library, otherwise the web app will not see the changes.
```
rm -rf node_modules/.vite
pnpm install
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
