# Helm chart

The chart is responsible for both:
- The Indexer
- The HTTP-Server

> It's advisable to check and adjust the available parameters in `values.yaml` accordingly before proceeding with a deployment

### Installation
```
$ helm upgrade --install chainthru chart/ \
  --namespace `namespace_name` --create-namespace
```
