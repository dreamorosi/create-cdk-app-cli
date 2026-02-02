use clap::Parser;
use include_assets::{include_dir, NamedArchive};
use spinoff::{spinners, Color, Spinner};

#[derive(Parser)]
#[command(version, about = "Create a new CDK project scaffold 🚀")]
struct Cli {
    /// The name of the CDK project (i.e. my-cdk-project)
    app_name: String,
    /// Optional flag to skip npm install
    #[clap(short, long)]
    no_install: bool,
}

struct AppNames {
    lower: String,
    pascal: String,
}

struct ProjectPaths {
    base_dir: String,
    bin_dir: String,
    lib_dir: String,
    test_dir: String,
    src_dir: String,
    events_dir: String,
}

struct ProjectFiles {
    package_json: String,
    cdk_json: String,
    vitest_config: Vec<u8>,
    git_ignore: Vec<u8>,
    bin_file: String,
    lib_file: String,
    src_file: Vec<u8>,
    events_file: Vec<u8>,
    cdk_context: Vec<u8>,
    test_file: Vec<u8>,
    ctx_file: Vec<u8>,
    tsconfig: String,
    biomeconfig: String,
}

const TSCONFIG_URL: &str = "https://gist.githubusercontent.com/dreamorosi/8785f2a8ae9e868be65de1a44018b936/raw/6e738f7abae160190a31b8d5bcdc5ff7af4c4cf6/tsconfig.json";
const BIOMECONFIG_URL: &str = "https://gist.githubusercontent.com/dreamorosi/3daec171ff98f2c921eb3a19459256dd/raw/88f6aad32ebcb787e3655bf5d68a5a1c9e1e52ae/biome.json";

async fn get_remote_config(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

fn validate_and_normalize_app_name(app_name: &str) -> Result<String, String> {
    if app_name.trim().is_empty() {
        return Err("App name cannot be empty".to_string());
    }
    
    // Normalize: trim whitespace and replace spaces with hyphens
    let normalized = app_name.trim().replace(' ', "-");
    
    if normalized.len() > 100 {
        return Err("App name is too long (max 100 characters)".to_string());
    }
    
    // Check for invalid characters (allow letters, numbers, hyphens, underscores)
    if !normalized.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err("App name can only contain letters, numbers, spaces, hyphens, and underscores".to_string());
    }
    
    // Check if directory already exists
    if std::path::Path::new(&format!("./{}", normalized)).exists() {
        return Err(format!("Directory '{}' already exists", normalized));
    }
    
    Ok(normalized)
}

fn generate_app_names(app_name: &str) -> AppNames {
    let sanitized_app_name = app_name.replace('-', "");
    let app_name_lower = sanitized_app_name.to_lowercase();
    let app_name_pascal = app_name
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            chars
                .next()
                .map(|c| c.to_uppercase().collect::<String>())
                .unwrap_or_default() + chars.as_str()
        })
        .collect::<String>();

    AppNames {
        lower: app_name_lower,
        pascal: app_name_pascal,
    }
}

fn create_project_paths(app_name: &str) -> ProjectPaths {
    let base_dir = format!("./{}", app_name);
    ProjectPaths {
        bin_dir: format!("{}/bin", base_dir),
        lib_dir: format!("{}/lib", base_dir),
        test_dir: format!("{}/test", base_dir),
        src_dir: format!("{}/src", base_dir),
        events_dir: format!("{}/events", base_dir),
        base_dir,
    }
}

fn create_directories(paths: &ProjectPaths) -> Result<(), String> {
    if let Err(e) = std::fs::create_dir_all(&paths.base_dir) {
        return Err(format!("Could not create project directory '{}': {}", paths.base_dir, e));
    }
    if let Err(e) = std::fs::create_dir_all(&paths.bin_dir) {
        return Err(format!("Could not create bin directory '{}': {}", paths.bin_dir, e));
    }
    if let Err(e) = std::fs::create_dir_all(&paths.lib_dir) {
        return Err(format!("Could not create lib directory '{}': {}", paths.lib_dir, e));
    }
    if let Err(e) = std::fs::create_dir_all(&paths.test_dir) {
        return Err(format!("Could not create test directory '{}': {}", paths.test_dir, e));
    }
    if let Err(e) = std::fs::create_dir_all(&paths.src_dir) {
        return Err(format!("Could not create src directory '{}': {}", paths.src_dir, e));
    }
    if let Err(e) = std::fs::create_dir_all(&paths.events_dir) {
        return Err(format!("Could not create events directory '{}': {}", paths.events_dir, e));
    }
    Ok(())
}

async fn load_template_files(app_names: &AppNames) -> Result<ProjectFiles, String> {
    let archive = NamedArchive::load(include_dir!("templates"));

    // Get template files (these should always exist - if not, it's a broken installation)
    let package_json = String::from_utf8_lossy(archive.get("package.json").expect("Installation corrupted: missing package.json template"))
        .replace("lowercase-name", &app_names.lower);
    let cdk_json = String::from_utf8_lossy(archive.get("cdk.json").expect("Installation corrupted: missing cdk.json template"))
        .replace("lowercase-name", &app_names.lower);
    let vitest_config = archive.get("vitest.config.ts").expect("Installation corrupted: missing vitest.config.ts template").to_vec();
    let git_ignore = archive.get(".gitignore").expect("Installation corrupted: missing .gitignore template").to_vec();
    let bin_file = String::from_utf8_lossy(archive.get("binfile.ts").expect("Installation corrupted: missing binfile.ts template"))
        .replace("lowercase-name", &app_names.lower)
        .replace("pascalcase-name", &app_names.pascal);
    let lib_file = String::from_utf8_lossy(archive.get("libfile.ts").expect("Installation corrupted: missing libfile.ts template"))
        .replace("pascalcase-name", &app_names.pascal);
    let src_file = archive.get("srcfile.ts").expect("Installation corrupted: missing srcfile.ts template").to_vec();
    let events_file = archive.get("payload.json").expect("Installation corrupted: missing payload.json template").to_vec();
    let cdk_context = archive.get("cdk.context.json").expect("Installation corrupted: missing cdk.context.json template").to_vec();
    let test_file = archive.get("testfile.ts").expect("Installation corrupted: missing testfile.ts template").to_vec();
    let ctx_file = archive.get("contextfile.ts").expect("Installation corrupted: missing contextfile.ts template").to_vec();

    // Fetch remote config files concurrently
    let (tsconfig_result, biome_result) = tokio::join!(
        get_remote_config(TSCONFIG_URL),
        get_remote_config(BIOMECONFIG_URL)
    );

    let tsconfig = match tsconfig_result {
        Ok(config) => config,
        Err(e) => {
            return Err(format!("Failed to fetch tsconfig.json from remote: {}. Please check your internet connection and try again.", e));
        }
    };
    let biomeconfig = match biome_result {
        Ok(config) => config,
        Err(e) => {
            return Err(format!("Failed to fetch biome.json from remote: {}. Please check your internet connection and try again.", e));
        }
    };

    Ok(ProjectFiles {
        package_json,
        cdk_json,
        vitest_config,
        git_ignore,
        bin_file,
        lib_file,
        src_file,
        events_file,
        cdk_context,
        test_file,
        ctx_file,
        tsconfig,
        biomeconfig,
    })
}

fn write_project_files(paths: &ProjectPaths, app_names: &AppNames, files: &ProjectFiles) -> Result<(), String> {
    if let Err(e) = std::fs::write(format!("{}/.gitignore", paths.base_dir), &files.git_ignore) {
        return Err(format!("Could not write .gitignore: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/biome.json", paths.base_dir), &files.biomeconfig) {
        return Err(format!("Could not write biome.json: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/cdk.json", paths.base_dir), &files.cdk_json) {
        return Err(format!("Could not write cdk.json: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/package.json", paths.base_dir), &files.package_json) {
        return Err(format!("Could not write package.json: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/tsconfig.json", paths.base_dir), &files.tsconfig) {
        return Err(format!("Could not write tsconfig.json: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/vitest.config.ts", paths.base_dir), &files.vitest_config) {
        return Err(format!("Could not write vitest.config.ts: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/{}.ts", paths.bin_dir, app_names.lower), &files.bin_file) {
        return Err(format!("Could not write bin file: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/{}.test.ts", paths.test_dir, app_names.lower), &files.test_file) {
        return Err(format!("Could not write test file: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/context.ts", paths.test_dir), &files.ctx_file) {
        return Err(format!("Could not write context.ts file: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/{}-stack.ts", paths.lib_dir, app_names.lower), &files.lib_file) {
        return Err(format!("Could not write lib file: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/index.ts", paths.src_dir), &files.src_file) {
        return Err(format!("Could not write src file: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/payload.json", paths.events_dir), &files.events_file) {
        return Err(format!("Could not write payload.json: {}", e));
    }
    if let Err(e) = std::fs::write(format!("{}/cdk.context.json", paths.base_dir), &files.cdk_context) {
        return Err(format!("Could not write cdk.context.json: {}", e));
    }
    Ok(())
}

fn install_dependencies(base_dir: &str) -> Result<(), String> {
    let mut spinner = Spinner::new(spinners::Dots, "Installing npm packages...", Color::Blue);
    
    let output = std::process::Command::new("npm")
        .arg("install")
        .current_dir(base_dir)
        .output();
    
    match output {
        Ok(result) => {
            if !result.status.success() {
                spinner.fail("npm install failed!");
                let mut error_msg = format!("npm install failed with exit code: {:?}", result.status.code());
                if !result.stderr.is_empty() {
                    error_msg.push_str(&format!("\nnpm error: {}", String::from_utf8_lossy(&result.stderr)));
                }
                error_msg.push_str("\nYou can try running 'npm install' manually in the project directory.");
                return Err(error_msg);
            }
            spinner.success("New CDK project created successfully!");
        }
        Err(e) => {
            spinner.fail("Failed to run npm install!");
            return Err(format!("Could not execute npm: {}. Make sure npm is installed and available in your PATH.", e));
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    // Validate and normalize app name
    let app_name = match validate_and_normalize_app_name(&args.app_name) {
        Ok(name) => name,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut spinner = Spinner::new(
        spinners::Dots,
        format!("Creating a new CDK project with name {}...", app_name),
        Color::Blue,
    );

    let app_names = generate_app_names(&app_name);
    let paths = create_project_paths(&app_name);

    if let Err(e) = create_directories(&paths) {
        spinner.fail("Failed to create directories!");
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    let files = match load_template_files(&app_names).await {
        Ok(files) => files,
        Err(e) => {
            spinner.fail("Failed to load template files!");
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = write_project_files(&paths, &app_names, &files) {
        spinner.fail("Failed to write project files!");
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    spinner.success("Scaffolding complete!");

    if args.no_install {
        println!("\nTo start working, run:\n\ncd {}", paths.base_dir);
        return;
    }

    if let Err(e) = install_dependencies(&paths.base_dir) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    println!("\nTo start working, run:\n\ncd {}", paths.base_dir);
}
