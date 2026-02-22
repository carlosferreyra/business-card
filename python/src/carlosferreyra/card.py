"""Business card display with Rich formatting."""

from rich.console import Console
from rich.panel import Panel
from rich.text import Text

from .config import CONFIG, get_display_url, get_social_tag, is_configured_url

console = Console()


def create_profile_card() -> None:
    """Create and display the profile business card."""

    # Create card content
    card_lines = []

    # Name and title
    name_text = Text(CONFIG.personal_info.name, style="bold bright_magenta")
    title_text = Text(CONFIG.personal_info.title, style="white")
    card_lines.extend([name_text, title_text, ""])

    # Company and location
    if CONFIG.personal_info.company:
        company_text = Text("Working at ", style="white") + Text(
            CONFIG.personal_info.company, style="bright_yellow"
        )
        card_lines.append(company_text)

    location_text = Text(f"📍 {CONFIG.personal_info.location}", style="dim white")
    card_lines.extend([location_text, ""])

    # Skills
    skills_text = Text("⚡ Skills: ", style="white") + Text(
        " | ".join(CONFIG.personal_info.skills), style="bright_cyan"
    )
    card_lines.extend([skills_text, ""])

    # Social links
    if is_configured_url(CONFIG.urls.github):
        github_url = get_display_url(CONFIG.urls.github)
        github_tag = get_social_tag(CONFIG.urls.github)
        github_text = (
            Text("📦 GitHub:    ", style="white")
            + Text("{ ")
            + Text(github_url, style="bright_green")
            + Text(" }")
        )
        if github_tag:
            github_text += Text(f" {github_tag}", style="dim")
        card_lines.append(github_text)

    if is_configured_url(CONFIG.urls.linkedin):
        linkedin_url = get_display_url(CONFIG.urls.linkedin)
        linkedin_tag = get_social_tag(CONFIG.urls.linkedin)
        linkedin_text = (
            Text("💼 LinkedIn:  ", style="white")
            + Text("{ ")
            + Text(linkedin_url, style="bright_blue")
            + Text(" }")
        )
        if linkedin_tag:
            linkedin_text += Text(f" {linkedin_tag}", style="dim")
        card_lines.append(linkedin_text)

    if is_configured_url(CONFIG.urls.twitter):
        twitter_url = get_display_url(CONFIG.urls.twitter or "")
        twitter_tag = get_social_tag(CONFIG.urls.twitter or "")
        twitter_text = (
            Text("🐦 Twitter:   ", style="white")
            + Text("{ ")
            + Text(twitter_url, style="bright_cyan")
            + Text(" }")
        )
        if twitter_tag:
            twitter_text += Text(f" {twitter_tag}", style="dim")
        card_lines.append(twitter_text)

    if is_configured_url(CONFIG.urls.portfolio):
        web_text = (
            Text("🌐 Website:   ", style="white")
            + Text("{ ")
            + Text(get_display_url(CONFIG.urls.portfolio), style="bright_cyan")
            + Text(" }")
        )
        card_lines.append(web_text)

    card_lines.append("")

    # CLI command
    cli_text = (
        Text("📇 Card:      ", style="white")
        + Text("uvx ", style="bright_red")
        + Text("carlosferreyra", style="white")
    )
    card_lines.extend([cli_text, ""])

    # Call to action
    cta1 = Text(
        "🚀 Available for exciting opportunities and collaborations!",
        style="bold bright_red",
    )
    cta2 = Text(
        "💭 Let's connect and create something amazing together!",
        style="bold bright_cyan",
    )
    card_lines.extend([cta1, cta2])

    # Create the panel
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
        padding=(1, 2),
    )

    # Display the panel properly
    console.print(panel)
