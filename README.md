# migrust



Create migration:
```sh
echo '{"query": "ALTER TABLE x ADD COLUMN a int;", "project_hash": "12345"}' | http -f --json --print hb POST http://127.0.0.1:8080/migration/post
echo '{"query": "ALTER TABLE y ADD COLUMN b text;", "project_hash": "12345"}' | http -f --json --print hb POST http://127.0.0.1:8080/migration/post
echo '{"query": "ALTER TABLE z ADD COLUMN d int;", "project_hash": "12345"}' | http -f --json --print hb POST http://127.0.0.1:8080/migration/post
```

Res:
```json
HTTP/1.1 200 OK
content-length: 2
content-type: application/json
date: Sun, 08 Sep 2024 10:31:39 GMT

17

```


Retrieve migrations:
```sh
http http://127.0.0.1:8080/migration/get/12345

```

Res:
```json
[
   {
        "project_hash": "12345",
        "query": "ALTER TABLE x ADD COLUMN a int;",
        "ts_created": {
            "nanos_since_epoch": 964909000,
            "secs_since_epoch": 1725798623
        }
    },
    {
        "project_hash": "12345",
        "query": "ALTER TABLE y ADD COLUMN b text;",
        "ts_created": {
            "nanos_since_epoch": 133536000,
            "secs_since_epoch": 1725798624
        }
    },
    {
        "project_hash": "12345",
        "query": "ALTER TABLE z ADD COLUMN d int;",
        "ts_created": {
            "nanos_since_epoch": 303836000,
            "secs_since_epoch": 1725798624
        }
    }
]
```


