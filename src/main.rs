use clap::Parser;
use include_assets::{include_dir, NamedArchive};
use spinoff::{spinners, Color, Spinner};

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about = "Create a new CDK project scaffold 🚀")]
struct Cli {
    /// The name of the CDK project (i.e. my-cdk-project)
    app_name: String,
    /// Optional flag to skip npm install
    #[clap(short, long)]
    no_install: bool,
}

#[tokio::main]
async fn get_remote_config(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(String::from(resp))
}

const TSCONFIG_URL: &str = "https://gist.github.com/dreamorosi/8785f2a8ae9e868be65de1a44018b936/raw/e7b125699fc752ac3cdfbe5a8528e7c00eb3220d/tsconfig.json";
const BIOMECONFIG_URL: &str = "https://gist.github.com/dreamorosi/3daec171ff98f2c921eb3a19459256dd/raw/076ca7b945de6577de0b467d6a269d8ea160561d/biome.json";

fn main() {
    // Parse the CLI arguments
    let args = Cli::parse();

    let mut spinner = Spinner::new(
        spinners::Dots,
        format!("Creating a new CDK project with name {}...", args.app_name),
        Color::Blue,
    );

    // Load the template files
    let archive = NamedArchive::load(include_dir!("templates"));

    // Remove dashes from the app name
    let sanitized_app_name = args.app_name.replace('-', "");

    // Generate the app name in different formats
    let app_name_lower = sanitized_app_name.to_lowercase();
    // Generate the PascalCase app name by splitting on dashes and capitalizing each part
    let app_name_pascal = args.app_name
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            chars
                .next()
                .map(|c| c.to_uppercase().collect::<String>())
                .unwrap_or_default() + chars.as_str()
        })
        .collect::<String>();

    // Use the original app name with dashes for the base directory
    let base_dir = format!("./{}", args.app_name);

    // Keep other directories relative to the base directory
    let bin_dir = format!("{}/bin", base_dir);
    let lib_dir = format!("{}/lib", base_dir);
    let test_dir = format!("{}/test", base_dir);
    let src_dir = format!("{}/src", base_dir);
    let events_dir = format!("{}/events", base_dir);

    // Get the package.json file from the template & replace "app-name" with the actual app name
    let package_json = String::from_utf8_lossy(archive.get("package.json").expect("Failed to get package.json"))
        .replace("lowercase-name", &app_name_lower);
    // Get the cdk.json file from the template & replace "app-name" with the actual app name
    let cdk_json = String::from_utf8_lossy(archive.get("cdk.json").expect("Failed to get cdk.json"))
        .replace("lowercase-name", &app_name_lower);
    // Get the vitest.config.ts file from the template
    let vitest_config = archive.get("vitest.config.ts").expect("Failed to get vitest.config.ts");
    // Get the gitignore file from the template
    let git_ignore = archive.get(".gitignore").expect("Failed to get .gitignore");
    // Get the bin file from the template & replace "app-name" with the actual app name
    let bin_file = String::from_utf8_lossy(archive.get("binfile.ts").expect("Failed to get binfile.ts"))
        .replace("lowercase-name", &app_name_lower)
        .replace("pascalcase-name", &app_name_pascal);// Get the lib file from the template & replace "app-name" with the actual app name
    let lib_file = String::from_utf8_lossy(archive.get("libfile.ts").expect("Failed to get libfile.ts"))
        .replace("pascalcase-name", &app_name_pascal);
    // Get the src file from the template
    let src_file = archive.get("srcfile.ts").expect("Failed to get srcfile.ts");

    // Get the tsconfig.json file from public gist
    let tsconfig = get_remote_config(TSCONFIG_URL).expect("Failed to fetch tsconfig.json");
    // Get the biome.json file from public gist
    let biomeconfig = get_remote_config(BIOMECONFIG_URL).expect("Failed to fetch biome.json");
    // Get payload.json file from the template
    let events_file = archive.get("payload.json").expect("Failed to get payload.json");
    // Get cdk.context.json file from the template
    let cdk_context = archive.get("cdk.context.json").expect("Failed to get cdk.context.json");
    // Get the test file from the template
    let test_file = archive.get("testfile.ts").expect("Failed to get testfile.ts");
    // Get the context file from the template
    let ctx_file = archive.get("contextfile.ts").expect("Failed to get contextfile.ts");
    
    // Create the directories
    std::fs::create_dir_all(&base_dir).expect("Failed to create base directory");
    std::fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    std::fs::create_dir_all(&lib_dir).expect("Failed to create lib directory");
    std::fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    std::fs::create_dir_all(&src_dir).expect("Failed to create src directory");
    std::fs::create_dir_all(&events_dir).expect("Failed to create events directory");

    // write files
    std::fs::write(format!("{}/.gitignore", base_dir), git_ignore).expect("Failed to write .gitignore");
    std::fs::write(format!("{}/biome.json", base_dir), biomeconfig).expect("Failed to write biome.json");
    std::fs::write(format!("{}/cdk.json", base_dir), cdk_json).expect("Failed to write cdk.json");
    // TODO replace app name
    std::fs::write(format!("{}/package.json", base_dir), package_json).expect("Failed to write package.json");
    std::fs::write(format!("{}/tsconfig.json", base_dir), tsconfig).expect("Failed to write tsconfig.json");
    std::fs::write(format!("{}/vitest.config.ts", base_dir), vitest_config).expect("Failed to write vitest.config.ts");
    std::fs::write(format!("{}/{}.ts", bin_dir, app_name_lower), bin_file).expect("Failed to write bin file");
    std::fs::write(
        format!("{}/{}.test.ts", test_dir, app_name_lower),
        test_file,
    ).expect("Failed to write test file");
    std::fs::write(format!("{}/context.ts", test_dir), ctx_file).expect("Failed to write context.ts file");
    std::fs::write(format!("{}/{}-stack.ts", lib_dir, app_name_lower), lib_file).expect("Failed to write lib file");
    std::fs::write(format!("{}/index.ts", src_dir), src_file).expect("Failed to write src file");
    std::fs::write(format!("{}/payload.json", events_dir), events_file).expect("Failed to write payload.json");
    std::fs::write(format!("{}/cdk.context.json", base_dir), cdk_context).expect("Failed to write cdk.context.json");

    spinner.success("Scaffolding complete!");

    if args.no_install {
        println!("\nTo start working, run:\n\ncd {}", base_dir);
        return;
    }

    // Create a spinner
    let mut spinner = Spinner::new(spinners::Dots, "Installing npm packages...", Color::Blue);

    // Run npm install in the new project directory
    std::process::Command::new("npm")
        .arg("install")
        .current_dir(&base_dir)
        .output()
        .expect("Failed to run npm install");

    spinner.success("New CDK project created successfully!");
    println!("\nTo start working, run:\n\ncd {}", base_dir);
}
