```bash
curl http://127.0.0.1:3333/
```
```bash
curl http://127.0.0.1:3333/todo/1
```
```bash
curl -X POST http://127.0.0.1:3333/todo
```
```bash
curl -X PUT http://127.0.0.1:3333/todo/1
```
```bash
curl -X DELETE http://127.0.0.1:3333/todo/1
```
```bash
curl -X POST http://127.0.0.1:3333/signUp -H "Content-Type: application/json" -d '{"name": "example_user", "password": "example_password"}'
```
```bash
curl -X POST http://127.0.0.1:3333/signIn -H "Content-Type: application/json" -d '{"name": "example_user", "password": "example_password"}'
```



