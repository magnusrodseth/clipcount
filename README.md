# `clipcount`: Counting words from the clipboard content

## Why does this exist?

Do you find yourself often needing to count the number of words in a piece of text? You could use a website, or the `wc` command-line tool, but why not use something **stupid simple**?

So I made this.

## How do I install it?

```sh
cargo install clipcount
```

## How do I use it?

1. ✂️ Copy some text to your clipboard

2. 🏃🏽 Run `clipcount` from the command line

3. ✅ You're done. What more did you expect?

## I miss `wc`

The application provides an optional flag to present the output in the same format as `wc`:

```sh
clipcount --wc
```

## License

Licensed under the [MIT license](https://github.com/dtolnay/syn/blob/HEAD/LICENSE-MIT).
