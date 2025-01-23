GET {{hundun_base_url}}/v1/models
x-ai-provider: openai
Authorization: Bearer {{openai_api_key}}
[Options]
very-verbose: true
HTTP 200
[Asserts]
jsonpath "$.object" == "list"
jsonpath "$.data" count > 1
