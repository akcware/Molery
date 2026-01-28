# Molery

macOS desktop GUI for the [Mole CLI](https://github.com/tw93/Mole) (`mo`) - a system optimizer for cleaning caches, analyzing disk usage, and managing system resources.

## Tech Stack

- **Frontend**: Svelte 5 + TypeScript + TailwindCSS
- **Backend**: Tauri 2.x (Rust)
- **CLI**: `mo` (Mole)

## Documentation

| Document | Description |
|----------|-------------|
| [docs/README.md](docs/README.md) | Documentation index |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | System design and file structure |
| [tasks/README.md](tasks/README.md) | Development task dashboard |
| [.ai/CONTEXT.md](.ai/CONTEXT.md) | Quick project context for LLM agents |
| [.ai/CONVENTIONS.md](.ai/CONVENTIONS.md) | Code patterns and naming rules |

## Development

### Prerequisites

- Node.js 18+
- Rust toolchain
- `mo` CLI installed (`brew install tw93/brew/mole`)

### Setup

```bash
# Install dependencies
bun install

# Run in development mode
bun run tauri dev

# Build for production
bun run tauri build
```

### Project Structure

```
Molery/
├── .ai/                    # LLM agent context files
├── docs/                   # Project documentation
│   ├── specs/              # Technical specifications
│   └── flows/              # User/system flows
├── tasks/                  # Development phases
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── services/       # Tauri invoke wrappers
│   │   ├── stores/         # State management
│   │   └── types/          # TypeScript definitions
│   └── routes/             # SvelteKit pages
└── src-tauri/              # Rust backend
    └── src/
        └── commands/       # Tauri commands
```

## Features

- **Dashboard**: System overview with quick stats
- **Clean**: Scan and remove cache, logs, trash
- **Uninstall**: Fully remove applications and associated files
- **Analyze**: Visualize disk usage by folder
- **Optimize**: System performance optimizations
- **Status**: System health overview

## License

MIT
