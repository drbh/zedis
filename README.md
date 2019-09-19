# ZEDIS <img src="https://img.shields.io/github/stars/drbh/zedis.svg" />

# Install

#### (MAC) Homebrew
```bash
brew tap drbh/zedis https://github.com/drbh/zedis
brew install zedis
```

#### (LINUX) `apt-get`
```
wget https://github.com/drbh/zedis/releases/download/v0.0001/zedis_1.0_amd64.deb 
sudo apt install ./zedis_1.0_amd64.deb
rm zedis_1.0_amd64.deb
```
# Run
``` 
zedis
```

#### 👍
```
# $ zedis
#	   _______ ___ ___ ___ 
#	  |_  / __|   \_ _/ __|
#	   / /| _|| |) | |\__ \ 
#	  /___|___|___/___|___/ 
#
```
A lightweight zero mq and sled based replacement for Redis. <90 LOC focused on a portable cross
language simple storage system. No need to run a full service.

Zedis is opinionated and limited. With ZEDIS you can only READ and WRITE key values pairs. All writes overwrite past values. 

Really Fast reads `< 250µs` for `~2 MB` json payload.   

Pretty Fast writes `< 18ms` for `~2 MB` json payload.  


# Commands

### Getting Values

```
GET key
```

Return the string values of the key. Returns `b'Error occurred: InvalidKey'` if key does not exist

### Setting Values 

```
SET key value
```
Insert a key value. If the key already exists the value will be overwritten.

### Removing Values

```
DEL key
```

Delete a key and it's value from zedis. This will also return the last known value of the key.

### Showing Keys

```
KEYS
```

This retrns a JSON format list of all of the keys. This is ineffiecent, it iterates through the whole DB and then concats the key names togther. Do not use if you have more than 1000 keys if you want instant results.

### Querying

```
PRE keyprefix
```

Returns all keys with that prefix. So "A" will return "Alpha", "Awesome"... this is case sensitive.


# Configuring

#### Setting Port

Now you can set the port/socket that `zedis` will be available on.

```
zedis 6677
```

If no port is specified zedis will fallback on `5555`

#### Read/Write from Python
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

# Dev

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
