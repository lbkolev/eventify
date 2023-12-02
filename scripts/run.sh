RUSTLOG=trace cargo r -- run \
    --indexer.enabled \
    --node-url="wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27" \
    --database-url="postgres://postgres:password@localhost:5432/chainthru" \
    --src-block=18692253 --dst-block=18694253 \
    --criterias-json='[{"name":"UniswapV3Factory","events":["PoolCreated(address,address,uint24,int24,address)"],"addresses":["0x1F98431c8aD98523631AE4a59f267346ea31F984"]},{"name":"ERC20","events":["Transfer(address,address,uint256)","Approve(address,address,uint256)"],"addresses":["0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2","0x6B175474E89094C44Da98b954EedeAC495271d0F"]}]'
