# Clients

#### Python 3 Client

[pip page](https://pypi.org/project/pyzedis)

```python
import pyzedis

# connect to the zedis instance
c = pyzedis.ZedisClient()

# save a dict as JSON
c.zset_json("example", {"status": "great"})
# done

## now we get it back and deserialize it
c.zget_json("example")
# {'status': 'great'}

# we can see it returns a object
{'status': 'great'} == c.zget_json("example")
# True
```