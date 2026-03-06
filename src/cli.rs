use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use console::style;
use dialoguer::{Input, MultiSelect, Select, theme::ColorfulTheme};
use rust_i18n::t;

use crate::{
    config_path, load_config, DocConfig,
    DOC_REPO_REQUIRED, DOC_REPO_SUPPLEMENTARY, PROJECT_REPO_FILES,
};

// ---------------------------------------------------------------------------
// Agent definitions
// ---------------------------------------------------------------------------

struct AgentDef {
    name: &'static str,
    mcp_config: McpConfig,
    skill_dir: Option<&'static str>,
}

enum McpConfig {
    /// Standard JSON: { "<key>": { "alcove": { "command": "...", "env": {...} } } }
    Json {
        path: &'static str,
        server_key: &'static str,
    },
    /// OpenCode format: { "mcp": { "alcove": { "type": "local", ... } } }
    OpenCode {
        path: &'static str,
    },
    /// Codex TOML format
    Codex {
        path: &'static str,
    },
}

fn home() -> PathBuf {
    dirs::home_dir().expect("Cannot determine home directory")
}

fn agents() -> Vec<AgentDef> {
    vec![
        AgentDef {
            name: "Claude Code",
            mcp_config: McpConfig::Json { path: "~/.claude.json", server_key: "mcpServers" },
            skill_dir: Some("~/.claude/skills/alcove"),
        },
        AgentDef {
            name: "Cursor",
            mcp_config: McpConfig::Json { path: "~/.cursor/mcp.json", server_key: "mcpServers" },
            skill_dir: Some("~/.cursor/skills/alcove"),
        },
        AgentDef {
            name: "Claude Desktop",
            mcp_config: McpConfig::Json {
                path: if cfg!(target_os = "macos") {
                    "~/Library/Application Support/Claude/claude_desktop_config.json"
                } else {
                    "~/.config/claude/claude_desktop_config.json"
                },
                server_key: "mcpServers",
            },
            skill_dir: None,
        },
        AgentDef {
            name: "Cline (VS Code)",
            mcp_config: McpConfig::Json {
                path: if cfg!(target_os = "macos") {
                    "~/Library/Application Support/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json"
                } else {
                    "~/.config/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json"
                },
                server_key: "mcpServers",
            },
            skill_dir: None,
        },
        AgentDef {
            name: "OpenCode",
            mcp_config: McpConfig::OpenCode { path: "~/.config/opencode/opencode.json" },
            skill_dir: Some("~/.opencode/skills/alcove"),
        },
        AgentDef {
            name: "Codex CLI",
            mcp_config: McpConfig::Codex { path: "~/.codex/config.toml" },
            skill_dir: None,
        },
        AgentDef {
            name: "Antigravity",
            mcp_config: McpConfig::Json { path: "~/.antigravity/settings.json", server_key: "mcpServers" },
            skill_dir: None,
        },
        AgentDef {
            name: "Gemini CLI",
            mcp_config: McpConfig::Json { path: "~/.gemini/settings.json", server_key: "mcpServers" },
            skill_dir: Some("~/.gemini/skills/alcove"),
        },
    ]
}

fn expand_path(p: &str) -> PathBuf {
    if let Some(stripped) = p.strip_prefix("~/") {
        home().join(stripped)
    } else {
        PathBuf::from(p)
    }
}

// ---------------------------------------------------------------------------
// Resolve docs root
// ---------------------------------------------------------------------------

/// Resolve docs root for setup: always show prompt with current value as default,
/// allowing the user to confirm or change it.
fn resolve_docs_root_interactive() -> Result<PathBuf> {
    let current = saved_docs_root();
    prompt_docs_root(current.as_deref())
}

/// Return saved docs root from env or config.toml (no prompt).
fn saved_docs_root() -> Option<PathBuf> {
    if let Ok(v) = std::env::var("DOCS_ROOT") {
        let p = PathBuf::from(&v);
        if p.is_dir() {
            return Some(p);
        }
    }
    let cfg = load_config();
    if let Some(p) = cfg.docs_root()
        && p.is_dir() {
            return Some(p);
        }
    None
}

/// Interactive prompt for docs root. Shows `default` as pre-filled value if provided.
fn prompt_docs_root(default: Option<&Path>) -> Result<PathBuf> {
    let theme = ColorfulTheme::default();
    let prompt = t!("setup.docs_prompt");
    let input: String = match default {
        Some(d) => Input::with_theme(&theme)
            .with_prompt(prompt.as_ref())
            .default(d.to_string_lossy().into_owned())
            .interact_text()?,
        None => Input::with_theme(&theme)
            .with_prompt(prompt.as_ref())
            .interact_text()?,
    };

    let p = PathBuf::from(shellexpand(&input));
    if !p.is_dir() {
        anyhow::bail!("{}", t!("setup.invalid_path", path = p.display()));
    }

    save_docs_root(&p)?;
    Ok(p)
}

fn shellexpand(s: &str) -> String {
    if let Some(stripped) = s.strip_prefix("~/") {
        format!("{}/{}", home().display(), stripped)
    } else {
        s.to_string()
    }
}

fn save_docs_root(path: &Path) -> Result<()> {
    let cfg_path = config_path();
    fs::create_dir_all(cfg_path.parent().unwrap())?;

    if cfg_path.exists() {
        let content = fs::read_to_string(&cfg_path)?;
        if content.contains("docs_root") {
            // Update existing
            let updated: String = content
                .lines()
                .map(|l| {
                    if l.trim_start().starts_with("docs_root") {
                        format!("docs_root = \"{}\"", path.display())
                    } else {
                        l.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(&cfg_path, updated)?;
        } else {
            // Prepend
            let updated = format!("docs_root = \"{}\"\n\n{}", path.display(), content);
            fs::write(&cfg_path, updated)?;
        }
    } else {
        fs::write(&cfg_path, format!("docs_root = \"{}\"\n", path.display()))?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Binary path
// ---------------------------------------------------------------------------

fn binary_path() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("alcove"))
}

// ---------------------------------------------------------------------------
// Skill file
// ---------------------------------------------------------------------------

const SKILL_CONTENT: &str = include_str!("../skill/SKILL.md");

fn install_skill_to(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir)?;
    fs::write(dir.join("SKILL.md"), SKILL_CONTENT)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// MCP config writers
// ---------------------------------------------------------------------------

fn write_json_mcp(config_path: &Path, server_key: &str, binary: &Path, docs_root: &Path) -> Result<()> {
    let mut config: serde_json::Value = if config_path.exists() {
        let content = fs::read_to_string(config_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    let server_entry = serde_json::json!({
        "command": binary.to_string_lossy(),
        "args": [],
        "env": {
            "DOCS_ROOT": docs_root.to_string_lossy()
        }
    });

    config[server_key]["alcove"] = server_entry;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(config_path, serde_json::to_string_pretty(&config)?)?;
    Ok(())
}

fn write_opencode_mcp(config_path: &Path, binary: &Path, docs_root: &Path) -> Result<()> {
    let mut config: serde_json::Value = if config_path.exists() {
        let content = fs::read_to_string(config_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    config["mcp"]["alcove"] = serde_json::json!({
        "type": "local",
        "command": [binary.to_string_lossy()],
        "environment": {
            "DOCS_ROOT": docs_root.to_string_lossy()
        }
    });

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(config_path, serde_json::to_string_pretty(&config)?)?;
    Ok(())
}

fn write_codex_mcp(config_path: &Path, binary: &Path, docs_root: &Path) -> Result<()> {
    let entry = format!(
        "\n[mcp_servers.alcove]\ncommand = \"{}\"\nargs = []\n\n[mcp_servers.alcove.env]\nDOCS_ROOT = \"{}\"\n",
        binary.display(),
        docs_root.display(),
    );

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if config_path.exists() {
        let content = fs::read_to_string(config_path)?;
        if content.contains("[mcp_servers.alcove]") {
            // Already configured
            return Ok(());
        }
        fs::write(config_path, format!("{content}{entry}"))?;
    } else {
        fs::write(config_path, entry)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Agent selection UI
// ---------------------------------------------------------------------------

fn select_agents(prompt: &str) -> Result<Vec<usize>> {
    let agent_list = agents();
    let names: Vec<&str> = agent_list.iter().map(|a| a.name).collect();

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&names)
        .defaults(&vec![false; names.len()])
        .interact()?;

    Ok(selected)
}

// ---------------------------------------------------------------------------
// Diagram format selection
// ---------------------------------------------------------------------------

const DIAGRAM_FORMATS: &[(&str, &str)] = &[
    ("mermaid", "Mermaid — GitHub/GitLab native, most popular"),
    ("plantuml", "PlantUML — Enterprise UML, richest diagram types"),
    ("d2", "D2 — Modern, clean rendering, Go-based"),
    ("ascii", "ASCII art — Universal, no renderer needed"),
    ("graphviz", "Graphviz (DOT) — Classic graph visualization"),
    ("structurizr", "Structurizr (C4) — Architecture-focused C4 model"),
    ("excalidraw", "Excalidraw — Hand-drawn style, brainstorming"),
];

// ---------------------------------------------------------------------------
// Document category selection
// ---------------------------------------------------------------------------

struct CategoryDef {
    label: &'static str,
    defaults: &'static [&'static str],
}

const CATEGORIES: &[CategoryDef] = &[
    CategoryDef {
        label: "Core (private project docs)",
        defaults: DOC_REPO_REQUIRED,
    },
    CategoryDef {
        label: "Team (internal extras)",
        defaults: DOC_REPO_SUPPLEMENTARY,
    },
    CategoryDef {
        label: "Public (repo-facing docs)",
        defaults: PROJECT_REPO_FILES,
    },
];

/// Returns (core_files, team_files, public_files) after interactive selection.
/// Pre-checks items based on existing config or defaults.
fn select_categories() -> Result<(Vec<String>, Vec<String>, Vec<String>)> {
    let cfg = load_fresh_config();
    let existing: [Vec<String>; 3] = [
        cfg.as_ref().map_or_else(
            || DOC_REPO_REQUIRED.iter().map(|s| s.to_string()).collect(),
            |c| c.core_files(),
        ),
        cfg.as_ref().map_or_else(
            || DOC_REPO_SUPPLEMENTARY.iter().map(|s| s.to_string()).collect(),
            |c| c.team_files(),
        ),
        cfg.as_ref().map_or_else(
            || PROJECT_REPO_FILES.iter().map(|s| s.to_string()).collect(),
            |c| c.public_files(),
        ),
    ];

    let theme = ColorfulTheme::default();
    let mut results: Vec<Vec<String>> = Vec::new();

    for (i, cat) in CATEGORIES.iter().enumerate() {
        let items: Vec<&str> = cat.defaults.to_vec();
        let defaults: Vec<bool> = items
            .iter()
            .map(|item| existing[i].iter().any(|e| e == *item))
            .collect();

        let selected = MultiSelect::with_theme(&theme)
            .with_prompt(cat.label)
            .items(&items)
            .defaults(&defaults)
            .interact()?;

        let files: Vec<String> = selected.iter().map(|&idx| items[idx].to_string()).collect();
        println!(
            "  {} {}",
            style("✓").green(),
            t!("setup.category_status", label = cat.label, selected = files.len(), total = items.len())
        );
        results.push(files);
    }

    Ok((
        results.remove(0),
        results.remove(0),
        results.remove(0),
    ))
}

/// Load config fresh from disk (bypasses OnceLock cache).
fn load_fresh_config() -> Option<DocConfig> {
    let path = config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).ok()?;
        toml::from_str::<DocConfig>(&content).ok()
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

pub fn cmd_setup() -> Result<()> {
    println!();
    println!("{}", style("══════════════════════════════════════").bold());
    println!("  {}", style(t!("setup.title")).bold());
    println!("{}", style("══════════════════════════════════════").bold());

    // 1. Docs root
    println!();
    println!("{}", style(format!("── {} ──", t!("setup.docs_repo"))).bold());
    let docs_root = resolve_docs_root_interactive()?;
    println!("  {} {}", style("✓").green(), t!("setup.docs_root_set", path = docs_root.display()));

    // 2. Document categories
    println!();
    println!("{}", style(format!("── {} ──", t!("setup.categories"))).bold());
    let (core_files, team_files, public_files) = select_categories()?;

    // 3. Diagram format
    println!();
    println!("{}", style(format!("── {} ──", t!("setup.diagram"))).bold());
    let existing_diagram = load_fresh_config()
        .map(|c| c.diagram_format())
        .unwrap_or_default();
    let diagram_default = DIAGRAM_FORMATS
        .iter()
        .position(|(k, _)| *k == existing_diagram)
        .unwrap_or(0);
    let format_labels: Vec<&str> = DIAGRAM_FORMATS.iter().map(|(_, l)| *l).collect();
    let diagram_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(t!("setup.diagram_prompt").as_ref())
        .items(&format_labels)
        .default(diagram_default)
        .interact()?;
    let diagram_format = DIAGRAM_FORMATS[diagram_idx].0;
    println!("  {} {}", style("✓").green(), t!("setup.diagram_set", format = diagram_format));

    // 4. Save config
    save_full_config(&docs_root, diagram_format, &core_files, &team_files, &public_files)?;

    // 4. Agent setup
    println!();
    println!("{}", style(format!("── {} ──", t!("setup.agents"))).bold());
    let selected = select_agents(&t!("setup.agents_prompt"))?;
    let agent_list = agents();
    let bin = binary_path();

    for idx in &selected {
        let agent = &agent_list[*idx];
        println!();
        println!("  {}", style(agent.name).cyan());

        // MCP
        match &agent.mcp_config {
            McpConfig::Json { path, server_key } => {
                let p = expand_path(path);
                write_json_mcp(&p, server_key, &bin, &docs_root)?;
                println!("  {} {}", style("✓").green(), t!("setup.mcp_set", path = path));
            }
            McpConfig::OpenCode { path } => {
                let p = expand_path(path);
                write_opencode_mcp(&p, &bin, &docs_root)?;
                println!("  {} {}", style("✓").green(), t!("setup.mcp_set", path = path));
            }
            McpConfig::Codex { path } => {
                let p = expand_path(path);
                write_codex_mcp(&p, &bin, &docs_root)?;
                println!("  {} {}", style("✓").green(), t!("setup.mcp_set", path = path));
            }
        }

        // Skill
        if let Some(skill_path) = agent.skill_dir {
            let p = expand_path(skill_path);
            install_skill_to(&p)?;
            println!("  {} {}", style("✓").green(), t!("setup.skill_set", path = skill_path));
        }
    }

    // 5. Summary
    println!();
    println!("{}", style(format!("── {} ──", t!("setup.done"))).bold());
    println!("  {}", t!("setup.binary", path = binary_path().display()));
    println!("  {}", t!("setup.config", path = config_path().display()));
    println!("  {}", t!("setup.docs", path = docs_root.display()));
    println!();
    println!("  {}", style(t!("setup.hint_update").to_string()).dim());
    println!("  {}", style(t!("setup.hint_uninstall").to_string()).dim());
    println!();

    Ok(())
}

pub fn cmd_uninstall() -> Result<()> {
    println!();
    println!("{}", style(t!("uninstall.title").to_string()).bold());
    println!();

    // Skills
    let skill_dirs = [
        "~/.claude/skills/alcove",
        "~/.cursor/skills/alcove",
        "~/.opencode/skills/alcove",
        "~/.gemini/skills/alcove",
    ];
    for d in &skill_dirs {
        let p = expand_path(d);
        if p.exists() {
            fs::remove_dir_all(&p)?;
            println!("  {} {}", style("✓").green(), t!("uninstall.removed_skill", path = d));
        }
    }

    // Config
    let cfg = config_path();
    if cfg.exists() {
        fs::remove_file(&cfg)?;
        println!("  {} {}", style("✓").green(), t!("uninstall.removed_config", path = cfg.display()));
    }
    // Legacy config
    let legacy = cfg.with_file_name("config");
    if legacy.exists() {
        fs::remove_file(&legacy)?;
        println!("  {} {}", style("✓").green(), t!("uninstall.removed_legacy", path = legacy.display()));
    }

    println!();
    println!("  {}", style(t!("uninstall.binary_hint").to_string()).yellow());
    println!();
    println!("  {}", t!("uninstall.mcp_hint"));
    println!("    Claude Code:    ~/.claude.json");
    println!("    Cursor:         ~/.cursor/mcp.json");
    println!("    Claude Desktop: ~/Library/Application Support/Claude/claude_desktop_config.json");
    println!("    OpenCode:       ~/.config/opencode/opencode.json");
    println!("    Codex:          ~/.codex/config.toml");
    println!("    Antigravity:    ~/.antigravity/settings.json");
    println!("    Gemini CLI:     ~/.gemini/settings.json");
    println!();

    Ok(())
}

// ---------------------------------------------------------------------------
// Save config.toml
// ---------------------------------------------------------------------------

/// Save full config with all categories (used by setup).
fn save_full_config(
    docs_root: &Path,
    diagram_format: &str,
    core_files: &[String],
    team_files: &[String],
    public_files: &[String],
) -> Result<()> {
    let cfg_path = config_path();
    fs::create_dir_all(cfg_path.parent().unwrap())?;

    let fmt_list = |files: &[String]| -> String {
        files.iter().map(|f| format!("\"{}\"", f)).collect::<Vec<_>>().join(", ")
    };

    let content = format!(
        "docs_root = \"{}\"\n\n[core]\nfiles = [{}]\n\n[team]\nfiles = [{}]\n\n[public]\nfiles = [{}]\n\n[diagram]\nformat = \"{}\"\n",
        docs_root.display(),
        fmt_list(core_files),
        fmt_list(team_files),
        fmt_list(public_files),
        diagram_format,
    );
    fs::write(&cfg_path, content)?;

    println!("  {} {}", style("✓").green(), t!("setup.config_saved", path = cfg_path.display()));
    Ok(())
}
