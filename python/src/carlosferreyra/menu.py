"""Interactive menu system."""

import inquirer
from rich.console import Console

from .config import CONFIG, is_configured_url

console = Console()


def get_menu_choices():
    """Get the menu choices for the interactive prompt."""
    choices = []

    if is_configured_url(CONFIG.urls.email):
        choices.append(("📧  Send an Email", "email"))

    if is_configured_url(CONFIG.urls.resume):
        choices.append(("📥  View Resume", "view_resume"))

    if is_configured_url(CONFIG.urls.portfolio):
        choices.append(("🌐  Visit Portfolio", "view_portfolio"))

    if is_configured_url(CONFIG.urls.github):
        choices.append(("💻  View GitHub", "view_github"))

    if is_configured_url(CONFIG.urls.linkedin):
        choices.append(("💼  View LinkedIn", "view_linkedin"))

    if is_configured_url(CONFIG.urls.twitter):
        choices.append(("🐦  View Twitter", "view_twitter"))

    choices.append(("🚪  Exit", "quit"))
    return choices


def prompt_user() -> str:
    """Prompt user for menu selection."""
    choices = get_menu_choices()

    questions = [
        inquirer.List(
            "action",
            message="What would you like to do?",
            choices=choices,
            carousel=True,
        )
    ]

    try:
        answers = inquirer.prompt(questions)
        return answers["action"] if answers else "quit"
    except (KeyboardInterrupt, EOFError):
        console.print("\n[yellow]👋 Thanks for stopping by! Have a great day![/yellow]")
        return "quit"
