A simple command line app to get help on commands to run for any task that you need to carry out. Defaults to using Groq llama 3 70B model to generate the output.

E.g.
```
clai How do I get IP of a docker container
Gets the IP address of a running Docker container.
$ docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' container_name
```

It can also be run from a docker image. Either build on your own from the included Dockerfile or run from DockerHub

```
docker run -it --name clai -e OPENAI_API_KEY=$OPENAI_API_KEY -v "/var/run/docker.sock:/var/run/docker.sock" clai "How do i get IP of a docker container"
Gets the IP address of a running Docker container.
$ docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' container_name
```

Requirements:
Needs the following environment variables to be set before running the app.

- `OPENAI_API_KEY` - Mandatory - API key for your upstream AI provider.
- `OPENAI_API_BASE` - Optional - API base URL to be used for upstream AI provider. Defaults to Groq `https://api.groq.com/openai/v1`
- `OPENAI_MODEL` - Optional - Model to be used for inference. Defaults to `llama3-70b-8192`
