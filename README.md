# 🦅 BLOODY-F4LCON

Terminal-first OSINT recon for usernames. Red/Black vibe, production-hardening: rate limiting, cache, configurable providers, headless JSON mode.

## ✨ Features
- Live provider checks (GitHub, Reddit, Steam, Twitter, PSNProfiles by default)
- Rate limiting + backoff, cache with TTL
- Configurable providers/user-agent via TOML
- TUI with active targets, intel feed, logs
- Headless mode (`--no-tui`) for scripting
- Tracing to stdout + `data/falcon.log`

## 📦 Install
**From repo**
```bash
cargo install --path .
```

**Direct from Git**
```bash
cargo install --git git@github.com:ind4skylivey/bloody-f4lcon.git
```

## 🚀 Quick Start
```bash
# Scan "shadow" with defaults
bloody-f4lcon shadow

# Limit to GitHub + Reddit
bloody-f4lcon shadow --providers github,reddit

# Disable cache
bloody-f4lcon shadow --no-cache

# Custom config
bloody-f4lcon shadow --config config/bloodyf4lcon.toml

# Headless JSON (no TUI)
bloody-f4lcon shadow --no-tui > result.json
```

## 🎮 TUI Controls
- ENTER → Scan current target (or add if input filled)
- TAB → Switch target
- q → Exit
- Backspace → Delete input

Panels:
- Header: version + platform count + hint strip
- Active Targets: index, id, hits, status
- Intel Feed: status, hits, platforms, restricted/rate-limited/failed, optional label
- Scan Engine: progress gauge or prompt
- System Logs: rolling feed

## ⚙️ Configuration
File: `config/bloodyf4lcon.toml`
```toml
timeout_ms = 5000
max_concurrent_requests = 5
cache_ttl_seconds = 600
user_agent = "bloody-f4lcon/1.0 (+https://github.com/ind4skylivey/bloody-f4lcon)"

[[providers]]
name = "github"
enabled = true
base_url = "https://github.com/{username}"
# ... add more providers as needed
```
Flags override pieces:
- `--config <path>` load alternate file
- `--providers a,b,c` enable subset (case-insensitive)
- `--no-cache` disable cache
- `--verbose` (repeat for debug/trace)
- `--log-file <path>` change log destination
- `--no-tui` headless JSON

## 🧪 Development
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Test: `cargo test`

GitHub Actions CI runs fmt + clippy + tests.

## ⚖️ Legal / Ethical
Use only on targets you are authorized to probe. OSINT still carries privacy and ToS considerations. You are responsible for respecting platform policies and local laws.

## 📸 Visual
![demo](docs/screenshot.png)

