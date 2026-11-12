# AImeter

AI usage limits next to your clock. Free, for Windows and macOS.

AImeter finds the AI tools you are already signed in to on this computer and shows how much of each limit is spent and when it resets. Nothing to type in for tools that keep a sign-in locally; services that work by API key need the key once.

Working title; the name will change.

## Services

| Service | How it is found |
| --- | --- |
| Claude | Claude Code sign-in |
| Codex / ChatGPT | Codex CLI sign-in |
| Cursor | Cursor editor sign-in |
| GitHub Copilot | GitHub CLI sign-in |
| OpenRouter | API key |
| DeepSeek | API key |

## Privacy

Everything stays on your computer. AImeter talks only to the services themselves, has no server and collects no statistics. Sign-ins of other programs are read, never changed or copied. API keys are kept in the system password store.

## Build

Needs Node 22+ and Rust stable.

```
npm install
npm run tauri build
```

`aimeter --json` prints what every service returned, for troubleshooting.
