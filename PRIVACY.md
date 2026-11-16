# AI Meter — privacy policy

AI Meter has no server of its own and collects no statistics. It does not
send anything about you, your computer or your use of AI tools to its author
or to anyone else.

## What AI Meter connects to, and why

AI Meter only connects to the services you use yourself, to ask them for
your own limits, and to GitHub for updates:

| Connection | When | What is sent |
| --- | --- | --- |
| The AI service whose limits are shown (for example `api.anthropic.com` for Claude Code, `chatgpt.com` for Codex, `api2.cursor.sh` for Cursor, `api.github.com` for GitHub Copilot, `openrouter.ai`, `api.deepseek.com`) | Every few minutes and when you press Refresh, only for tools found signed in on this computer or that you connected yourself | The sign-in or API key that tool or you already gave it, exactly as the tool itself sends it. The answer (your limits) is shown in the panel and kept only in memory. |
| `gemini.google.com` | Only after you choose to sign in to Gemini inside AI Meter | Your Gemini web session, to read your limits |
| `github.com` (releases of Konvertov/ai-meter) | About once every few hours | A request for the newest version number; new versions are downloaded from there |

Nothing else is contacted. There is no analytics, no crash reporting and no
advertising.

## What stays on your computer

- **Sign-ins of other programs** (Claude Code, Codex, Cursor and others) are
  only read. AI Meter never changes, renews, copies or uploads them anywhere
  except to the service they belong to.
- **API keys and the Gemini session** you add are kept on this computer: in
  the Windows Credential Manager on Windows, in a file only your user can
  read on macOS.
- **Token statistics** for Claude Code are counted from Claude Code's own log
  files on this computer and are never sent anywhere.
- **Settings** (language, theme, hidden tools) are stored locally.

## Third-party services

Each AI service you use has its own privacy policy, which applies to the
requests AI Meter makes on your behalf, the same as to the tool itself.

## Removing AI Meter

- **Windows:** Settings → Apps → AI Meter → Uninstall.
- **macOS:** quit AI Meter from its panel, then move AI Meter from
  Applications to the Trash. Its saved keys are in
  `~/Library/Application Support/AImeter`.

Questions: [t.me/konvertov](https://t.me/konvertov).
