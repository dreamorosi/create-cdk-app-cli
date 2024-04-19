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
const BIOMECONFIG_URL: &str = "https://gist.github.com/dreamorosi/3daec171ff98f2c921eb3a19459256dd/raw/3ab22a2b1e203eb0a9e55c9e341f06d8fa43c26d/biome.json";

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

    // Generate the app name in different formats
    let app_name_lower = args.app_name.to_lowercase();
    let app_name_pascal = format!(
        "{}{}",
        args.app_name.chars().nth(0).unwrap().to_uppercase(),
        &args.app_name[1..]
    );

    // Create references to the directories
    let base_dir = format!("./{}", app_name_lower);
    let bin_dir = format!("{}/bin", base_dir);
    let lib_dir = format!("{}/lib", base_dir);
    let test_dir = format!("{}/test", base_dir);
    let src_dir = format!("{}/src", base_dir);

    // Get the package.json file from the template & replace "app-name" with the actual app name
    let package_json = String::from_utf8_lossy(archive.get("package.json").unwrap())
        .replace("lowercase-name", &app_name_lower);
    // Get the cdk.json file from the template & replace "app-name" with the actual app name
    let cdk_json = String::from_utf8_lossy(archive.get("cdk.json").unwrap())
        .replace("lowercase-name", &app_name_lower);
    // Get the vitest.config.ts file from the template
    let vitest_config = archive.get("vitest.config.ts").unwrap();
    // Get the gitignore file from the template
    let git_ignore = archive.get(".gitignore").unwrap();
    // Get the bin file from the template & replace "app-name" with the actual app name
    let bin_file = String::from_utf8_lossy(archive.get("binfile.ts").unwrap())
        .replace("lowercase-name", &app_name_lower)
        .replace("pascalcase-name", &app_name_pascal);
    // Get the test file from the template & replace "app-name" with the actual app name
    let test_file = String::from_utf8_lossy(archive.get("testfile.ts").unwrap())
        .replace("lowercase-name", &app_name_lower)
        .replace("pascalcase-name", &app_name_pascal);
    // Get the lib file from the template & replace "app-name" with the actual app name
    let lib_file = String::from_utf8_lossy(archive.get("libfile.ts").unwrap())
        .replace("pascalcase-name", &app_name_pascal);
    // Get the src file from the template
    let src_file = archive.get("srcfile.ts").unwrap();

    // Get the tsconfig.json file from public gist
    let tsconfig = get_remote_config(TSCONFIG_URL).unwrap();
    // Get the biome.json file from public gist
    let biomeconfig = get_remote_config(BIOMECONFIG_URL).unwrap();

    // Create the directories
    std::fs::create_dir_all(&base_dir).unwrap();
    std::fs::create_dir_all(&bin_dir).unwrap();
    std::fs::create_dir_all(&lib_dir).unwrap();
    std::fs::create_dir_all(&test_dir).unwrap();
    std::fs::create_dir_all(&src_dir).unwrap();

    // write files
    std::fs::write(format!("{}/.gitignore", base_dir), git_ignore).unwrap();
    std::fs::write(format!("{}/biome.json", base_dir), biomeconfig).unwrap();
    std::fs::write(format!("{}/cdk.json", base_dir), cdk_json).unwrap();
    // TODO replace app name
    std::fs::write(format!("{}/package.json", base_dir), package_json).unwrap();
    std::fs::write(format!("{}/tsconfig.json", base_dir), tsconfig).unwrap();
    std::fs::write(format!("{}/vitest.config.ts", base_dir), vitest_config).unwrap();
    std::fs::write(format!("{}/{}.ts", bin_dir, app_name_lower), bin_file).unwrap();
    std::fs::write(
        format!("{}/{}.test.ts", test_dir, app_name_lower),
        test_file,
    )
    .unwrap();
    std::fs::write(format!("{}/{}-stack.ts", lib_dir, app_name_lower), lib_file).unwrap();
    std::fs::write(format!("{}/index.ts", src_dir), src_file).unwrap();

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
