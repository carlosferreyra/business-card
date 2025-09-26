# Carlos Ferreyra CLI Business Card

A modern, interactive CLI business card showcasing Carlos Ferreyra's portfolio and contact
information with beautiful animations and gradient colors.

Built with TypeScript and Bun for optimal performance and developer experience.

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?logo=typescript&logoColor=white)
![Bun](https://img.shields.io/badge/Bun-000000?logo=bun&logoColor=white)

## Features

- 🎨 Beautiful gradient colors and animations
- 📧 Direct email contact
- 📥 Quick resume access
- 🌐 Portfolio website access
- 💻 Professional links (GitHub, LinkedIn, Twitter)
- ⚡ Fast and responsive interface
- 🖥️ User-friendly CLI menu
- 🔧 Built with TypeScript for type safety
- 🚀 Powered by Bun for fast execution

## Quick Start

Run the business card directly in your terminal:

```bash
npx carlosferreyra
```

No installation required! Just run the command and interact with the card.

## Development

### Prerequisites

- [Bun](https://bun.sh) (recommended) or Node.js >= 18
- TypeScript

### Installation

```bash
# Clone the repository
git clone https://github.com/carlosferreyra/carlosferreyra-cli-ts.git
cd carlosferreyra-cli-ts

# Install dependencies
bun install

# Run in development mode
bun run dev

# Build for production
bun run build

# Run the built version
bun run start
```

### Scripts

- `bun run dev` - Run in development mode with hot reload
- `bun run build` - Build the TypeScript project
- `bun run start` - Run the compiled JavaScript
- `bun run type-check` - Check TypeScript types without emitting

## Technologies Used

- **Bun** - Fast JavaScript runtime and package manager
- **TypeScript** - Type-safe JavaScript
- **Chalk** - Terminal styling
- **Boxen** - Boxes in terminal
- **Inquirer** - Interactive CLI
- **Gradient-string** - Beautiful gradients
- **Figlet** - ASCII art text
- **Nanospinner** - Loading animations
- **Open** - Open URLs in default browser

## Customization

To customize this business card for yourself:

1. Update the personal information in `src/config.ts`
2. Modify the theme colors and animation speeds
3. Add or remove menu options in `src/menu.ts`
4. Update the action handlers in `src/actions.ts`

## Project Structure

```bash
src/
├── index.ts      # Main application entry point
├── config.ts     # Personal information and configuration
├── types.ts      # TypeScript type definitions
├── utils.ts      # Utility functions
├── banner.ts     # Welcome banner with ASCII art
├── card.ts       # Business card display
├── menu.ts       # Interactive menu system
└── actions.ts    # Action handlers for menu options
```

## Connect with Carlos

- **GitHub**: [github.com/carlosferreyra](https://github.com/carlosferreyra)
- **LinkedIn**: [linkedin.com/in/carlosferreyra](https://linkedin.com/in/carlosferreyra)
- **Website**: [carlosferreyra.dev](https://carlosferreyra.dev)
- **Email**: [carlos@example.com](mailto:carlos@example.com)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Changelog

### Version 1.0.0

- Initial TypeScript implementation
- Bun runtime support
- Modern CLI interface with gradients and animations
- Interactive menu system
- Professional business card display
- Type-safe configuration system Personal CLI tool for carlosferreyra (Typescript version)
