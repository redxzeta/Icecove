Output demo-agent.gif

Set Shell "bash"
Set FontSize 14
Set Width 900
Set Height 520
Set Theme "Dracula"
Set Padding 24
Set TypingSpeed 50ms
Set PlaybackSpeed 1

# ── Hidden setup ──────────────────────────────────────────────────────────────
Hide

Type "export DOCS_ROOT=$(mktemp -d)"
Enter
Sleep 200ms

Type "export PATH=$(pwd)/demo/bin:$PATH"
Enter
Sleep 100ms

Type "cp -r demo/docs/my-app $DOCS_ROOT/ && cp -r demo/docs/my-api $DOCS_ROOT/"
Enter
Sleep 200ms

Type "mkdir -p $DOCS_ROOT/.alcove && cp demo/docs/.alcove/policy.toml $DOCS_ROOT/.alcove/"
Enter
Sleep 100ms

Type "alcove index 2>/dev/null"
Enter
Sleep 4000ms

Type "mkdir -p /tmp/projects/my-app /tmp/projects/my-api"
Enter
Sleep 100ms

Type "export PS1='\[\033[34m\]~/projects/\[\033[1;32m\]\W\[\033[0m\] \$ '"
Enter
Sleep 100ms

Show

# ── Scene 1: Claude Code · my-app · project search ───────────────────────────

Type "cd /tmp/projects/my-app"
Enter
Sleep 700ms

Type 'claude "how is authentication implemented?"'
Enter
Sleep 3500ms

Sleep 2200ms

# ── Scene 2: Gemini CLI · my-api · zero reconfiguration ──────────────────────

Type "cd /tmp/projects/my-api"
Enter
Sleep 700ms

Type 'gemini "what auth method does this service use?"'
Enter
Sleep 3500ms

Sleep 2000ms

# ── Scene 3: Codex · global search across all projects ───────────────────────

Ctrl+L
Sleep 400ms

Type 'codex "compare auth approaches across my projects"'
Enter
Sleep 3500ms

Sleep 2200ms

# ── Scene 4: Claude · validate + generate missing docs ───────────────────────

Ctrl+L
Sleep 400ms

Type "cd /tmp/projects/my-app"
Enter
Sleep 600ms

Type 'claude "audit our docs and create any missing required files"'
Enter
Sleep 5000ms

Sleep 3000ms
