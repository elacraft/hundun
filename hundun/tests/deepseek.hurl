GET {{hundun_base_url}}/models
Authorization: Bearer {{deepseek_api_key}}
x-ai-provider: deepseek
[Options]
very-verbose: true
HTTP 200
[Asserts]
jsonpath "$.object" == "list"
jsonpath "$.data" count > 1
