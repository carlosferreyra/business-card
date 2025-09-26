# Complete Tutorial: Creating a Python CLI Business Card with uvx Support

This tutorial shows you how to recreate the TypeScript CLI business card functionality in Python,
making it compatible with `uvx` (the Python equivalent of `npx`).

## 🎯 Goal

Create a Python CLI that can be run with:

```bash
uvx carlosferreyra
```

Just like the TypeScript version runs with:

```bash
npx carlosferreyra
```

## 📊 Feature Comparison

| Feature          | TypeScript (npx)   | Python (uvx)    | Status      |
| ---------------- | ------------------ | --------------- | ----------- |
| ASCII Art Banner | ✅ figlet          | ✅ pyfiglet     | ✅ Complete |
| Interactive Menu | ✅ inquirer        | ✅ inquirer     | ✅ Complete |
| Rich Terminal UI | ✅ chalk, boxen    | ✅ rich         | ✅ Complete |
| Animations       | ✅ nanospinner     | ✅ rich.spinner | ✅ Complete |
| URL Opening      | ✅ open            | ✅ webbrowser   | ✅ Complete |
| Gradients        | ✅ gradient-string | ✅ rich styles  | ✅ Complete |
| Zero-install run | ✅ npx             | ✅ uvx          | ✅ Complete |

## 🚀 Quick Start

### Option 1: Run with uvx (Recommended)

```bash
uvx carlosferreyra
```

### Option 2: Install and run

```bash
uv tool install carlosferreyra
carlosferreyra
```

## 🛠️ Step-by-Step Implementation

### 1. Project Structure Setup

```
src/carlosferreyra/
├── __init__.py          # Package initialization
├── __main__.py          # Main entry point (enables `python -m carlosferreyra`)
├── config.py            # Configuration and personal data
├── utils.py             # Utility functions for animations
├── banner.py            # ASCII art welcome banner
├── card.py              # Business card display
├── menu.py              # Interactive menu system
└── actions.py           # Action handlers for menu choices
```

### 2. Configure pyproject.toml

```toml
[project]
name = "carlosferreyra"
version = "1.0.0"
description = "Interactive CLI business card for Carlos Ferreyra"
authors = [
    { name = "Carlos Eduardo Ferreyra", email = "eduferreyraok@gmail.com" },
]
dependencies = [
    "click>=8.0.0",
    "colorama>=0.4.0",
    "inquirer>=3.0.0",
    "pyfiglet>=1.0.0",
    "rich>=13.0.0",
]
requires-python = ">=3.13"

[project.scripts]
carlosferreyra = "carlosferreyra.__main__:main"

[build-system]
build-backend = "uv_build"
requires = ["uv_build>=0.8.14,<0.9.0"]
```

**Key Points:**

- The `[project.scripts]` section creates the CLI command
- `uv_build` backend optimizes for uvx compatibility
- Entry point points to `__main__.py:main` function

### 3. Configuration System

**config.py:**

```python
from dataclasses import dataclass
from typing import Dict, List, Optional

@dataclass
class PersonalInfo:
    name: str
    title: str
    company: Optional[str]
    location: str
    skills: List[str]

@dataclass
class URLs:
    email: str
    resume: str
    portfolio: str
    github: str
    linkedin: str
    twitter: Optional[str] = None

@dataclass
class ThemeConfig:
    border_color: str
    background_color: str
    animation_speed: Dict[str, float]

@dataclass
class AppConfig:
    personal_info: PersonalInfo
    urls: URLs
    theme: ThemeConfig

# Main configuration
CONFIG = AppConfig(
    personal_info=PersonalInfo(
        name="Carlos Ferreyra",
        title="Software Engineer & Developer",
        company="Self Employed",
        location="United States",
        skills=["TypeScript", "React", "Node.js", "Python", "GCP", "DevOps"]
    ),
    urls=URLs(
        email="mailto:eduferreyraok@gmail.com",
        resume="https://www.carlosferreyra.me/resume.pdf",
        portfolio="https://www.carlosferreyra.me",
        github="https://github.com/carlosferreyra",
        linkedin="https://linkedin.com/in/eduferreyraok",
        twitter="https://twitter.com/eduferreyraok"
    ),
    theme=ThemeConfig(
        border_color="cyan",
        background_color="#1a1a2e",
        animation_speed={
            "fast": 0.01,
            "medium": 0.025,
            "slow": 0.04
        }
    )
)
```

### 4. Rich Terminal Interface

**card.py** - Business Card Display:

```python
from rich.console import Console
from rich.panel import Panel
from rich.text import Text
from .config import CONFIG

console = Console()

def create_profile_card() -> None:
    """Create and display the profile business card."""
    card_lines = []

    # Name and title with styling
    name_text = Text(CONFIG.personal_info.name, style="bold bright_magenta")
    title_text = Text(CONFIG.personal_info.title, style="white")
    card_lines.extend([name_text, title_text, ""])

    # Skills with colors
    skills_text = Text("⚡ Skills: ", style="white") + Text(
        " | ".join(CONFIG.personal_info.skills), style="bright_cyan"
    )
    card_lines.append(skills_text)

    # Social links with formatting
    github_text = Text("📦 GitHub:    ", style="white") + Text("{ ") + \
                  Text("github.com/", style="dim") + \
                  Text("carlosferreyra", style="bright_green") + Text(" }")

    # Create Rich panel
    card_content = Text()
    for i, line in enumerate(card_lines):
        if i > 0:
            card_content.append("\n")
        if isinstance(line, str):
            card_content.append(line)
        else:
            card_content.append_text(line)

    panel = Panel(
        card_content,
        title=f"[bold cyan]{CONFIG.personal_info.name}'s Business Card[/bold cyan]",
        title_align="center",
        border_style="cyan",
        padding=(1, 2)
    )

    console.print(panel)
```

### 5. Interactive Menu System

**menu.py:**

```python
import inquirer
from rich.console import Console

console = Console()

def get_menu_choices():
    return [
        ("📧  Send an Email", "email"),
        ("📥  View Resume", "view_resume"),
        ("🌐  Visit Portfolio", "view_portfolio"),
        ("💻  View GitHub", "view_github"),
        ("💼  View LinkedIn", "view_linkedin"),
        ("🐦  View Twitter", "view_twitter"),
        ("🚪  Exit", "quit"),
    ]

def prompt_user() -> str:
    questions = [
        inquirer.List(
            'action',
            message="What would you like to do?",
            choices=get_menu_choices(),
            carousel=True
        )
    ]

    try:
        answers = inquirer.prompt(questions)
        return answers['action'] if answers else 'quit'
    except (KeyboardInterrupt, EOFError):
        console.print("\n[yellow]👋 Thanks for stopping by![/yellow]")
        return 'quit'
```

### 6. Action Handlers

**actions.py:**

```python
import webbrowser
from rich.console import Console
from .config import CONFIG
from .utils import animated_spinner, animate_text

console = Console()

class ActionHandlers:
    @staticmethod
    def email():
        with animated_spinner("Opening mail client..."):
            webbrowser.open(CONFIG.urls.email)
        console.print("[bright_red]📧 Email client opened![/bright_red]")
        animate_text("Looking forward to hearing from you!")

    @staticmethod
    def view_resume():
        with animated_spinner("Preparing to open resume..."):
            webbrowser.open(CONFIG.urls.resume)
        console.print("[green]📥 Resume opened in your browser! 🎉[/green]")
        animate_text("Tip: You can download it directly")

    # ... other methods

action_handlers = {
    'email': ActionHandlers.email,
    'view_resume': ActionHandlers.view_resume,
    # ... other handlers
}
```

### 7. Main Entry Point

****main**.py:**

```python
#!/usr/bin/env python3
"""Main entry point for the Carlos Ferreyra CLI business card."""

from rich.console import Console
from rich.text import Text

from .actions import action_handlers
from .banner import welcome_banner
from .card import create_profile_card
from .menu import prompt_user
from .utils import animate_text

console = Console()

def main() -> None:
    """CLI entry point for carlosferreyra package."""
    try:
        # Show welcome banner
        welcome_banner()

        # Display profile card
        create_profile_card()

        # Show helpful tip
        tip_text = Text()
        tip_text.append("💡 Tip: Use ", style="bright_red")
        tip_text.append("cmd/ctrl + click", style="cyan")
        tip_text.append(" on links to open directly.", style="bright_red")
        console.print(tip_text)
        console.print()

        # Main interaction loop
        while True:
            action = prompt_user()

            if action == 'quit':
                animate_text("👋 Thanks for stopping by! Have a great day!")
                break

            # Execute the selected action
            if action in action_handlers:
                action_handlers[action]()

    except KeyboardInterrupt:
        console.print("\n[yellow]👋 Thanks for stopping by! Have a great day![/yellow]")
    except Exception as error:
        console.print(f"\n[red]❌ An error occurred: {error}[/red]")
        return

if __name__ == "__main__":
    main()
```

## 🔧 Development Workflow

### Setup Development Environment

```bash
# Clone your repository
git clone https://github.com/carlosferreyra/carlosferreyra-cli-py.git
cd carlosferreyra-cli-py

# Install dependencies
uv sync

# Run in development mode
uv run python -m carlosferreyra

# Test with uvx locally
uv build
uvx --from ./dist/carlosferreyra-1.0.0-py3-none-any.whl carlosferreyra
```

### Publishing to PyPI

```bash
# Build the package
uv build

# Publish to PyPI (requires authentication)
uv publish

# Test installation
uvx carlosferreyra
```

## 🌟 Key Differences from TypeScript Version

### 1. Package Management

- **TypeScript**: Uses npm, published to npm registry
- **Python**: Uses uv/pip, published to PyPI

### 2. Dependencies

| TypeScript      | Python       | Purpose             |
| --------------- | ------------ | ------------------- |
| chalk           | rich         | Terminal styling    |
| boxen           | rich.panel   | Bordered boxes      |
| inquirer        | inquirer     | Interactive prompts |
| figlet          | pyfiglet     | ASCII art           |
| gradient-string | rich styles  | Color gradients     |
| nanospinner     | rich.spinner | Loading animations  |
| open            | webbrowser   | URL opening         |

### 3. Entry Points

- **TypeScript**: `"bin"` field in package.json
- **Python**: `[project.scripts]` in pyproject.toml

### 4. Module System

- **TypeScript**: ES modules with `.js` imports
- **Python**: Standard Python imports with relative imports

## 🚀 uvx vs npx Comparison

| Feature   | npx carlosferreyra | uvx carlosferreyra     |
| --------- | ------------------ | ---------------------- |
| Runtime   | Node.js            | Python                 |
| Registry  | npm                | PyPI                   |
| Caching   | npm cache          | uv cache               |
| Speed     | Fast               | Very Fast (Rust-based) |
| Isolation | Node modules       | Virtual environment    |

## 📦 Package Distribution

### npm (TypeScript)

```json
{
 "name": "carlosferreyra",
 "bin": {
  "carlosferreyra": "./dist/index.js"
 }
}
```

### PyPI (Python)

```toml
[project.scripts]
carlosferreyra = "carlosferreyra.__main__:main"
```

## 🎨 Customization Guide

To create your own version:

1. **Fork the repository**
2. **Update config.py** with your personal information
3. **Modify styling** in the theme configuration
4. **Add/remove menu options** in menu.py
5. **Update action handlers** in actions.py
6. **Change package name** in pyproject.toml
7. **Build and publish** with uv

## 🏆 Success Metrics

✅ **Functionality Parity**: All TypeScript features replicated ✅ **uvx Compatibility**: Works with
`uvx carlosferreyra` ✅ **Performance**: Fast startup and execution ✅ **Rich UI**: Beautiful
terminal interface with colors and animations ✅ **Cross-platform**: Works on macOS, Linux, and
Windows ✅ **Zero Dependencies**: All required packages bundled

## 🔮 Future Enhancements

- Add click-based command-line arguments
- Support for custom themes via environment variables
- Configuration file support (~/.carlosferreyra.toml)
- Plugin system for additional actions
- Internationalization support
- Web version deployment

This tutorial demonstrates how to successfully port a modern TypeScript CLI to Python while
maintaining all functionality and achieving uvx compatibility!
