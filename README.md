
[![pipeline status](https://gitlab.com/jeremymtan/jjta-cloud-final/badges/master/pipeline.svg)](https://gitlab.com/jeremymtan/jjta-cloud-final/-/commits/master)

# Movie Recommendation Web App

## Goal

The goal of this project is to deploy an open-source model via a web service developed in Rust, aimed at constructing a movie recommendation system. Users have the flexibility to input either a movie description directly into a text box or provide keywords, facilitating autofill functionality for the system. Following user input, the system provides recommendations for the optimal movie choice, enriched with relevant movie information such as descriptions, release years, directors, ratings, and more.

Throughout the project lifecycle, our objectives include containerizing the service for seamless deployment with AWS ECS and Fargate. We achieve this by implementing a CI/CD pipeline and streamlining the build, test, and deployment processes. Furthermore, we focus on enhancing the system's robustness by incorporating monitoring, metrics collection, and comprehensive documentation.

For a detailed explanation of the project's functionality and architecture, please refer to the provided YouTube video.

## Web App Functionality (Deliveries)

Example: user input full description (input and output)

![Screenshot_2024-04-27_at_12.30.50_PM](/uploads/723f6c8017beaeb8a081fc594e2fdca0/Screenshot_2024-04-27_at_12.30.50_PM.png)

![Screenshot_2024-04-27_at_12.32.20_PM](/uploads/350189faca2db29dd955fd5c55bf2be2/Screenshot_2024-04-27_at_12.32.20_PM.png)


Example: user input keywords (input and output)

![Screenshot_2024-04-27_at_12.31.57_PM](/uploads/bcf3549b9486314bd0fb743e003bf286/Screenshot_2024-04-27_at_12.31.57_PM.png)

![Screenshot_2024-04-27_at_12.32.05_PM](/uploads/a588206f197329a8afd201b3a38863b3/Screenshot_2024-04-27_at_12.32.05_PM.png)


## Architecture

![4ff0ebdef50aef930c2715eb3a8deb9f](/uploads/b8f2c06a9a0d9e0678b5c0c8c9a40654/4ff0ebdef50aef930c2715eb3a8deb9f.JPG)

## Dataset Background

The [IMDb Movie Dataset](https://www.kaggle.com/datasets/rajugc/imdb-movies-dataset-based-on-genre) : All Movies by Genre is a comprehensive collection of movies listed on IMDb, organized by genre. This dataset encompasses various movie attributes, including movie ID, name, release year, genre, rating, description, director, star, votes, and gross box office.

**Data Source:**

Sourced from IMDb, the dataset provides extensive movie information, updated regularly with new titles and details.

**Purpose:**

This dataset offers valuable insights into movie trends, genre preferences, and industry dynamics. Researchers, film enthusiasts, and data scientists can explore trends over time, analyze genre characteristics, and build predictive models or recommendation systems based on user preferences.

**Data Cleaning:**

- Merged movies from 16 categories and created a new "Genre" column.
- Filtered movies released between 2020 and 2023.
- Retained relevant columns: movie ID, name, year, runtime, genre, rating, description, director, star, and votes.
The cleaned dataset comprises 33,402 rows and is stored as "combined_data.csv".

## Dependencies Installation and Setup 

### Initialize Cargo Lambda
1. Use the command `cargo lambda new <repository name>` to initialize the project.

###  Sign Up for Qdrant and Cohere
1. Register for Qdrant to create a vector database cluster. Obtain the API key and cluster URL.
2. Sign up for Cohere and obtain an API key for generating embeddings.

### Environment Variables 
1. Store Qdrant and Cohere API keys in a `.env` file.
2. Export access key using the following commands:
    ```bash
    set -a
    source .env
    set +a
    ```

### Add Dependencies in the Cargo.toml file 
Run `cargo build --features "download-libtorch"` to include dependencies.

### Modify Content
Create a `combined_data.jsonl` file containing columns from the movie data CSV. This file will be used for further processing and analysis.

## LLM Model 

Incorporating a robust language model to comprehend movie descriptions from the CSV data:

### Embedding Descriptions:
- Leveraging the Large Language Model (LLM) to transform movie descriptions into vector embeddings. These embeddings capture the nuanced content of each movie's description, enabling sophisticated analysis.

### Model Used: Rust-Bert
Utilizing the Sentence Embeddings pipeline from Rust-Bert to:
- Compute sentence/text embeddings for semantic comparison (e.g., cosine-similarity).
- Facilitate tasks such as semantic textual similarity, semantic search, or paraphrase mining.
- Pretrained models available on the Hugging Face Hub can be utilized. However, it's necessary to convert them using the script `utils/convert_model.py` beforehand. Refer to `tests/sentence_embeddings.rs` for example implementations.

### Testing the Model Locally:
- Two test scripts are provided for local testing:
  - `src/test_model.rs`: Reads descriptions from a CSV file, encodes them into sentence embeddings using the Rust BERT library, and prints out the embeddings.
  - `src/test_model_1encode.rs`: Reads descriptions from a CSV file, encodes the first description into sentence embeddings, and prints out the embedding for the first description.
- These scripts define a struct `MovieRecord` and provide a function `read_descriptions` to load descriptions from a CSV file.
- The main function sets up the model for sentence embeddings using Rust BERT, computes embeddings, and prints the results.

### Cargo.toml Configuration:
- Two bins are temporarily added in the `cargo.toml` file for running the test scripts separately (`test_model.rs` and `test_model_1encode.rs`). These bins are commented out later for easier deployment debugging.

### Local Testing:
Execute `cargo run --bin <bin_name>` to test the model locally.

## Quadrant Vector Databse 

### Data Ingestion

1. **Modify `src/setup_final.rs`**:
   - This script reads movie records from a JSONL file, computes sentence embeddings for movie descriptions using Rust BERT, and stores the embeddings along with other movie data in a Qdrant database.
  
   **Stages and Usage:**
   1. **Reading Movie Records from JSONL:**
      - Defines a struct `MovieRecord` representing movie data.
      - Provides a function `read_movies` to read movie records from a JSONL file and return a vector of `MovieRecord` structs.
   2. **Storing Data in Qdrant:**
      - Defines structs `QdrantPayload` and `MovieRecordFields` to structure data for Qdrant.
      - Provides an asynchronous function `store_in_qdrant` to store movie data and embeddings in Qdrant.
      - Connects to the Qdrant database using environment variables for URI and API key.
      - Checks if the collection exists and creates it if not.
      - Constructs a payload containing movie data and embeddings.
      - Upserts the payload into the Qdrant collection.
   3. **Main Function:**
      - Uses `tokio::main` attribute to run the asynchronous main function.
      - Loads movie records from the JSONL file using `read_movies`.
      - Sets up the model for sentence embeddings using Rust BERT.
      - Iterates over each movie record:
        - Computes embeddings for the movie description.
        - Converts the embeddings to a vector of `f32`.
        - Utilizes `upsert_points_batch_blocking` to store movie data and embeddings in Qdrant in batches of 200 for efficiency.

2. **Add Bins in the `cargo.toml` File**:
   - As this is a separate package, bins are temporarily added to run the file separately from `main.rs` (these are commented out later for easier deployment debugging).

3. **Run the Bin for Data Ingestion**:
   - Execute `cargo run --bin qdrant combined_data.jsonl`.

4. **Open Qdrant Dashboard**:
   - Check the test collection in the Qdrant dashboard. The Qdrant cluster should now have a collection filled with associated embeddings or "points".

![Screenshot_2024-04-27_at_12.36.34_PM](/uploads/83738728f8ad88b2cf551ab37cf2fbb6/Screenshot_2024-04-27_at_12.36.34_PM.png)


## Generating Descriptions & Recommendations

Then we utilizes Actix Web, Qdrant Client, Reqwest, Serde JSON, and other libraries for building a movie recommendation system. Here's a breakdown of its components:

1. **Modify `src/main.rs`**:

### Imports and Constants
- Imports necessary libraries and defines constants such as `SEARCH_LIMIT` for search result limits.

### Data Structures
- Defines structures like `MovieDescription`, `CohereResponse`, `Meta`, `ApiVersion`, and `BilledUnits` for deserializing JSON data.

### Endpoint Handlers
- **`movie`**: Handles POST requests to `/movie`. It takes a movie description as input, performs inference using Cohere API, retrieves movie details from Qdrant, and returns an HTML page with movie information.
- **`generate_text`**: Handles POST requests to `/generate-text`. It takes a JSON object containing a movie description, generates text using the OpenLLaMA LLM model, and returns the generated text.

### Inference Functions
- **`infer`**: Sends a prompt to the Cohere API to obtain text embeddings, then searches for similar points in a Qdrant index based on the embeddings.
- **`infer2`**: Uses the LLM (OpenLLaMA) to generate text based on the provided prompt. It loads the model from the specified path and generates text using the model.

### Main Function
- Starts the Actix Web server, defining routes for `/movie` and `/generate-text`.

2. **Add Bins in the `cargo.toml` File**:
   - As this is a separate package, bins are temporarily added to run the file separately (these are commented out later for easier deployment debugging).

3. **Run the Bin for Local Webstie Testing**:
   - Execute `cargo run --bin website`.
 
## Frontend Implementation 

The frontend of the Movie Recommendation system is designed to offer users a user-friendly web interface for interacting with the recommendation system. Below is an overview of the frontend implementation:

- **Index.html:** This file serves as the front page of the web application and provides a visually appealing layout and user interface for users. It includes:
    - A hero section with a background image and a form for users to input movie descriptions.
    - Text area for users to type in descriptions of the movies they want to watch.
    - Buttons for submitting the description to get a movie recommendation and for generating a description.

- **Features:**
    - **Input Box:** A text box where users can input descriptions of the movies they are interested in.
    - **Buttons:** 
        - "Get Recommendation": Submits the description to receive a movie recommendation based on the input.
        - "Generate Description": Generates a description of the movie based on the input provided.

- **Why Choose Us Section:** This section provides additional information about the movie recommendation system, highlighting its key features and benefits:
    - "Missed a movie?": Encourages users to describe the movies they're interested in rather than trying to remember specific details.
    - "Describe, not remember": Emphasizes the ease of use by allowing users to simply describe the movie they want to watch.
    - "Movie Discovered": Assures users that they can find movies released between 2020 and 2023 using the recommendation system.

- **Result Webpage:** The movie recommendation is generated in the backend by the `main.rs` file. After users submit their movie descriptions, the backend processes the input and generates relevant movie recommendations based on the provided description. The result webpage then displays these recommendations to the user.

- **Script:** The JavaScript script included in the HTML file enables dynamic functionality for generating text based on user input. It asynchronously sends the description entered by the user to the server and updates the text area with the generated movie description.

This frontend implementation aims to provide an intuitive and seamless user experience, allowing users to easily input their preferences and receive relevant movie recommendations.

## Dockerization and ECS Deployment

### Steps to Deploy:

1. **Create VPC Group:** Establish a VPC group to expose the Rust Actix website to the public, enabling external access.

    ![Screenshot_2024-04-27_at_1.29.11_PM](/uploads/6cba3967c4ff1b02e540149a0bbfb77d/Screenshot_2024-04-27_at_1.29.11_PM.png)

2. **Deploy with ECS via Fargate:**
   - **Cluster Creation:** Initialize an ECS cluster and specify deployment with Fargate to ensure efficient resource allocation.

    ![Screenshot_2024-04-27_at_1.28.20_PM](/uploads/39c58a9a74b7eb016f96f4b8c083dccf/Screenshot_2024-04-27_at_1.28.20_PM.png)

   - **Task Definition:** Define a task to run within the cluster. 
     - Specify memory and CPU requirements for optimal model inference speed.
     - Set up health check commands to ensure container health.
     - Configure metrics monitoring for memory and CPU usage.
     - Link the public Docker image to the task for deployment.
     - Add a security group rule allowing inbound traffic on port 50505 from any IP address to enable public access.
     - Specify system architecture and environment variables required by the container.

     ![Screenshot_2024-04-27_at_1.30.20_PM](/uploads/83a4c214c3ef6f350b1aa355b0a4ac1a/Screenshot_2024-04-27_at_1.30.20_PM.png)


   - **Execution Role:** Utilize a default execution role for the task as no interaction with other services is necessary.

3. **Deployment and Continuous Deployment:**
   - After creating the task definition, deploy on demand by deploying the task definition to the cluster, creating a task within the cluster.
   - Continuous deployment is enabled as the task is linked to the latest Docker image pushed in the Docker registry or ECR private registry.

![Screenshot_2024-04-27_at_1.23.09_PM](/uploads/217faaff8edd21ccdead8947880218b0/Screenshot_2024-04-27_at_1.23.09_PM.png)


## Monitoring and Metrics

![Screenshot_2024-04-27_at_1.16.29_PM](/uploads/692dce06e8eae6566738ec874562a110/Screenshot_2024-04-27_at_1.16.29_PM.png)

## Continuous Integration and Continuous Deployment (CI/CD) Pipeline

This project utilizes a GitLab CI/CD pipeline to automate the build, test, and deployment processes. The CI/CD pipeline is configured using a `.gitlab-ci.yml` file, and the project includes a `Makefile` to streamline various development tasks.

### `.gitlab-ci.yml` Configuration
The CI/CD pipeline consists of two main stages:

1. **Build and Test (`build-test`)**:
   - This stage runs on an Ubuntu image and sets up necessary dependencies including Rust and Zig.   
  - It then executes various tasks such as linting, formatting, testing, and building the project using the provided Makefile.
   - Rust and Zig versions are specified.

2. **Deployment (`deploy`)**:
   - Deployment is not handled directly in the CI/CD pipeline due to potential time constraints on GitLab.
   - However, a deployment script is provided and can be executed on Gitlab to deploy the Rust binary with Docker in Docker image using AWS ECR to push to private registry or alternatively to a public Docker registry.

### `Makefile` Targets
The `Makefile` contains several targets to facilitate development and automation:

- **`rust-version`**: Displays the versions of various Rust command-line utilities.
- **`format`**: Formats Rust code using `cargo fmt`.
- **`lint`**: Lints Rust code using `cargo clippy`.
- **`test`**: Executes Rust tests using `cargo test`.
- **`watch`**: Watches for changes and rebuilds the project using `cargo run`.
- **`build`**: Builds the project for release using `cargo build`.
- **`deploy`**: placeholder as CI/CD can take of it, but “docker build -t clodufinal .” will build image.

### Notes:
- The CI/CD pipeline is primarily focused on building and testing the project.
- Deployment is handled separately due to potential limitations on GitLab CI/CD duration.
- The provided Makefile targets help automate common development tasks and streamline the development process.

## Conclusion 

The Movie Recommendation Web App project represents a sophisticated deployment of an open-source model through a Rust-based web service, aiming to construct an efficient movie recommendation system. Leveraging Actix Web, Qdrant, Cohere, and open-source LLM models such as Rust-Bert and OpenLLaMA, the system offers users a seamless interface for inputting movie descriptions or keywords to receive personalized recommendations. Containerization with Docker and deployment on ECS ensure scalability, while a CI/CD pipeline streamlines development. With a user-centric frontend and robust backend architecture, this project showcases the fusion of advanced technologies and meticulous design to deliver a powerful solution in the realm of movie recommendation systems.