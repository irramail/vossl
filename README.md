# vossl
read from json rpc over http server and write to redis testing

# Test
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "openssl_version", "id":123, "params": ["asd", "wer"] }' 127.0.0.1:3030

for t in {1..100}; do curl -s  -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "openssl_version", "id":123, "params": ["qwe'$t'", "ver1"] }' $server_ip & ; done  >/dev/null

```json
{"jsonrpc": "2.0", "method": "new_track", "id":1, "params": ["445", "muscafe_fastpop_august2022_168.mp3 6%  D30%", "14:27:07", "08/12/22"] }
```

# Docker
```sh
# image id bc1a6df9b31e
# /etc/lighttpd/conf-enabled/stat.conf
# server.modules   += ( "mod_proxy" )
# $HTTP["url"] =~ "(^/$)" {
#  #proxy.server  = ( "" => ("" => ( "host" => "192.168.0.9", "port" => 3030 )))
#  proxy.server  = ( "" => ( ( "host" => "192.168.0.9", "port" => 3030 ), ( "host" => "192.168.0.9", "port" => 3031 ), ( "host" => "192.168.0.9", "port" => 3032 )))
# }
docker run --restart unless-stopped -dp 3030:3030 bc1a6df9b31e
docker run --restart unless-stopped -dp 3031:3030 bc1a6df9b31e
docker run --restart unless-stopped -dp 3032:3030 bc1a6df9b31e
```

# Diagnostics
```
# root only
tcpdump -A -s 0 'tcp port 80 and (((ip[2:2] - ((ip[0]&0xf)<<2)) - ((tcp[12]&0xf0)>>2)) != 0)' | grep \"64\"
```

Client side
```
while :; do  wget --no-check-certificate -O /dev/null --header="Content-Type: application/json" --post-data='{"jsonrpc": "2.0", "method": "new_track", "id":1, "params": [“1”, "'`date +%s`'_a.mp3 2%  D3%", "'`date +%T`'", "'`date +%D`'”] }' http://$server_ip ; sleep 5; done
```
