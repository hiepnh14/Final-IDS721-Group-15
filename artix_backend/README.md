# Final Project
Daniel, Emily, Yilin, Hiep

# Description

## How to run the Project backend
1. Download the bloom-560m-q5_1-ggjt.bin file from https://huggingface.co/rustformers/bloom-ggml/blob/main/bloom-560m-q5_1-ggjt.bin, and put it under the src directory before running the project.

2. Run backend: go to `artix_backend` folder:
```command
cargo run
```
3. To test the backend locally: 
```command
curl -X POST http://127.0.0.1:8080/ -H "Content-Type: application/json" -d '{"message": "Hello, A"}'
```

The result should be a Json, with `response` as key: `{"response":"Hello, Aunt Sally."}% `


## Deploy front end on Docker:
1. Create Docker image:
```
docker build -t rs-model-frontend .   
```
2. Deploy front end on Docker:
```
docker run -p 3000:3000 rs-model-frontend
```