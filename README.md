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

## Why 94 bottles?
The [Brewfather API](https://docs.brewfather.app/api#update-batch) only allows some fields on a batch to be edited, and those are numbers (or fixed strings).

Those numbers are stored as 64 bit floats, and all but the `measuredMashPh` field are truncated at 6 decimal places.

For `measuredMashPh` we can use the least significant 42 bits without materially changing the pH - eg if the tru measured pH is 5.4, we can encode bottle ids and store 5.451171875000118.

Each of the 42 bits represents a bottle id, so we can assign any or all of bottles 1 through 42 to a given batch.

We use `fermenterTopUp` and completely re-write that to a really big number with only a couple of decimal places (to avoid getting trucnated), and that gives us access to 52 bits to mess with. This means that the `fermenterTopUp` value will now read some ridiculously large number that is meaningless in terms of brewing, so you will just have to ignore it. 

Since 42 + 52 = 94, that is the limit on how many bottle we can manage. If there are requests for more, I can perhaps add another field to completely nuke and bump up the capacity by another 52.

## Why Rust / wasm?
I wanted this app to be completely self-contained and require no back end server. JavaScript usues 64 bit floats for all of its numbers (yay!), but [bitwise operators in the language are in 32 bits](https://www.w3schools.com/js/js_bitwise.asp) (boo!).

Rust has excellent support for bitwise operations on 64 bit numbers, and targets wasm easily, so the bitwise stuff happens in wasm.