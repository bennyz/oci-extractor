```shell
# Start local registry to avoid hitting rate limits
$ docker run -d -p 5000:5000 --restart=always --name registry docker://registry:2
$ docker pull alpine:latest
$ docker tag alpine:latest localhost:5000/my-alpine
$ docker push localhost:5000/my-alpine

# Sanity check
$ curl http://localhost:5000/v2/my-alpine/manifests/latest
```

