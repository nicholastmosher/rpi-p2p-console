# Rpi P2P Console

This is a toy project for controlling an LED strip at home
using a Rust/Yew app and hopefully P2P connection.

## Dependencies

You'll need the following tools:

- Node/npm/yarn
- Trunk

## Running

```
$ cd crates/rpi-yewi
```

TailwindCSS watch helper:

```
npx tailwindcss -c tailwind.config.js -o tailwind.css --watch
```

Yew app:

```
$ cd crates/rpi-yewi
$ trunk serve
```

Then go to http://localhost:8080
