# mongo_emb 

## install
```shell
 pip install -U --index-url https://test.pypi.org/simple/mongo_emb
```

## embed mongodb implemented by rust with python api
```python
from mongo_emb import PyMongoEmb

db = PyMongoEmb("db23")
col = db['test']
data = [{"foo": "ba", "titi": "kpkp"}]
col.insert_one(data[0])
print(col.len())
col.update_one({"_id": '68f73fe7790a4a5d60d08dba'}, {"$set": {"xx": 12}}, upsert=True)
for i in col.find({}):
    print (i)
col.delete_one({'titi': 'kpkp'})

print(col.len())

```

## Current methods supported for collection
 - delete_one
 - delete_many
 - find
 - find_one
 - insert_many
 - insert_one
 - len
 - name
 - update_many (with upsert option)
 - update_one (with upsert option)
 - aggregate