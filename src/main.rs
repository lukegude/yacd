use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct PipelineConfig {
    name: String,
    dockerfile_location: String,
    docker_image_tag: String,
    kubernetes_yaml_location: String,
    docker_image_flags: String,
    push_repository: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide a subcommand: init or deploy");
        return;
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            println!("Usage:");
            println!("    yacd init            - Initialize a new pipeline");
            println!("    yacd deploy [name]   - Deploy a pipeline");
            println!("    yacd delete [name]   - Delete a pipeline");
        },
        "init" => init_pipeline(),
        "deploy" => deploy_pipeline(args.get(2)),
        "delete" => delete_pipeline(),
        _ => println!("Invalid subcommand. Please use 'init', 'deploy' or '--help' for usage information."),
    }
}

fn init_pipeline() {
    let mut pipelines = match fs::read_to_string("pipelines.json") {
        Ok(json) => serde_json::from_str(&json).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    };

    let mut pipeline = PipelineConfig {
        name: String::new(),
        dockerfile_location: String::new(),
        docker_image_tag: String::new(),
        kubernetes_yaml_location: String::new(),
        docker_image_flags: String::new(),
        push_repository: String::new(),
    };

    println!("Enter the pipeline name:");
    pipeline.name = read_input();

    println!("Enter the Dockerfile location:");
    let mut input = read_input();
    if input.starts_with("'") && input.ends_with("'") {
        input = input[1..input.len() - 1].to_string();
    }
    pipeline.dockerfile_location = input;

    println!("Enter the Docker image tag:");
    pipeline.docker_image_tag = read_input();

    println!("Enter any special flags you want to pass to docker build (e.g. --platform linux/amd64, separate multiple flags with spaces):");
    pipeline.docker_image_flags = read_input_multiple();

    println!("Enter the Docker push repository (e.g. my-registry.com/my-repo):");
    pipeline.push_repository = read_input();

    fn read_input_multiple() -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().replace("\n", " ")
    }

    println!("Enter the Kubernetes YAML location:");
    let mut input = read_input();
    if input.starts_with("'") && input.ends_with("'") {
        input = input[1..input.len() - 1].to_string();
    }
    pipeline.kubernetes_yaml_location = input;

    pipelines.push(pipeline);

    let config_json = serde_json::to_string(&pipelines).unwrap();
    fs::write("pipelines.json", config_json).unwrap();
}

fn delete_pipeline() {
    let pipeline_name = match std::env::args().nth(2) {
        Some(name) => name,
        None => {
            println!("Please provide the name of the pipeline to delete");
            read_input()
        }
    };

    let mut config_json = match fs::read_to_string("pipelines.json") {
        Ok(json) => json,
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                println!("pipelines.json not found. Please run 'yacd init' first.");
            } else {
                println!("Error reading pipelines.json: {}", error);
            }
            return;
        }
    };
    let mut pipelines: Vec<PipelineConfig> = match serde_json::from_str(&config_json) {
        Ok(pipelines) => pipelines,
        Err(error) => {
            println!("Error parsing pipelines.json: {}", error);
            return;
        }
    };
    if let Some(position) = pipelines.iter().position(|p| p.name == pipeline_name) {
        pipelines.remove(position);
        config_json = serde_json::to_string(&pipelines).unwrap();
        fs::write("pipelines.json", config_json).unwrap();
        println!("Pipeline '{}' deleted successfully", pipeline_name);
    } else {
        println!("Pipeline '{}' not found in pipelines.json", pipeline_name);
    }
}

fn deploy_pipeline(pipeline_name: Option<&String>) {
    let pipeline_name = match pipeline_name {
        Some(name) => name,
        None => {
            println!("Please provide the name of the pipeline to deploy");
            return;
        }
    };

    let config_json = match fs::read_to_string("pipelines.json") {
        Ok(json) => json,
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                println!("pipelines.json not found. Please run 'yacd init' first.");
            } else {
                println!("Error reading pipelines.json: {}", error);
            }
            return;
        }
    };
    let pipelines: Vec<PipelineConfig> = match serde_json::from_str(&config_json) {
        Ok(pipelines) => pipelines,
        Err(error) => {
            println!("Error parsing pipelines.json: {}", error);
            return;
        }
    };
    let pipeline = pipelines.iter().find(|p| p.name == *pipeline_name);
    if let Some(pipeline) = pipeline {
        let status = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(&pipeline.docker_image_tag)
            .arg("-f")
            .arg(&pipeline.dockerfile_location)
            .arg(".")
            .status()
            .expect("failed to run docker build");
        if !status.success() {
            println!("Docker build failed");
            return;
        }

        let status = Command::new("docker")
            .arg("push")
            .arg(&pipeline.push_repository)
            .status()
            .expect("failed to run docker push");
        if !status.success() {
            println!("Docker push failed");
            return;
        }

        let status = Command::new("kubectl")
            .arg("delete")
            .arg("-f")
            .arg(&pipeline.kubernetes_yaml_location)
            .status()
            .expect("failed to run kubectl delete");
        if !status.success() {
            println!("No deployment found");
        }
        let status = Command::new("kubectl")
            .arg("create")
            .arg("-f")
            .arg(&pipeline.kubernetes_yaml_location)
            .status()
            .expect("failed to run kubectl create");
        if !status.success() {
            println!("Deployment failed");
        }
    }

    println!("Pipeline '{}' not found", pipeline_name);
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
