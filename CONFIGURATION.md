# Configuration Guide

This document explains the centralized configuration system used by both the Python and TypeScript
implementations of the CLI business card.

## Overview

Both implementations share a single `config.json` file located at the root of the repository. This
ensures:

- **Consistency**: Personal information is identical across both versions
- **Maintainability**: Update once, deploy everywhere
- **Simplicity**: One source of truth for all data

## Configuration File Location

The `config.json` file is located at:

```
business-card/config.json
```

Both implementations will:

1. First try to load from the repository root (development mode)
2. Fall back to a bundled copy in the distribution package (production mode)

## Configuration Structure

### Personal Information

```json
"personalInfo": {
  "name": "Your Full Name",
  "title": "Your Professional Title",
  "company": "Your Company (optional)",
  "location": "Your Location",
  "skills": ["Skill 1", "Skill 2", "Skill 3"]
}
```

- **name**: Your full name as displayed in the banner
- **title**: Your professional title or role
- **company**: Your current company (can be omitted if not applicable)
- **location**: Your city/country or region
- **skills**: Array of your top skills (recommended: 4-8 items)

### URLs

```json
"urls": {
  "email": "mailto:your.email@example.com",
  "resume": "",
  "portfolio": "https://your-portfolio.com",
  "github": "https://github.com/yourusername",
  "linkedin": "https://linkedin.com/in/yourusername",
  "twitter": "https://twitter.com/yourusername"
}
```

- **email**: Use `mailto:` prefix for email links
- **resume**: Direct link to your resume (PDF recommended), or `""` to hide it
- **portfolio**: Your personal website or portfolio, or `""` to hide it
- **github**: Your GitHub profile URL, or `""` to hide it
- **linkedin**: Your LinkedIn profile URL, or `""` to hide it
- **twitter**: Your Twitter/X profile URL (optional, can be `""`)

When a URL is empty, both implementations automatically hide that menu option and related card row.

### Theme

```json
"theme": {
  "borderColor": "cyan",
  "backgroundColor": "#1a1a2e",
  "animationSpeed": {
    "fast": 8,
    "medium": 25,
    "slow": 40
  }
}
```

- **borderColor**: Terminal color name (e.g., "cyan", "magenta", "green")
- **backgroundColor**: Hex color code for background
- **animationSpeed**: Values in milliseconds
  - Used directly in TypeScript
  - Converted to seconds in Python (divided by 1000)

## Implementation Details

### TypeScript Implementation

Location: `typescript/src/config.ts`

The TypeScript version:

- Reads from `../../config.json` (development) or `../config.json` (production)
- Uses the animation speed values directly (milliseconds)
- Bundled with the package during `npm run build`

### Python Implementation

Location: `python/src/carlosferreyra/config.py`

The Python version:

- Reads from the root config.json (development) or bundled copy (production)
- Converts animation speeds from milliseconds to seconds
- Included in the package via `tool.uv.include` in `pyproject.toml`

## Customizing for Your Own Use

1. **Edit config.json**:

   ```bash
   # Edit the root configuration file
   vim config.json
   ```

2. **Validate your changes**:
   - The `config.schema.json` provides JSON Schema validation
   - Most IDEs will show errors if the structure is invalid

3. **Test locally**:

   ```bash
   # Python version
   cd python && uv run python -m carlosferreyra

   # TypeScript version
   cd typescript && bun run dev
   ```

4. **Build and publish**:

   ```bash
   # Python
   cd python && uv build

   # TypeScript
   cd typescript && npm run build
   ```

## Schema Validation

The `config.schema.json` file provides:

- IDE autocomplete support
- Type validation
- Required field checking
- Format validation for URLs

To enable schema validation in your IDE, ensure the `$schema` property in `config.json` points to
the schema file.

## Common Issues

### Config file not found

If you see "Failed to load configuration file":

1. Ensure `config.json` exists in the repository root
2. Check file permissions
3. Verify the JSON syntax is valid

### Animation speeds look wrong

- **TypeScript**: Uses milliseconds directly (8-40ms typical)
- **Python**: Converts to seconds automatically (0.008-0.04s typical)

If animations are too fast or slow, adjust the values in `config.json`.

### Missing required fields

The schema requires:

- All `personalInfo` fields except `company`
- All `theme` fields

For `urls`, fields can be omitted or set to `""`. Missing/empty URL fields are treated as "not
configured" and their menu items are hidden.

## Contributing

When contributing to this project:

1. Never hardcode personal information in the source files
2. Always use the centralized `config.json`
3. Update `config.schema.json` if adding new fields
4. Test both Python and TypeScript implementations after config changes
