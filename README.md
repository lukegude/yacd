# YACD - Yet Another Continuous Deployment

## Description
YACD is a powerful tool for automating the deployment process in your development workflow.

## Installation
To install YACD, follow these steps:

1. Install Rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Clone the repository.
3. `sh install.sh`

## Usage
To use YACD, run the following command:
  - `yacd init` - Create a deployment. specify docker file, tags, flags, push tag, kubernetes YAML
  - `yacd deploy <deployment name>` - This will deploy the pipeline
  - `yacd delete <deployment name>` - This will delete the saved deployment
  - `yacd list` - This will list all saved deployments
