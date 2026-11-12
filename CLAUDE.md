# Project Rules

Engineering rules and skills are global (`~/.claude/`, installed from agent-kit). The rule index in `~/.claude/rules/index.md` is always loaded; individual rules are lazy-loaded from `~/.claude/rules-custom/` when their trigger fires.

## Tech Stack & Workflow

- **Primary language:** Rust. Use the `~/.claude/rules-custom/*-<lang>.md` variants (architecture, naming-conventions, tech-matrix, testing-unit/integration/e2e) wherever a rule links to a per-language file.
- **Change workflow:** OpenSpec (`openspec/` directory, `opsx:*` commands/skills — `new`, `continue`, `apply`, `verify`, `archive`, etc.). All feature/change work goes through OpenSpec artifacts, not ad-hoc research/design/plan/implement docs.
- **Sources of truth, in order:** (1) the specification — the OpenSpec artifacts under `openspec/`; (2) the code. Nothing else outranks them.
- **No glossary.** The project keeps no vocabulary document of any kind. A term that needs defining is defined in the spec that uses it; a name that needs deciding is decided in that capability's spec and design artifacts.
