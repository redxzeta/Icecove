<p align="center">
  <img src="../alcove.png" alt="Alcove" width="100%" />
</p>

<p align="center">프로젝트 문서를 위한 조용한 공간.</p>

<p align="center">
  <a href="../README.md">English</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.es.md">Español</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.pt-BR.md">Português</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.ru.md">Русский</a>
</p>

<p align="center">
  <a href="https://crates.io/crates/alcove"><img src="https://img.shields.io/crates/v/alcove.svg" alt="crates.io" /></a>
  <a href="https://crates.io/crates/alcove"><img src="https://img.shields.io/crates/d/alcove.svg" alt="Downloads" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License" /></a>
  <a href="https://buymeacoffee.com/epicsaga"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-FFDD00?style=flat&logo=buy-me-a-coffee&logoColor=black" alt="Buy Me a Coffee" /></a>
</p>

Alcove는 AI 코딩 에이전트에게 프라이빗 프로젝트 문서에 대한 범위 지정된 읽기 전용 접근 권한을 제공하는 MCP 서버입니다 — 공개 저장소에 문서가 유출되지 않습니다.

## 문제

PRD, 아키텍처 결정, 배포 런북, 시크릿 맵 등 GitHub 저장소에 올려서는 안 되는 내부 문서가 있습니다. 하지만 AI 에이전트가 이 문서를 읽지 못하면 도움을 줄 수 없습니다.

Alcove는 프라이빗 문서와 AI 에이전트 사이에 위치합니다. 터미널의 현재 작업 디렉토리(CWD)에서 작업 중인 프로젝트를 자동 감지하고, 해당 프로젝트의 문서만 MCP 프로토콜을 통해 제공합니다.

```
~/projects/my-app $ claude "인증은 어떻게 구현되어 있나요?"

  → Alcove가 프로젝트 감지: my-app
  → ~/documents/my-app/ARCHITECTURE.md 읽기
  → 에이전트가 실제 프로젝트 컨텍스트로 답변
```

## 주요 기능

- **프로젝트 자동 감지** — CWD 기반, 프로젝트별 설정 불필요
- **범위 지정 접근** — 각 프로젝트는 자신의 문서만 볼 수 있음
- **프라이버시 설계** — 문서는 로컬 문서 저장소에 보관, 외부 노출 없음
- **크로스 레포 감사** — GitHub에 실수로 푸시된 내부 문서를 찾아 수정 제안
- **8개 이상 에이전트 지원** — Claude Code, Cursor, Claude Desktop, Cline, OpenCode, Codex, Antigravity, Gemini CLI

## 빠른 시작

```bash
cargo install alcove
alcove setup
```

이것만 하면 됩니다. `setup`이 대화형으로 모든 것을 안내합니다:

1. 문서가 어디에 있는지
2. 어떤 문서 카테고리를 추적할지
3. 선호하는 다이어그램 형식
4. 어떤 AI 에이전트를 설정할지 (MCP + 스킬 파일)

설정을 변경하려면 언제든 `alcove setup`을 다시 실행하세요. 이전 선택을 기억합니다.

## 소스에서 설치

```bash
git clone https://github.com/epicsagas/alcove.git
cd alcove
make install
```

## 작동 방식

```mermaid
flowchart LR
    subgraph Projects["프로젝트"]
        A1["my-app/\n  src/ ..."]
        A2["my-api/\n  src/ ..."]
    end

    subgraph Docs["프라이빗 문서"]
        D1["my-app/\n  PRD.md\n  ARCH.md"]
        D2["my-api/\n  PRD.md\n  ..."]
    end

    subgraph MCP["Alcove MCP 서버"]
        T1(overview)
        T2(search)
        T3(get_file)
        T4(audit)
        T5(init)
        T6(list)
    end

    A1 -- "CWD 감지" --> D1
    A2 -- "CWD 감지" --> D2
    MCP -- "읽기" --> D1
    MCP -- "읽기" --> D2
```

문서는 별도 디렉토리(`DOCS_ROOT`)에 정리됩니다. Alcove는 거기서 읽어 MCP의 stdio 프로토콜을 통해 AI 에이전트에게 제공합니다. 에이전트는 `get_doc_file("PRD.md")` 같은 도구를 호출하여 프로젝트별 답변을 얻습니다.

## 문서 분류

Alcove는 문서를 세 단계로 분류합니다:

| 분류 | 위치 | 예시 |
|------|------|------|
| **doc-repo-required** | Alcove (프라이빗) | PRD, Architecture, Decisions, Conventions |
| **doc-repo-supplementary** | Alcove (프라이빗) | Deployment, Onboarding, Testing, Runbook |
| **project-repo** | GitHub 저장소 (공개) | README, CHANGELOG, CONTRIBUTING |

`audit` 도구는 양쪽 위치를 확인하고 조치를 제안합니다 — 프라이빗 PRD에서 공개 README를 생성하거나, 잘못 배치된 리포트를 alcove로 가져오는 등.

## MCP 도구

| 도구 | 기능 |
|------|------|
| `get_project_docs_overview` | 분류 및 크기와 함께 모든 문서 목록 |
| `search_project_docs` | 모든 프로젝트 문서에서 키워드 검색 |
| `get_doc_file` | 경로로 특정 문서 읽기 |
| `list_projects` | 문서 저장소의 모든 프로젝트 표시 |
| `audit_project` | 크로스 레포 감사 및 조치 제안 |
| `init_project` | 템플릿에서 새 프로젝트 문서 스캐폴드 |

## CLI

```
alcove              MCP 서버 시작 (에이전트가 호출)
alcove setup        대화형 설정 — 언제든 다시 실행하여 재설정
alcove uninstall    스킬, 설정 및 레거시 파일 제거
```

## 설정

설정 파일 위치: `~/.config/alcove/config.toml`:

```toml
docs_root = "/Users/you/documents"

[core]
files = ["PRD.md", "ARCHITECTURE.md", "PROGRESS.md", "DECISIONS.md", "CONVENTIONS.md", "SECRETS_MAP.md", "DEBT.md"]

[team]
files = ["ENV_SETUP.md", "ONBOARDING.md", "DEPLOYMENT.md", "TESTING.md", ...]

[public]
files = ["README.md", "CHANGELOG.md", "CONTRIBUTING.md", "SECURITY.md", ...]

[diagram]
format = "mermaid"
```

모든 설정은 `alcove setup`으로 대화형으로 진행됩니다. 파일을 직접 편집할 수도 있습니다.

## 업데이트

```bash
cargo install alcove
```

## 삭제

```bash
alcove uninstall          # 스킬 & 설정 제거
cargo uninstall alcove    # 바이너리 제거
```

## 지원 에이전트

| 에이전트 | MCP | 스킬 |
|----------|-----|------|
| Claude Code | `~/.claude.json` | `~/.claude/skills/alcove/` |
| Cursor | `~/.cursor/mcp.json` | `~/.cursor/skills/alcove/` |
| Claude Desktop | 플랫폼 설정 | — |
| Cline (VS Code) | VS Code globalStorage | — |
| OpenCode | `~/.config/opencode/opencode.json` | `~/.opencode/skills/alcove/` |
| Codex CLI | `~/.codex/config.toml` | — |
| Antigravity | `~/.antigravity/settings.json` | — |
| Gemini CLI | `~/.gemini/settings.json` | `~/.gemini/skills/alcove/` |

## 지원 언어

CLI는 시스템 로케일을 자동 감지합니다. `ALCOVE_LANG` 환경 변수로 오버라이드할 수도 있습니다.

| 언어 | 코드 |
|------|------|
| English | `en` |
| 한국어 | `ko` |
| 简体中文 | `zh-CN` |
| 日本語 | `ja` |
| Español | `es` |
| हिन्दी | `hi` |
| Português (Brasil) | `pt-BR` |
| Deutsch | `de` |
| Français | `fr` |
| Русский | `ru` |

```bash
# 언어 오버라이드
ALCOVE_LANG=ko alcove setup
```

## 라이선스

Apache-2.0
