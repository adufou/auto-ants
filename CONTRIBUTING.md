# Contributing to Auto-Cult

## Getting Started

### Prerequisites
- Rust 1.93+ (edition 2024)
- Python 3.7+ (for pre-commit hooks)
- Git

### Development Setup

1. Clone the repository
2. Run the setup script to install pre-commit hooks:

   ```bash
   ./scripts/setup-hooks.sh
   ```

3. Start developing!

### Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com/) to ensure code quality.

**Installed hooks:**
- `cargo fmt` - Automatically format Rust code
- `cargo check` - Verify code compiles
- File checks - Prevent large files, merge conflicts, trailing whitespace

**The hooks run automatically** when you commit. If formatting issues are found, they will be auto-fixed. Just re-stage and commit again:

```bash
git add .
git commit -m "Your message"
# If hooks auto-fix files:
git add .
git commit -m "Your message"
```

**Manual execution:**
```bash
# Run on staged files
python -m pre_commit run

# Run on all files
python -m pre_commit run --all-files

# Skip hooks (use sparingly)
git commit --no-verify -m "Your message"
```

**Updating hooks:**
```bash
python -m pre_commit autoupdate
```

### Code Style

- Follow Rust standard formatting (enforced by `cargo fmt`)
- Use conventional commits: `feat:`, `fix:`, `chore:`, `refacto:`
- Keep commits atomic and focused

### Project Structure

```
auto-cult/
├── src/
│   ├── components/        # ECS components (camera, markers, movement)
│   ├── resources/         # Game resources (camera, rendering, spatial, terrain)
│   ├── systems/           # Game systems
│   └── plugins/           # Bevy plugins
├── assets/                # Game assets (sprites, textures)
└── scripts/               # Development scripts
```

## Questions?

Feel free to open an issue or reach out to the team!
