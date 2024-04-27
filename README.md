# Current Way to Run

# deployed on ECS with Fargate

107.22.21.202:5050
1. cargo build --features "download-libtorch"
2. export .env through set method
3. cargo run --bin <bin_name>