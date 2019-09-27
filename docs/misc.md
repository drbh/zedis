# Misc

## 🦀 `01010` Developer Stuff

### Check alive
```bash
nc -v -z -w 5 localhost 5555
# found 0 associations
# found 1 connections:
#      1:	flags=82<CONNECTED,PREFERRED>
# 	outif lo0
# 	src 127.0.0.1 port 52227
# 	dst 127.0.0.1 port 5555
# 	rank info not available
# 	TCP aux info available

# Connection to localhost port 5555 [tcp/personal-agent] succeeded!
```

#### Build it
Clone, build and add a symlink so you can access `zedis` in the cli
```bash
git clone https://github.com/drbh/zedis.git && cd zedis
cargo build --release && sh install.sh 
```

#### Packaging

```
fpm -f -s dir -t deb -n zedis target/release/zedis=/usr/local/
```



### Raw Client Implementation

This is an example of whats under the hood of the `pyzedis` package above. This pattern can be reused in any lang!

#### Python 3 client example
```python
import zmq
import json

port = "5555"

context = zmq.Context()
socket = context.socket(zmq.REQ)
socket.connect("tcp://localhost:%s" % port)

socket.send_string("SET david richard blyn holtz");socket.recv()
# 'done.'

socket._string("GET david");socket.recv()
# 'richard blyn holtz'

jsonblob = json.dumps({"example": "you can store seralized JSON"})
socket.send_string("SET js "+jsonblob);socket.recv()
# 'done.'

socket.send_string("GET js")
json.loads(socket.recv())
# {u'exmple': u'lorem ipsum...'}

socket.send_string("DEL js");socket.recv();socket.recv()
# b'2 yo yo'

## ADDING A BUNCH OF KEYS
for x in range(0, 100):
    socket.send_string("SET "+str(x)+" "+str(x)+" yo yo")
    socket.recv()

socket.send_string("PRE 1");socket.recv()
# b'["1", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19"]'


socket.send_string("KEYS");socket.recv()
# b'["0", "1", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22"]'

```
