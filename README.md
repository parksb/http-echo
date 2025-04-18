# http-echo

HTTP Echo is a small rust web server.

The default port is 5678, but this is configurable via the `--listen` flag:

```
http-echo --listen 0.0.0.0:8080 --text "hello world"
```

Then visit http://localhost:8080/ in your browser.
