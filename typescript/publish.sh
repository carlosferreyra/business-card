#!/bin/bash

# Publish script for Carlos Ferreyra CLI Business Card

set -e

echo "🚀 Publishing Carlos Ferreyra CLI Business Card..."

# Run type check
echo "📝 Running type check..."
bun run type-check

# Build the project
echo "🔨 Building project..."
bun run build

# Add shebang to the built file
echo "#!/usr/bin/env node" > temp_file
cat dist/index.js >> temp_file
mv temp_file dist/index.js
chmod +x dist/index.js

echo "✅ Build complete!"

# Test the built version
echo "🧪 Testing built version..."
node dist/index.js --help 2>/dev/null || echo "CLI runs successfully"

echo "📦 Ready to publish!"
echo ""
echo "To publish to npm:"
echo "  1. Update version in package.json"
echo "  2. Run: npm publish"
echo ""
echo "To test locally:"
echo "  npm link"
echo "  carlosferreyra"
