# gleap

An unofficial command-line tool for interacting with the [Gleap](https://gleap.io) customer support API. Manage tickets, read conversations, and add notes — all from the terminal.

> **Disclaimer**: This is an **unofficial**, community-built tool. It is not affiliated with, endorsed by, or supported by Gleap. Use at your own risk. No guarantees are provided regarding completeness, accuracy, or compatibility with future Gleap API changes.

## Quick Start

```bash
# Install
cargo install --path .

# Set credentials
export GLEAP_API_KEY="your-api-key"
export GLEAP_PROJECT_ID="your-project-id"

# List open tickets
gleap tickets list --status OPEN

# Get a specific ticket
gleap tickets get <ticket-id>

# Add an internal note
gleap messages note --ticket <ticket-id> "Investigating — see GH issue #42"
```

## Installation

### Using Cargo

```bash
# Install from local source
cargo install --path .

# Or build and install with locked dependencies
cargo install --path . --locked
```

This installs `gleap` to your cargo bin directory (typically `~/.cargo/bin`), which should be in your PATH if you're using rustup.

### Manual Build

```bash
cargo build --release
cp target/release/gleap /usr/local/bin/
```

### Uninstallation

```bash
# If installed via cargo
cargo uninstall gleap

# If installed manually
rm /usr/local/bin/gleap
```

## Configuration

Set the following environment variables:

| Variable | Required | Description |
|----------|----------|-------------|
| `GLEAP_API_KEY` | Yes | Gleap API key (find in Project > Settings > Security) |
| `GLEAP_PROJECT_ID` | Yes | Gleap project ID |
| `GLEAP_BASE_URL` | No | API base URL (default: `https://api.gleap.io/v3`) |

```bash
export GLEAP_API_KEY="your-api-key"
export GLEAP_PROJECT_ID="your-project-id"
```

## Output Format

All commands output JSON. Pipe to `jq` for filtering and transformation.

```bash
# Get ticket titles
gleap tickets list | jq '.tickets[].title'

# Get open bug count
gleap tickets list --status OPEN --type BUG | jq '.totalCount'

# Extract conversation text
gleap messages list --ticket <id> | jq '.[].comment'
```

## Usage

### Tickets

#### List tickets

```bash
gleap tickets list [OPTIONS]
```

**Options:**
- `--status <STATUS>` — Filter by status (`OPEN`, `IN_PROGRESS`, `DONE`)
- `--type <TYPE>` — Filter by type (`BUG`, `FEATURE_REQUEST`, `INQUIRY`)
- `--priority <PRIORITY>` — Filter by priority (`LOW`, `MEDIUM`, `HIGH`)
- `--sort <SORT>` — Sort field with direction prefix (default: `-createdAt`)
- `-l, --limit <N>` — Max results (default: 20)
- `-s, --skip <N>` — Offset for pagination (default: 0)

```bash
# List open bugs, newest first
gleap tickets list --status OPEN --type BUG

# List high priority tickets
gleap tickets list --priority HIGH --sort -priority

# Paginate through results
gleap tickets list --limit 10 --skip 10
```

#### Get a ticket

```bash
gleap tickets get <ID>
```

Returns the full ticket object including form data, custom data, screenshots, and metadata.

```bash
# Get ticket and extract screenshot URL
gleap tickets get abc123 | jq '.imageUrl'

# Get ticket description
gleap tickets get abc123 | jq '.formData.description'
```

#### Search tickets

```bash
gleap tickets search <QUERY> [OPTIONS]
```

**Options:**
- `-l, --limit <N>` — Max results (default: 20)
- `-s, --skip <N>` — Offset for pagination (default: 0)

```bash
gleap tickets search "login button not working"
```

#### Update a ticket

```bash
gleap tickets update <ID> [OPTIONS]
```

**Options:**
- `--status <STATUS>` — New status
- `--priority <PRIORITY>` — New priority
- `--title <TITLE>` — New title

```bash
# Close a ticket
gleap tickets update abc123 --status DONE

# Escalate priority
gleap tickets update abc123 --priority HIGH
```

#### Get ticket logs

```bash
# Console logs (JavaScript errors, console output)
gleap tickets console-logs <ID>

# Network logs (HTTP requests/responses)
gleap tickets network-logs <ID>

# Activity logs (ticket history)
gleap tickets activity-logs <ID>
```

### Messages

#### List messages for a ticket

```bash
gleap messages list --ticket <ID> [OPTIONS]
```

**Options:**
- `--sort <SORT>` — Sort field (default: `createdAt`)
- `-l, --limit <N>` — Max results (default: 20)
- `-s, --skip <N>` — Offset for pagination (default: 0)

```bash
# Get full conversation thread
gleap messages list --ticket abc123

# Get latest messages first
gleap messages list --ticket abc123 --sort -createdAt --limit 5
```

#### Add an internal note

```bash
gleap messages note --ticket <ID> "Note text"
```

Internal notes are visible to your team only, not to the customer.

```bash
gleap messages note --ticket abc123 "Linked to GitHub issue #42. Root cause identified in auth service."
```

#### Reply to a ticket

```bash
gleap messages reply --ticket <ID> "Reply text"
```

```bash
gleap messages reply --ticket abc123 "We've deployed a fix. Please try again and let us know."
```

## API Coverage

This CLI currently covers a subset of the Gleap API, focused on ticket management and conversations. The following endpoints are implemented:

| Resource | Operations |
|----------|-----------|
| **Tickets** | list, get, search, update, console-logs, network-logs, activity-logs |
| **Messages** | list, note (internal), reply (comment) |

The Gleap API has many more endpoints (help center, engagements, surveys, statistics, sessions, etc.) that are not yet implemented. Contributions welcome.

For the full Gleap API documentation, see [docs.gleap.io](https://docs.gleap.io/documentation/server/api-overview).

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

### Building

```bash
cargo build
cargo test
```

### Project Structure

```
src/
├── main.rs              # CLI entry point and command dispatch
├── lib.rs               # Library crate re-exports
├── config.rs            # Environment variable configuration
├── error.rs             # Error types and exit codes
├── cli/                 # Clap argument definitions
│   ├── tickets.rs       # Ticket subcommands
│   ├── messages.rs      # Message subcommands
│   └── shared.rs        # Shared args (pagination)
├── client/              # Gleap API HTTP client
│   ├── mod.rs           # GleapClient base (auth, request helpers)
│   ├── tickets.rs       # Ticket API methods
│   └── messages.rs      # Message API methods
├── models/              # Request/response types
│   ├── ticket.rs        # Ticket, TicketFilters, enums
│   └── message.rs       # Message, CreateMessageRequest
└── commands/            # Command handlers
    ├── tickets/         # list, get, search, update, logs
    └── messages/        # list, note, reply
```

### Adding New Resource Groups

The codebase is namespaced for expansion. To add support for a new API resource (e.g., help center):

1. Add models: `src/models/help_center.rs`
2. Add client methods: `src/client/help_center.rs`
3. Add CLI args: `src/cli/help_center.rs`
4. Add commands: `src/commands/help_center/`
5. Register the new domain variant in `src/cli/mod.rs`

## License

MIT
