use bamboo_api::HttpClient;
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Parser)]
#[command(about = "List all Bamboo projects and plans")]
struct Args {
    /// Bamboo base URL (e.g. https://bamboo.example.com)
    #[arg(long, env = "BAMBOO_URL")]
    url: String,

    /// Bamboo API token
    #[arg(long, env = "BAMBOO_TOKEN")]
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("reqwest=debug,reqwest_tracing=debug,info")),
        )
        .init();

    let client = HttpClient::new()
        .with_base_url(args.url)
        .with_api_key(args.token);

    // List all projects
    let projects_response = client.get_projects(Some("projects.project"), None).await?;
    let projects = projects_response
        .projects
        .and_then(|p| p.project)
        .unwrap_or_default();

    println!("=== Projects ===");
    if projects.is_empty() {
        println!("No projects found.");
    } else {
        println!("{:<12} {}", "KEY", "NAME");
        println!("{}", "-".repeat(50));
        for project in &projects {
            let key = project.key.as_deref().unwrap_or("-");
            let name = project.name.as_deref().unwrap_or("-");
            println!("{:<12} {}", key, name);
        }
        println!("{} project(s) total.\n", projects.len());
    }

    // List all plans
    let plans_response = client.get_all_plan_list(Some("plans.plan")).await?;
    let plans = plans_response
        .plans
        .and_then(|p| p.plan)
        .unwrap_or_default();

    println!("=== Plans ===");
    if plans.is_empty() {
        println!("No plans found.");
        return Ok(());
    }

    println!("{:<20} {}", "KEY", "NAME");
    println!("{}", "-".repeat(60));
    for plan in &plans {
        let key = match (plan.project_key.as_deref(), plan.short_key.as_deref()) {
            (Some(p), Some(k)) => format!("{}-{}", p, k),
            _ => "-".to_string(),
        };
        let name = plan.build_name.as_deref().unwrap_or("-");
        println!("{:<20} {}", key, name);
    }
    println!("{} plan(s) total.", plans.len());

    Ok(())
}
