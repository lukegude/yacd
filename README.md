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

## Initializing a new pipeline
For example, if you want to deploy a pipeline called `my-pipeline` with the Dockerfile `my-pipeline/Dockerfile` and the Kubernetes YAML `my-pipeline/kubernetes.yaml`, you would run:
```bash
$ yacd init
Initializing a new pipeline...
Enter the pipeline name: my-pipeline # This will be the name of the deployment
Enter the Dockerfile location: /Users/username/random-folder/dockerfile-name # This will be the location of the Dockerfile (must use **FULL** path)
Enter the Docker image tag: my-registry.com/my-repo:my-pipeline # This is the docker build tag. It is the full image name and tag (e.g. my-registry.com/my-repo:latest)
Enter any special flags you want to pass to docker build (e.g. --platform linux/amd64, separate multiple flags with spaces): --platform linux/amd64 # This will be any special flags you want to pass to docker build (e.g. --platform linux/amd64)
Enter the Kubernetes YAML location: /Users/username/random-folder/kubernetes.yaml # This will be the location of the Kubernetes YAML (must use **FULL** path)
```
> A good way to get the full path of the Dockerfile and Kubernetes YAML is to find them in finder, then drag and drop them into the terminal. It can accept paths with or without quotes. The following are accepted formats: `'/Users/username/random-folder/Dockerfile'` and `/Users/username/random-folder/Dockerfile`.