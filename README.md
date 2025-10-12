# Interactive CLI Business Card

A modern, interactive CLI business card available in both **Python** and **TypeScript**
implementations. Choose your preferred runtime and package manager!

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Python](https://img.shields.io/badge/Python-3.13+-blue.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue.svg)
![uvx](https://img.shields.io/badge/uvx-compatible-green.svg)
![npx](https://img.shields.io/badge/npx-compatible-green.svg)

## 🚀 Quick Start

### Python Version (uvx)

```bash
uvx carlosferreyra
```

### TypeScript Version (npx)

```bash
npx carlosferreyra
```

## Features

- 🎨 Beautiful terminal formatting with animations
- 📧 Direct email contact
- 📥 Quick resume access
- 🌐 Portfolio website access
- 💻 Professional links (GitHub, LinkedIn, Twitter)
- ⚡ Fast and responsive interface
- 🖥️ Interactive CLI menu
- 🚀 ASCII art banner
- 📦 Zero-install execution

## Project Structure

## Project Structure

```
business-card/
├── config.json            # 🎯 Centralized configuration (single source of truth)
├── config.schema.json     # JSON Schema for validation
├── CONFIGURATION.md       # Configuration documentation
├── python/                # Python implementation
│   ├── src/carlosferreyra/
│   ├── pyproject.toml
│   └── README.md
├── typescript/            # TypeScript implementation
│   ├── src/
│   ├── package.json
│   └── README.md
└── README.md              # This file
```

### Centralized Configuration

Both implementations share a single `config.json` file containing:

- **Personal Information**: Name, title, company, location, skills
- **URLs**: Email, resume, portfolio, GitHub, LinkedIn, Twitter
- **Theme**: Colors and animation settings

This ensures consistency across both versions. See [CONFIGURATION.md](CONFIGURATION.md) for details.

## 🔧 Configuration

Both implementations share a **single centralized configuration file** (`config.json`) at the root
of the repository. This ensures consistency across both Python and TypeScript versions.

The configuration includes:

- **Personal Information**: Name, title, company, location, and skills
- **URLs**: Email, resume, portfolio, GitHub, LinkedIn, and Twitter links
- **Theme**: Border color, background color, and animation speeds

To customize the business card for your own use, simply edit `config.json`:

```json
{
 "personalInfo": {
  "name": "Your Name",
  "title": "Your Title",
  "company": "Your Company",
  "location": "Your Location",
  "skills": ["Skill1", "Skill2", "Skill3"]
 },
 "urls": {
  "email": "mailto:your.email@example.com",
  "resume": "https://your-resume-url.com",
  "portfolio": "https://your-portfolio.com",
  "github": "https://github.com/yourusername",
  "linkedin": "https://linkedin.com/in/yourusername",
  "twitter": "https://twitter.com/yourusername"
 },
 "theme": {
  "borderColor": "cyan",
  "backgroundColor": "#1a1a2e",
  "animationSpeed": {
   "fast": 8,
   "medium": 25,
   "slow": 40
  }
 }
}
```

The `config.schema.json` file provides IDE autocompletion and validation support.

## Implementation Comparison

| Feature         | Python (uvx)         | TypeScript (npx)     |
| --------------- | -------------------- | -------------------- |
| Runtime         | Python 3.13+         | Node.js 18+          |
| Package Manager | PyPI (uv/pip)        | npm                  |
| Execution       | `uvx carlosferreyra` | `npx carlosferreyra` |
| Terminal UI     | Rich                 | Chalk + Boxen        |
| ASCII Art       | PyFiglet             | Figlet               |
| Animations      | Rich Spinner         | Nanospinner          |
| Gradients       | Rich Styles          | Gradient-string      |
| Menus           | Inquirer (Python)    | Inquirer (JS)        |

## Development

### Python Version

```bash
cd python/
uv sync
uv run python -m carlosferreyra
```

### TypeScript Version

```bash
cd typescript/
bun install
bun run dev
```

## Connect with Carlos

- **GitHub**: [github.com/carlosferreyra](https://github.com/carlosferreyra)
- **LinkedIn**: [linkedin.com/in/eduferreyraok](https://linkedin.com/in/eduferreyraok)
- **Website**: [carlosferreyra.me](https://carlosferreyra.me)
- **Email**: [eduferreyraok@gmail.com](mailto:eduferreyraok@gmail.com)

## License

MIT License - see individual implementations for details.
