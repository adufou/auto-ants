# Auto-Cult

A Bevy-based game project.

## Development Setup

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed setup instructions and development guidelines.

Quick start:
```bash
# Install pre-commit hooks
./scripts/setup-hooks.sh

# Build and run
cargo run
```

## Project Structure

- `src/components/` - ECS components (camera, markers, movement)
- `src/resources/` - Game resources (camera, rendering, spatial, terrain)
- `src/systems/` - Game systems
- `src/plugins/` - Bevy plugins
- `assets/` - Game assets (sprites, textures)
