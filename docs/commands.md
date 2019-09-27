## Commands

### Get values

```
GET key
```

Return the string values of the key. Returns `b'Error occurred: InvalidKey'` if key does not exist

### Set values 

```
SET key value
```
Insert a key value. If the key already exists the value will be overwritten.

### Remove values

```
DEL key
```

Delete a key and it's value from zedis. This will also return the last known value of the key.

### Show keys

```
KEYS
```

This retrns a JSON format list of all of the keys. This is ineffiecent, it iterates through the whole DB and then concats the key names togther. Do not use if you have more than 1000 keys if you want instant results.

### Query

```
PRE keyprefix
```

### Flush

Force everything in memory to disk (useful before a graceful shutdown. rarely needed)

```
FLUSH
```

### Clear

Empty all values in DB

```
CLEAR
```