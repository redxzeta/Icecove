Output demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 900
Set Height 560
Set Theme "Dracula"
Set Padding 24
Set TypingSpeed 45ms
Set PlaybackSpeed 1

# ── Hidden setup ──────────────────────────────────────────────────────────────
Hide

Type "export DOCS_ROOT=$(mktemp -d)"
Enter
Sleep 200ms

Type "export PS1='\[\033[1;32m\]\W\[\033[0m\] $ '"
Enter
Sleep 100ms

Type "mkdir -p $DOCS_ROOT/.alcove"
Enter
Sleep 100ms

Type "cp -r demo/docs/my-app $DOCS_ROOT/ && cp -r demo/docs/my-api $DOCS_ROOT/"
Enter
Sleep 200ms

Type "cp demo/docs/.alcove/policy.toml $DOCS_ROOT/.alcove/"
Enter
Sleep 100ms

Type "alcove index 2>/dev/null"
Enter
Sleep 4000ms

Type "mkdir -p /tmp/demo/my-app /tmp/demo/my-api"
Enter
Sleep 200ms

Show

# ── Scene 1: Project-scoped ranked search (my-app) ───────────────────────────

Type "cd /tmp/demo/my-app"
Enter
Sleep 700ms

Type 'alcove search "authentication"'
Enter
Sleep 3000ms

Sleep 2200ms

# ── Scene 2: Switch project — zero reconfiguration (my-api) ──────────────────

Type "cd /tmp/demo/my-api"
Enter
Sleep 700ms

Type 'alcove search "authentication"'
Enter
Sleep 3000ms

Sleep 1800ms

# ── Scene 3: Global search across all projects ────────────────────────────────

Ctrl+L
Sleep 400ms

Type 'alcove search "rotation" --scope global'
Enter
Sleep 3000ms

Sleep 2500ms

# ── Scene 4: Validate docs against team policy ────────────────────────────────

Ctrl+L
Sleep 400ms

Type "cd /tmp/demo/my-app"
Enter
Sleep 500ms

Type "alcove validate"
Enter
Sleep 2500ms

Sleep 3000ms
