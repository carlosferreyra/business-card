use anyhow::{Context, Result, anyhow};
use arboard::Clipboard;
use clap::Parser;
use inquire::Select;
use qrcode_generator::QrCodeEcc;
use serde::Deserialize;
use std::collections::HashMap;

const RESUME_JSON: &str = include_str!("../resume.json");
const CLI_LABEL: &str = "cli";

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

#[derive(Debug)]
struct AppConfig {
    personal_info: PersonalInfo,
    links: Vec<LinkConfig>,
    projects: Vec<ProjectConfig>,
}

#[derive(Debug, Clone)]
struct LinkConfig {
    id: String,
    label: String,
    url: String,
}

#[derive(Debug)]
struct PersonalInfo {
    name: String,
    title: String,
    summary: String,
    location: String,
    skills: Vec<String>,
    email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResumeCatalog {
    profiles: HashMap<String, ResumeProfile>,
    personal_info: ResumePersonalInfo,
    links: Vec<ResumeLink>,
    skills: Vec<ResumeSkill>,
    projects: Vec<ResumeProject>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResumeProfile {
    title: String,
    summary: String,
    personal_info: Option<ResumePersonalInfoOverride>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResumePersonalInfo {
    name: String,
    email: String,
    location: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResumePersonalInfoOverride {
    name: Option<String>,
    email: Option<String>,
    location: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResumeLink {
    id: String,
    label: String,
    url: String,
    labels: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ResumeSkill {
    items: Vec<String>,
    labels: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ResumeProject {
    name: String,
    description: String,
    url: String,
    labels: Vec<String>,
}

#[derive(Debug, Clone)]
struct ProjectConfig {
    name: String,
    description: String,
    url: String,
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

    if let Some(target_id) = cli.open {
        let link = config
            .links
            .iter()
            .find(|l| l.id == target_id && !l.url.trim().is_empty())
            .ok_or_else(|| anyhow!("ID '{}' not found.", target_id))?;

        open::that(&link.url).context("Failed to open URL")?;
        return Ok(());
    }

    render_card(&config);
    println!(
        "👋 Hi! I'm {}, nice to meet you.",
        config.personal_info.name
    );
    interactive_menu(&config)
}

fn load_config() -> Result<AppConfig> {
    let catalog = serde_json::from_str::<ResumeCatalog>(RESUME_JSON)
        .context("Failed to parse embedded resume.json")?;
    AppConfig::from_resume(catalog, CLI_LABEL)
}

impl AppConfig {
    fn from_resume(catalog: ResumeCatalog, label: &str) -> Result<Self> {
        let profile = catalog
            .profiles
            .get(label)
            .with_context(|| format!("Missing '{}' profile in resume.json", label))?;

        let overrides = profile.personal_info.as_ref();
        let personal_info = PersonalInfo {
            name: overrides
                .and_then(|p| p.name.clone())
                .unwrap_or(catalog.personal_info.name),
            title: profile.title.clone(),
            summary: profile.summary.clone(),
            location: overrides
                .and_then(|p| p.location.clone())
                .unwrap_or(catalog.personal_info.location),
            skills: flatten_labeled_skills(catalog.skills, label),
            email: overrides
                .and_then(|p| p.email.clone())
                .unwrap_or(catalog.personal_info.email),
        };

        let links = catalog
            .links
            .into_iter()
            .filter(|link| has_label(&link.labels, label))
            .map(|link| LinkConfig {
                id: link.id,
                label: link.label,
                url: link.url,
            })
            .collect();

        let projects = catalog
            .projects
            .into_iter()
            .filter(|project| has_label(&project.labels, label))
            .map(|project| ProjectConfig {
                name: project.name,
                description: project.description,
                url: project.url,
            })
            .collect();

        Ok(Self {
            personal_info,
            links,
            projects,
        })
    }
}

fn flatten_labeled_skills(skills: Vec<ResumeSkill>, label: &str) -> Vec<String> {
    let mut flattened = Vec::new();

    for skill in skills {
        if !has_label(&skill.labels, label) {
            continue;
        }

        for item in skill.items {
            if !flattened.contains(&item) {
                flattened.push(item);
            }
        }
    }

    flattened
}

fn has_label(labels: &[String], label: &str) -> bool {
    labels.iter().any(|candidate| candidate == label)
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

fn render_card(config: &AppConfig) {
    println!("\n╭───────────────────────────────────────────────────────────────╮");
    println!("│ {}", config.personal_info.name);
    println!("│ {}", config.personal_info.title);

    println!("│ 📍 {}", config.personal_info.location);
    println!("│ {}", config.personal_info.summary);
    if !config.personal_info.skills.is_empty() {
        println!("│ ⚡ Skills: {}", config.personal_info.skills.join(" | "));
    }

    if !config.projects.is_empty() {
        println!("├───────────────────────────────────────────────────────────────┤");
        println!("│ 🚀 PORTFOLIO PROJECTS:                                       │");
        for project in config.projects.iter().take(5) {
            println!("│ • {}", project.name);
            println!("│   {}", project.description);
            println!("│   {}", display_url(&project.url));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_catalog() -> ResumeCatalog {
        serde_json::from_str(
            r#"{
                "profiles": {
                    "cli": {
                        "slug": "cli",
                        "title": "CLI Engineer",
                        "summary": "Builds useful command-line tools.",
                        "personalInfo": {
                            "location": "United States"
                        },
                        "targets": ["business-card"]
                    },
                    "default": {
                        "slug": "default",
                        "title": "Software Engineer",
                        "summary": "Default summary.",
                        "targets": ["pdf"]
                    }
                },
                "personalInfo": {
                    "name": "Carlos Ferreyra",
                    "email": "carlos@example.com",
                    "location": "Argentina"
                },
                "githubUsername": "carlosferreyra",
                "links": [
                    {
                        "id": "github",
                        "label": "GitHub",
                        "url": "https://github.com/carlosferreyra",
                        "labels": ["cli"]
                    },
                    {
                        "id": "email",
                        "label": "Email",
                        "url": "mailto:carlos@example.com",
                        "labels": ["default", "cli"]
                    },
                    {
                        "id": "hidden",
                        "label": "Hidden",
                        "url": "https://example.com",
                        "labels": ["default"]
                    }
                ],
                "skills": [
                    {
                        "category": "Languages",
                        "items": ["Rust", "Python"],
                        "labels": ["cli"]
                    },
                    {
                        "category": "Other",
                        "items": ["Cooking"],
                        "labels": ["default"]
                    },
                    {
                        "category": "Automation",
                        "items": ["Python", "GitHub Actions"],
                        "labels": ["cli"]
                    }
                ],
                "experience": [],
                "education": [],
                "certifications": [],
                "projects": [
                    {
                        "name": "Business Card",
                        "description": "Interactive CLI card",
                        "url": "https://github.com/carlosferreyra/business-card",
                        "labels": ["cli"]
                    },
                    {
                        "name": "Hidden Project",
                        "description": "Not for CLI",
                        "url": "https://example.com",
                        "labels": ["default"]
                    }
                ]
            }"#,
        )
        .unwrap()
    }

    #[test]
    fn builds_cli_view_model_from_cli_profile() {
        let config = AppConfig::from_resume(sample_catalog(), CLI_LABEL).unwrap();

        assert_eq!(config.personal_info.name, "Carlos Ferreyra");
        assert_eq!(config.personal_info.title, "CLI Engineer");
        assert_eq!(
            config.personal_info.summary,
            "Builds useful command-line tools."
        );
        assert_eq!(config.personal_info.location, "United States");
        assert_eq!(config.personal_info.email, "carlos@example.com");
    }

    #[test]
    fn filters_cli_labeled_links_skills_and_projects() {
        let config = AppConfig::from_resume(sample_catalog(), CLI_LABEL).unwrap();

        assert_eq!(
            config
                .links
                .iter()
                .map(|link| link.id.as_str())
                .collect::<Vec<_>>(),
            vec!["github", "email"]
        );
        assert!(config.links.iter().all(|link| link.id != "hidden"));
        assert_eq!(
            config.personal_info.skills,
            vec!["Rust", "Python", "GitHub Actions"]
        );
        assert_eq!(config.projects.len(), 1);
        assert_eq!(config.projects[0].name, "Business Card");
    }

    #[test]
    fn open_lookup_only_sees_filtered_non_empty_links() {
        let config = AppConfig::from_resume(sample_catalog(), CLI_LABEL).unwrap();

        let github = config
            .links
            .iter()
            .find(|link| link.id == "github" && !link.url.trim().is_empty());
        let hidden = config
            .links
            .iter()
            .find(|link| link.id == "hidden" && !link.url.trim().is_empty());

        assert!(github.is_some());
        assert!(hidden.is_none());
    }
}
