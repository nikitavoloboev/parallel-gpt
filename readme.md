# Parallel GPT

Read [main.rs](src-tauri/src/main.rs) for details.

## Run

```
pnpm i
pnpm run dev
```

## TODO

- tauri solid app that listens to a websocket of local server
  - maybe build on https://github.com/huntabyte/chatty
- you select text, prompt it with `explain ..` async. then get notification when you get reply.
- switch to it
- each answer is in a separate tab
- add modal to enter key. store it safely
- add settings with hotkeys to map to which model with which pre prompt
- combine it with sharegpt.dev
- take ideas from https://reflect.academy/artificial-intelligence
