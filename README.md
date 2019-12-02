# vossl
read from json rpc over http server and write to redis testing

# Test
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "openssl_version", "id":123, "params": ["asd", "wer"] }' 127.0.0.1:3030

for t in {1..100}; do curl -s  -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "openssl_version", "id":123, "params": ["qwe'$t'", "ver1"] }' 178.169.70.179 & ; done  >/dev/null