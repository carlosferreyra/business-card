use anyhow::{Context, Result, anyhow};
use arboard::Clipboard;
use clap::Parser;
use inquire::Select;
use qrcode_generator::QrCodeEcc;
use serde::Deserialize;
use std::time::Duration;

const CONFIG_JSON: &str = include_str!("../config.json");

#[derive(Parser, Debug)]
#[command(
    name = "carlosferreyra",
    version,
    about = "Interactive CLI business card"
)]
struct Cli {
    #[arg(long)]
    open: Option<String>,
}

// --- Data Structures ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppConfig {
    personal_info: PersonalInfo,
    github_username: String,
    links: Vec<LinkConfig>,
}

#[derive(Debug, Deserialize, Clone)]
struct LinkConfig {
    id: String,
    label: String,
    url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PersonalInfo {
    name: String,
    title: String,
    company: Option<String>,
    location: String,
    skills: Vec<String>,
    email: String,
}

#[derive(Debug, Deserialize)]
struct GithubSearchResponse {
    items: Vec<GithubRepo>,
}

#[derive(Debug, Deserialize)]
struct GithubRepo {
    name: String,
    stargazers_count: u32,
    language: Option<String>,
    owner: GithubOwner,
}

#[derive(Debug, Deserialize)]
struct GithubOwner {
    login: String,
}

#[derive(Debug, Deserialize)]
struct GithubOrg {
    login: String,
}

// --- Main Logic ---

fn main() {
    if let Err(error) = run() {
        eprintln!("❌ {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;

    // Fetch orgs first, then fetch all relevant repos
    let orgs = fetch_user_orgs(&config.github_username);
    let repos = fetch_portfolio_repos(&config.github_username, &orgs);

    if let Some(target_id) = cli.open {
        let link = config
            .links
            .iter()
            .find(|l| l.id == target_id && !l.url.trim().is_empty())
            .ok_or_else(|| anyhow!("ID '{}' not found.", target_id))?;

        open::that(&link.url).context("Failed to open URL")?;
        return Ok(());
    }

    render_card(&config, &repos);
    println!(
        "👋 Hi! I'm {}, nice to meet you.",
        config.personal_info.name
    );
    interactive_menu(&config)
}

fn load_config() -> Result<AppConfig> {
    serde_json::from_str::<AppConfig>(CONFIG_JSON).context("Failed to parse embedded config.json")
}

// --- Integration Functions ---

fn fetch_user_orgs(username: &str) -> Vec<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("rust-cli-card")
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap_or_default();

    let url = format!("https://api.github.com/users/{}/orgs", username);

    client
        .get(url)
        .send()
        .ok()
        .and_then(|res| res.json::<Vec<GithubOrg>>().ok())
        .map(|orgs| orgs.into_iter().map(|o| o.login).collect())
        .unwrap_or_default()
}

fn fetch_portfolio_repos(username: &str, orgs: &[String]) -> Vec<GithubRepo> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("rust-cli-card")
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    // Build the query: topic:portfolio AND (user:me OR org:org1 OR org:org2...)
    let mut query = format!("topic:portfolio+user:{}", username);
    for org in orgs {
        query.push_str(&format!("+org:{}", org));
    }

    let url = format!(
        "https://api.github.com/search/repositories?q={}&sort=stars&order=desc",
        query
    );

    client
        .get(url)
        .send()
        .ok()
        .and_then(|res| res.json::<GithubSearchResponse>().ok())
        .map(|search_res| search_res.items)
        .unwrap_or_default()
}

// --- UI Components ---

fn show_qr(url: &str) {
    println!("\n📱 Scan to open on your mobile:");
    if let Ok(matrix) = qrcode_generator::to_matrix(url, QrCodeEcc::Low) {
        let margin = "  ";
        for i in (0..matrix.len()).step_by(2) {
            let mut line = String::from(margin);
            for j in 0..matrix[i].len() {
                let top = matrix[i][j];
                let bottom = if i + 1 < matrix.len() {
                    matrix[i + 1][j]
                } else {
                    false
                };
                match (top, bottom) {
                    (true, true) => line.push('█'),
                    (true, false) => line.push('▀'),
                    (false, true) => line.push('▄'),
                    (false, false) => line.push(' '),
                }
            }
            println!("{}", line);
        }
    }
    println!();
}

fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut cb = Clipboard::new().context("Clipboard access failed")?;
    cb.set_text(text.to_owned())
        .context("Copy operation failed")?;
    println!("✅ Copied to clipboard!");
    Ok(())
}

fn interactive_menu(config: &AppConfig) -> Result<()> {
    let mut options = vec![
        "📋 Copy Email".to_string(),
        "📱 View QR Code (LinkedIn)".to_string(),
    ];

    let links: Vec<LinkConfig> = config
        .links
        .iter()
        .filter(|l| !l.url.trim().is_empty())
        .cloned()
        .collect();

    for l in &links {
        options.push(l.label.clone());
    }
    options.push("🚪 Exit".to_string());

    loop {
        let selection = match Select::new("What would you like to do?", options.clone()).prompt() {
            Ok(v) => v,
            Err(_) => break,
        };

        if selection == "🚪 Exit" {
            println!("Have a great day!");
            break;
        }

        if selection == "📋 Copy Email" {
            let _ = copy_to_clipboard(&config.personal_info.email);
        } else if selection == "📱 View QR Code (LinkedIn)" {
            if let Some(li) = config.links.iter().find(|l| l.id == "linkedin") {
                show_qr(&li.url);
            }
        } else if let Some(link) = links.iter().find(|l| l.label == selection) {
            let _ = open::that(&link.url);
        }
    }
    Ok(())
}

fn render_card(config: &AppConfig, repos: &[GithubRepo]) {
    println!("\n╭───────────────────────────────────────────────────────────────╮");
    println!("│ {}", config.personal_info.name);
    println!("│ {}", config.personal_info.title);

    if let Some(company) = &config.personal_info.company
        && !company.trim().is_empty()
    {
        println!("│ 🏢 {}", company.trim());
    }

    println!("│ 📍 {}", config.personal_info.location);
    println!("│ ⚡ Skills: {}", config.personal_info.skills.join(" | "));

    if !repos.is_empty() {
        println!("├───────────────────────────────────────────────────────────────┤");
        println!("│ 🚀 PORTFOLIO PROJECTS (GitHub Top Stars):                     │");
        for repo in repos.iter().take(5) {
            // Mostramos el nombre del dueño si es una organización
            let display_name = if repo.owner.login != config.github_username {
                format!("{}/{}", repo.owner.login, repo.name)
            } else {
                repo.name.clone()
            };

            println!(
                "│ • {:<25} [⭐ {:>3}] [{:<8}]",
                display_name,
                repo.stargazers_count,
                repo.language.as_deref().unwrap_or("N/A")
            );
        }
    }

    println!("├───────────────────────────────────────────────────────────────┤");
    for link in config.links.iter().filter(|l| !l.url.trim().is_empty()) {
        let key = format!("{}:", link.id.to_uppercase());
        println!("│ {:<12} {}", key, display_url(&link.url));
    }
    println!("╰───────────────────────────────────────────────────────────────╯\n");
}

fn display_url(url: &str) -> String {
    url.trim()
        .replace("mailto:", "")
        .replace("https://", "")
        .replace("http://", "")
        .trim_end_matches('/')
        .to_string()
}
