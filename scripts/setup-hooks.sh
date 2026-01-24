#!/usr/bin/env bash
# Setup script for pre-commit hooks

set -e

echo "🔧 Setting up pre-commit hooks for auto-cult..."
echo ""

# Check if Python is available and working
PYTHON_CMD=""
for cmd in python python3; do
    if command -v $cmd &> /dev/null; then
        # Test if it actually works
        if $cmd --version &> /dev/null; then
            PYTHON_CMD=$cmd
            break
        fi
    fi
done

if [ -z "$PYTHON_CMD" ]; then
    echo "❌ ERROR: Python not found. Please install Python 3.7+"
    echo "   Download from: https://www.python.org/"
    exit 1
fi

echo "✓ Found Python: $($PYTHON_CMD --version)"

# Check if pre-commit is installed
if ! $PYTHON_CMD -m pre_commit --version &> /dev/null; then
    echo ""
    echo "📦 pre-commit not found, installing..."
    $PYTHON_CMD -m pip install --user pre-commit

    # Verify installation worked
    if ! $PYTHON_CMD -m pre_commit --version &> /dev/null; then
        echo ""
        echo "❌ ERROR: Failed to install pre-commit"
        echo "   Try installing manually: $PYTHON_CMD -m pip install pre-commit"
        exit 1
    fi
fi

echo "✓ pre-commit is installed"

# Install the git hook scripts
echo ""
echo "📌 Installing pre-commit hooks..."
$PYTHON_CMD -m pre_commit install

# Run hooks on all files to verify setup
echo ""
echo "🧪 Running hooks on all files to verify setup..."
echo "   (This may take a moment...)"
$PYTHON_CMD -m pre_commit run --all-files || true

echo ""
echo "✅ Setup complete! Pre-commit hooks are now active."
echo ""
echo "📝 How it works:"
echo "   • Hooks run automatically when you 'git commit'"
echo "   • cargo fmt will format staged Rust files"
echo "   • cargo check will verify code compiles"
echo ""
echo "💡 Useful commands:"
echo "   • Run hooks manually:     pre-commit run --all-files"
echo "   • Skip hooks once:        git commit --no-verify"
echo "   • Update hook versions:   pre-commit autoupdate"
echo ""
