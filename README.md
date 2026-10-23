# soul-sacrifice-claude-code-hooks

My little toolkit for helping Claude Code and Obsidian work together.

## installation

available on homebrew tap, for macOS on Apple silicon.

```sh
brew tap ohataken/soul-sacrifice-claude-code-hooks https://github.com/ohataken/soul-sacrifice-claude-code-hooks
brew trust --formula ohataken/soul-sacrifice-claude-code-hooks/soul-sacrifice-claude-code-hooks
brew install soul-sacrifice-claude-code-hooks
```

## hooks

### SessionStart

```json
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "SOUL_SACRIFICE_SESSIONS_DIR=/path/to/vault/sessions soul-sacrifice-claude-code-hooks session_start '%Y-%m-%dT%H%M%S'"
          }
        ]
      }
    ]
  }
}
```

### UserPromptSubmit

```json
{
  "hooks": {
    "UserPromptSubmit": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "SOUL_SACRIFICE_SESSIONS_DIR=/path/to/vault/sessions soul-sacrifice-claude-code-hooks user_prompt_submit '%Y-%m-%dT%H%M%S'"
          }
        ]
      }
    ]
  }
}
```
