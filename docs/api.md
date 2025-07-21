# kafql-store REST API

All endpoints are served over HTTP (default port 3000).

## Endpoints

### List all records
**GET /records**

Response:
```json
[
  {
    "key": "user-123",
    "value": {"name": "Alice"},
    "timestamp": 1710000000000,
    "topic": "users",
    "partition": 0,
    "offset": 42
  },
  ...
]
```

---

### Get a record by key
**GET /records/:key**

Response (200):
```json
{
  "key": "user-123",
  "value": {"name": "Alice"},
  "timestamp": 1710000000000,
  "topic": "users",
  "partition": 0,
  "offset": 42
}
```
Response (404):
```json
null
```

---

### Create or update a record
**POST /records**

Request:
```json
{
  "key": "user-123",
  "value": {"name": "Alice"},
  "topic": "users",
  "partition": 0
}
```
Response (201):
```json
{
  "key": "user-123",
  "value": {"name": "Alice"},
  "timestamp": 1710000000000,
  "topic": "users",
  "partition": 0,
  "offset": -1
}
```

---

### Delete a record (tombstone)
**DELETE /records/:key**

Response (204):
No content

Response (404):
```json
null
``` 