use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct PipelineConfig {
    name: String,
    dockerfile_location: String,
    docker_image_tag: String,
    kubernetes_yaml_location: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide a subcommand: init or deploy");
        return;
    }

    match args[1].as_str() {
        "init" => init_pipeline(),
        "deploy" => deploy_pipeline(args.get(2)),
        _ => println!("Invalid subcommand. Please use 'init' or 'deploy'."),
    }
}

fn init_pipeline() {
    let mut pipeline = PipelineConfig {
        name: String::new(),
        dockerfile_location: String::new(),
        docker_image_tag: String::new(),
        kubernetes_yaml_location: String::new(),
    };

    println!("Enter the pipeline name:");
    pipeline.name = read_input();

    println!("Enter the Dockerfile location:");
    pipeline.dockerfile_location = read_input();

    println!("Enter the Docker image tag:");
    pipeline.docker_image_tag = read_input();

    println!("Enter the Kubernetes YAML location:");
    pipeline.kubernetes_yaml_location = read_input();

    // Serialize the pipeline config and save it to a file
    let config_json = serde_json::to_string(&pipeline).unwrap();
    fs::write(format!("{}.json", pipeline.name), config_json).unwrap();
}

fn deploy_pipeline(pipeline_name: Option<&String>) {
    let pipeline_name = match pipeline_name {
        Some(name) => name,
        None => {
            println!("Please provide the name of the pipeline to deploy");
            return;
        }
    };

    // Load the pipeline config from the saved file
    let filename = format!("{}.json", pipeline_name);
    if let Ok(config_json) = fs::read_to_string(&filename) {
        let pipeline: PipelineConfig = serde_json::from_str(&config_json).unwrap();
        println!("Deploying pipeline: {:?}", pipeline);
    } else {
        println!("Pipeline '{}' not found", pipeline_name);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}