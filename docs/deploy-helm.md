# Helm charts

The functionality is split between two different helm charts.
One is responsible for the server and its seamless scaling, the other one for the indexer

> It's advisable to check and adjust the available parameters in `values.yaml` accordingly before proceeding with a deployment

## Manual deployment
```
$ cd ./chainthru/charts/
```

### Server
```
helm install chainthru-server ./chainthru-server
```

### Indexer
```
helm install chainthru-indexer ./chainthru-indexer
```
