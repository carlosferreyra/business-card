use anyhow::{Context, Result, anyhow};
use clap::{Parser, ValueEnum};
use inquire::Select;
use serde::Deserialize;

const CONFIG_JSON: &str = include_str!("../config.json");

#[derive(Parser, Debug)]
#[command(
    name = "carlosferreyra",
    version,
    about = "Interactive CLI business card"
)]
struct Cli {
    #[arg(index = 1, hide = true)]
    command: Option<String>,

    #[arg(long, value_enum, help = "Open a specific link directly")]
    open: Option<LinkTarget>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum LinkTarget {
    Email,
    Resume,
    Portfolio,
    Github,
    Linkedin,
    Twitter,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppConfig {
    personal_info: PersonalInfo,
    urls: Urls,
    #[allow(dead_code)]
    theme: Option<ThemeConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PersonalInfo {
    name: String,
    title: String,
    company: Option<String>,
    location: String,
    skills: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Urls {
    email: String,
    resume: String,
    portfolio: String,
    github: String,
    linkedin: String,
    twitter: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ThemeConfig {
    border_color: String,
    background_color: String,
    animation_speed: AnimationSpeed,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnimationSpeed {
    fast: u32,
    medium: u32,
    slow: u32,
}

#[derive(Clone)]
struct MenuItem {
    label: String,
    url: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("❌ {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Collapsed: Use '&&' with let-chains (Rust 1.82+)
    if let Some(command) = &cli.command
        && command != "carlosferreyra"
    {
        return Err(anyhow!(
            "Unknown command '{command}'. Use 'carlosferreyra' or omit it."
        ));
    }

    let config = load_config()?;

    if let Some(target) = cli.open {
        let url = url_for_target(&config.urls, target)
            .ok_or_else(|| anyhow!("Requested link is not configured"))?;
        open::that(url).context("Failed to open URL")?;
        println!("Opened {target:?}");
        return Ok(());
    }

    render_card(&config);
    interactive_menu(&config)
}

fn load_config() -> Result<AppConfig> {
    serde_json::from_str::<AppConfig>(CONFIG_JSON)
        .context("Failed to parse the embedded configuration.")
}

fn interactive_menu(config: &AppConfig) -> Result<()> {
    let mut options = configured_menu_items(config);
    options.push(MenuItem {
        label: "🚪 Exit".to_string(),
        url: String::new(),
    });

    loop {
        let labels: Vec<String> = options.iter().map(|item| item.label.clone()).collect();
        let selection = match Select::new("What would you like to do?", labels).prompt() {
            Ok(value) => value,
            Err(inquire::InquireError::OperationCanceled)
            | Err(inquire::InquireError::OperationInterrupted) => {
                println!("👋 Thanks for stopping by! Have a great day!");
                break;
            }
            Err(err) => return Err(anyhow!("Menu prompt failed: {err}")),
        };

        if selection.contains("Exit") {
            println!("👋 Thanks for stopping by! Have a great day!");
            break;
        }

        if let Some(item) = options.iter().find(|item| item.label == selection) {
            open::that(&item.url).context("Failed to open URL")?;
            println!("Opened {}", display_url(&item.url));
        }
    }

    Ok(())
}

fn configured_menu_items(config: &AppConfig) -> Vec<MenuItem> {
    let mut items = Vec::new();

    push_menu_item(&mut items, "📧 Send an Email", &config.urls.email);
    push_menu_item(&mut items, "📥 View Resume", &config.urls.resume);
    push_menu_item(&mut items, "🌐 Visit Portfolio", &config.urls.portfolio);
    push_menu_item(&mut items, "💻 View GitHub", &config.urls.github);
    push_menu_item(&mut items, "💼 View LinkedIn", &config.urls.linkedin);

    if let Some(twitter) = &config.urls.twitter {
        push_menu_item(&mut items, "🐦 View Twitter", twitter);
    }

    items
}

fn push_menu_item(items: &mut Vec<MenuItem>, label: &str, url: &str) {
    if is_configured(url) {
        items.push(MenuItem {
            label: label.to_string(),
            url: url.trim().to_string(),
        });
    }
}

fn render_card(config: &AppConfig) {
    println!();
    println!("╭───────────────────────────────────────────────────────────────╮");
    println!("│ {}", config.personal_info.name);
    println!("│ {}", config.personal_info.title);

    // Collapsed: Check Option and String content in one go
    if let Some(company) = &config.personal_info.company
        && !company.trim().is_empty()
    {
        println!("│ Working at {}", company.trim());
    }

    println!("│ 📍 {}", config.personal_info.location);
    println!("│ ⚡ Skills: {}", config.personal_info.skills.join(" | "));

    if is_configured(&config.urls.github) {
        println!("│ 📦 GitHub:   {}", display_url(&config.urls.github));
    }
    if is_configured(&config.urls.linkedin) {
        println!("│ 💼 LinkedIn: {}", display_url(&config.urls.linkedin));
    }

    // Collapsed
    if let Some(twitter) = &config.urls.twitter
        && is_configured(twitter)
    {
        println!("│ 🐦 Twitter:  {}", display_url(twitter));
    }

    if is_configured(&config.urls.portfolio) {
        println!("│ 🌐 Website:  {}", display_url(&config.urls.portfolio));
    }

    println!("│");
    println!("│ 🚀 Available for exciting opportunities and collaborations!");
    println!("╰───────────────────────────────────────────────────────────────╯");
    println!("💡 Tip: Use cmd/ctrl + click on links to open directly.");
    println!();
}

fn display_url(url: &str) -> String {
    url.trim()
        .trim_start_matches("mailto:")
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_string()
}

fn is_configured(url: &str) -> bool {
    !url.trim().is_empty()
}

fn url_for_target(urls: &Urls, target: LinkTarget) -> Option<&str> {
    let value = match target {
        LinkTarget::Email => Some(urls.email.as_str()),
        LinkTarget::Resume => Some(urls.resume.as_str()),
        LinkTarget::Portfolio => Some(urls.portfolio.as_str()),
        LinkTarget::Github => Some(urls.github.as_str()),
        LinkTarget::Linkedin => Some(urls.linkedin.as_str()),
        LinkTarget::Twitter => urls.twitter.as_deref(),
    }?;

    is_configured(value).then_some(value)
}
