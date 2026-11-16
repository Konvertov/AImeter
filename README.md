# AI Meter

AI usage limits next to your clock. Free, for Windows and macOS.

AI Meter finds the AI tools you are already signed in to on this computer and shows how much of each limit is spent and when it resets. Nothing to type in for tools that keep a sign-in locally; services that work by API key need the key once.

Working title; the name will change.

## Services

| Service | How it is found |
| --- | --- |
| Claude Code | Claude Code sign-in |
| Codex / ChatGPT | Codex CLI sign-in |
| Cursor | Cursor editor sign-in |
| GitHub Copilot | GitHub CLI sign-in |
| Antigravity | the running Antigravity app |
| Windsurf | Windsurf editor (last numbers it saved) |
| Grok | Grok CLI sign-in |
| Kiro | Kiro editor (last numbers it saved) |
| JetBrains AI | any JetBrains IDE with AI Assistant |
| Devin | Devin CLI or app sign-in |
| Amp | Amp CLI sign-in |
| OpenCode Go | OpenCode sign-in |
| Kimi Code | Kimi CLI sign-in |
| Codebuff | Codebuff CLI sign-in |
| OpenRouter | API key |
| DeepSeek | API key |
| abliteration.ai | API key |
| Gemini | sign in once in a window |

## Privacy

Everything stays on your computer. AI Meter talks only to the services themselves, has no server and collects no statistics. Sign-ins of other programs are read, never changed or copied. API keys are kept on your computer only: in the system password store on Windows, in a file only your user can read on macOS.

## Build

Needs Node 22+ and Rust stable.

```
npm install
npm run tauri build
```

`aimeter --json` prints what every service returned, for troubleshooting.
