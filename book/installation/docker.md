# Docker

## Docker Hub

We are yet to publish our Docker images to Docker Hub. In the meantime, you can build the Docker image locally using the provided Dockerfile.

## Building the Docker Image

To build the Docker image, follow these steps:

1. Run the following command to build the Docker image:

    ```bash
    docker build . -t contower:local
    ```

2. After the build process is complete, you can run the Docker image using the following command:

    ```bash
    docker run contower:local contower --help
    ```

This will display the help message for Contower, indicating that the Docker image was built successfully.

## Using the Docker image

The functionality to fully utilize the Docker image is not available yet as we are still in the initial stages of development. As we continue to build and implement features, more detailed instructions on how to use the Docker image will be provided. Stay tuned for updates!
