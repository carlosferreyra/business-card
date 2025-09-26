# Contributing to Business Card CLI

Thanks for your interest in contributing! This project contains both Python and TypeScript
implementations of the same CLI business card.

## Development Setup

### Python Implementation

```bash
cd python/
uv sync
uv run python -m carlosferreyra
```

### TypeScript Implementation

```bash
cd typescript/
bun install  # or npm install
bun run dev  # or npm run dev
```

## Testing Both Versions

### Test Python version locally:

```bash
cd python/
uv build
uvx --from ./dist/carlosferreyra-*.whl carlosferreyra
```

### Test TypeScript version locally:

```bash
cd typescript/
bun run build
node dist/index.js
```

## Making Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes in the appropriate implementation directory
4. Test both versions if changes affect shared functionality
5. Submit a pull request

## Code Style

- **Python**: Follow PEP 8, use type hints
- **TypeScript**: Follow standard TypeScript conventions
- **Both**: Use meaningful variable names and add comments for complex logic

## Adding Features

When adding new features:

1. Implement in both Python and TypeScript versions
2. Maintain feature parity between implementations
3. Update documentation in both directories
4. Test on multiple platforms (macOS, Linux, Windows)
