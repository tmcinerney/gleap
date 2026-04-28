# gleap

An unofficial command-line tool for the [Gleap](https://gleap.io) customer support API. Manage tickets, read conversations, and add notes — all from the terminal.

> **Disclaimer**: This is an **unofficial**, community-built tool. It is not affiliated with, endorsed by, or supported by Gleap. Use at your own risk.

## Installation

### Pre-built binaries (recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/tmcinerney/gleap/releases/latest):

| Platform | Asset |
|----------|-------|
| macOS (Apple Silicon) | `gleap-darwin-aarch64.tar.gz` |
| macOS (Intel) | `gleap-darwin-x86_64.tar.gz` |
| Linux (x86_64) | `gleap-linux-x86_64.tar.gz` |

```bash
curl -fsSL https://github.com/tmcinerney/gleap/releases/latest/download/gleap-darwin-aarch64.tar.gz | tar -xz
sudo mv gleap /usr/local/bin/
```

### From source

```bash
cargo install --path .
```

## Quick Start

```bash
# Authenticate (stored in system keychain)
gleap auth login

# List open tickets
gleap tickets list --status OPEN

# Search tickets
gleap tickets search "login bug"

# Add an internal note
gleap messages note --ticket <ticket-id> "Investigating — see GH issue #42"
```

## Authentication

Credentials are resolved in order: **environment variables** → **system keychain**.

### System keychain (recommended)

```bash
gleap auth login     # Prompts for API key and project ID
gleap auth status    # Show current auth source
gleap auth logout    # Remove stored credentials
```

Uses macOS Keychain, Windows Credential Manager, or Linux keyutils.

### Environment variables

For CI/automation, set these instead:

```bash
export GLEAP_API_KEY="your-api-key"
export GLEAP_PROJECT_ID="your-project-id"
```

Find your API key in Gleap under Project > Settings > Security.

`GLEAP_BASE_URL` can optionally override the API endpoint (default: `https://api.gleap.io/v3`).

## Usage

All commands output JSON. Pipe to `jq` for filtering.

### Tickets

```bash
# List with filters
gleap tickets list --status OPEN --type BUG --priority HIGH
gleap tickets list --status INPROGRESS --limit 10 --skip 10

# Get a single ticket
gleap tickets get <ID>

# Full-text search (returns up to 30 results ranked by relevance)
gleap tickets search "login button not working"

# Create a ticket (auto-tagged with "gleap-cli")
gleap tickets create "Login page broken on mobile"
gleap tickets create "Add dark mode" --type FEATURE_REQUEST --priority LOW --tags "ui,frontend"

# Update a ticket
gleap tickets update <ID> --status DONE
gleap tickets update <ID> --priority HIGH --title "New title"

# Delete a ticket
gleap tickets delete <ID>

# View captured logs
gleap tickets logs console <ID>
gleap tickets logs network <ID>
gleap tickets logs activity <ID>
```

### Messages

```bash
# List messages on a ticket
gleap messages list --ticket <ID>
gleap messages list --ticket <ID> --limit 5

# Add an internal note (team only)
gleap messages note --ticket <ID> "Root cause identified in auth service."

# Delete a message
gleap messages delete <ID>

# Reply to the customer
gleap messages reply --ticket <ID> "We've deployed a fix. Please try again."
```

### Collections (Help Center)

```bash
# List all collections
gleap collections list

# Get a single collection
gleap collections get <ID>

# Create a collection
gleap collections create --title "Getting Started"
gleap collections create --title "FAQ" --description "Frequently asked questions"

# Update a collection
gleap collections update <ID> --title "Updated Title"

# Delete a collection
gleap collections delete <ID>
```

### Articles (Help Center)

```bash
# List articles in a collection
gleap articles list --collection <ID>

# Get a single article
gleap articles get --collection <ID> <ARTICLE_ID>

# Create an article (defaults to draft, English)
gleap articles create --collection <ID> --title "Getting Started" --content-file content.json
gleap articles create --collection <ID> --title "Erste Schritte" --language de --published

# Update an article
gleap articles update --collection <ID> <ARTICLE_ID> --title "Updated Title"
gleap articles update --collection <ID> <ARTICLE_ID> --content-file updated.json --published true

# Delete an article
gleap articles delete --collection <ID> <ARTICLE_ID>

# Move an article to a different collection
gleap articles move --collection <ID> <ARTICLE_ID> --to <TARGET_COLLECTION_ID>
```

## Verbose Output

Use `-v` flags globally for debugging:

```bash
gleap -v tickets list       # Request method, URL, response status, timing
gleap -vv tickets list      # + response headers, raw body on errors
gleap -vvv tickets list     # + full response body always
```

## API Coverage

| Resource | Operations |
|----------|-----------|
| **Auth** | login, logout, status |
| **Tickets** | list, get, search, create, update, delete, logs (console, network, activity) |
| **Messages** | list, note (internal), reply (comment), delete |
| **Collections** | list, get, create, update, delete |
| **Articles** | list, get, create, update, delete, move |

The Gleap API has many more endpoints (engagements, surveys, statistics, sessions, etc.) that are not yet implemented. Contributions welcome.

### References

- [API Overview](https://docs.gleap.io/documentation/server/api-overview) — REST API docs and querying guide
- [Postman Collection](https://documenter.getpostman.com/view/18586034/2s8YRiJYVC) — Interactive API explorer

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 2 | Authentication failure (401/403) |
| 3 | API error |
| 4 | Configuration error |
| 5 | Not found (404) |
| 6 | Rate limited (429) |
| 7 | HTTP error |
| 8 | IO error |
| 9 | Serialization error |

## Development

```bash
cargo build
cargo test
```

### Project Structure

```
src/
├── main.rs              # CLI entry point and command dispatch
├── lib.rs               # Library crate re-exports
├── config/              # Credential resolution (env vars → keychain)
│   ├── mod.rs
│   └── keychain.rs
├── error.rs             # Error types and exit codes
├── cli/                 # Clap argument definitions
│   ├── auth.rs
│   ├── tickets.rs
│   ├── messages.rs
│   ├── collections.rs
│   ├── articles.rs
│   └── shared.rs        # Shared args (pagination)
├── client/              # Gleap API HTTP client
│   ├── mod.rs           # GleapClient (auth, request helpers, verbose logging)
│   ├── tickets.rs
│   ├── messages.rs
│   ├── collections.rs
│   └── articles.rs
├── models/              # Request/response types
│   ├── ticket.rs
│   ├── message.rs
│   ├── collection.rs
│   └── article.rs
└── commands/            # Command handlers
    ├── auth.rs
    ├── tickets/         # list, get, search, create, update, delete, logs
    ├── messages/        # list, note, reply, delete
    ├── collections/     # list, get, create, update, delete
    └── articles/        # list, get, create, update, delete, move
```

## License

MIT
