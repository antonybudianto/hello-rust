# hello-rust

## Setup

### VSCode

-   Install official [rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

-   Add to your `settings.json`:

```json
"[rust]": {
  "editor.defaultFormatter": "rust-lang.rust-analyzer",
  "editor.formatOnSave": true
}
```

## Development

```sh
make build
make run
```

## Build prod

```sh
make build-prod
make run-prod
```

## Vercel Rust

https://github.com/vercel-community/rust
