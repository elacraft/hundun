# OLLAMA_DEBUG=1 OLLAMA_HOST=0.0.0.0:11434 ollama run qwen2.5-coder:7b

# https://ollama.readthedocs.io/en/api/

# List Local Models
GET {{ hundun_base_url }}/api/tags
x-ai-provider: ollama
[Options]
very-verbose: true
repeat: 5
HTTP 200
[Asserts]
jsonpath "$.models" count > 0


POST {{hundun_base_url}}/api/show
Content-Type: application/json
x-ai-provider: ollama
{
  "model": "llama3.1"
}


# Embbedding
POST {{hundun_base_url}}/api/embed
Content-Type: application/json
x-ai-provider: ollama
[Options]
very-verbose: true
{
  "model": "nomic-embed-text",
  "input": "Why is the sky blue?"
}


# Load a Model, If an empty prompt is provided, the model will be loaded into memory.
POST http://localhost:11434/api/generate
Content-Type: application/json
x-ai-provider: ollama
{
  "model": "llama3.1"
}
HTTP 200
[Asserts]
jsonpath "$.model" == "llama3.1"


# List Running Models that are currently loaded into memory.
GET http://localhost:11434/api/ps
x-ai-provider: ollama
HTTP 200
[Asserts]
jsonpath "$.models" count > 0


POST http://localhost:11434/api/generate
Content-Type: application/json
x-ai-provider: ollama
[Options]
very-verbose: true
{
  "model": "llama3.1",
  "prompt": "Why is the sky blue?",
  "stream": false
}


POST http://localhost:11434/api/generate
Content-Type: application/json
x-ai-provider: ollama
[Options]
very-verbose: true
{
  "model": "qwen2.5-coder:7b-instruct",
  "prompt": "def compute_gcd(a, b):",
  "suffix": "    return result",
  "options": {
    "temperature": 1.0,
    "num_ctx": 4096
  },
  "stream": false
}

POST http://localhost:11434/api/generate
Content-Type: application/json
x-ai-provider: ollama
[Options]
very-verbose: true
{
  "model": "llama3.1",
  "prompt": "Why is the sky blue?",
  "stream": false,
  "options": {
    "num_keep": 5,
    "seed": 42,
    "num_predict": 100,
    "top_k": 20,
    "top_p": 0.9,
    "min_p": 0.0,
    "tfs_z": 0.5,
    "typical_p": 0.7,
    "repeat_last_n": 33,
    "temperature": 0.8,
    "repeat_penalty": 1.2,
    "presence_penalty": 1.5,
    "frequency_penalty": 1.0,
    "mirostat": 1,
    "mirostat_tau": 0.8,
    "mirostat_eta": 0.6,
    "penalize_newline": true,
    "stop": ["\n", "user:"],
    "numa": false,
    "num_ctx": 1024,
    "num_batch": 2,
    "num_gpu": 1,
    "main_gpu": 0,
    "low_vram": false,
    "vocab_only": false,
    "use_mmap": true,
    "use_mlock": false,
    "num_thread": 8
  }
}


POST http://localhost:11434/api/chat
Content-Type: application/json
x-ai-provider: ollama
[Options]
very-verbose: true
{
  "model": "qwen2.5-coder:7b",
  "store": true,
  "stream": false,
  "messages": [
    {"role": "user", "content": "write a rust axum server with graceful shutdown"}
  ]
}
HTTP 200
[Asserts]
jsonpath "$.model" == "qwen2.5-coder:7b"

POST http://localhost:11434/api/chat
Content-Type: application/json
x-ai-provider: ollama
[Options]
very-verbose: true
{
  "model": "llama3.1",
  "stream": false,
  "messages": [
    {
      "role": "user",
      "content": "why is the sky blue?"
    }
  ]
}

POST {{hundun_base_url}}/api/chat
Content-Type: application/json
x-ai-provider: ollama
[Options]
very-verbose: true
{
  "model": "qwen2.5-coder:7b",
  "store": true,
  "stream": false,
  "messages": [
    {"role": "user", "content": "write a rust axum server with graceful shutdown"}
  ]
}
HTTP 200
[Asserts]
jsonpath "$.model" == "qwen2.5-coder:7b"

POST {{hundun_base_url}}/api/chat
Content-Type: application/json
x-ai-provider: ollama
{
  "model": "llama3.1",
  "messages": [
    {
      "role": "user",
      "content": "What is the weather today in Paris?"
    }
  ],
  "stream": false,
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "get_current_weather",
        "description": "Get the current weather for a location",
        "parameters": {
          "type": "object",
          "properties": {
            "location": {
              "type": "string",
              "description": "The location to get the weather for, e.g. San Francisco, CA"
            },
            "format": {
              "type": "string",
              "description": "The format to return the weather in, e.g. 'celsius' or 'fahrenheit'",
              "enum": ["celsius", "fahrenheit"]
            }
          },
          "required": ["location", "format"]
        }
      }
    }
  ]
}
HTTP 200

POST {{hundun_base_url}}/api/chat
Content-Type: application/json
x-ai-provider: ollama
{
  "model": "llama3.1",
  "messages": [
    {
      "role": "user",
      "content": "What is the weather today in Paris?"
    }
  ],
  "stream": true,
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "get_current_weather",
        "description": "Get the current weather for a location",
        "parameters": {
          "type": "object",
          "properties": {
            "location": {
              "type": "string",
              "description": "The location to get the weather for, e.g. San Francisco, CA"
            },
            "format": {
              "type": "string",
              "description": "The format to return the weather in, e.g. 'celsius' or 'fahrenheit'",
              "enum": ["celsius", "fahrenheit"]
            }
          },
          "required": ["location", "format"]
        }
      }
    }
  ]
}
HTTP 200
