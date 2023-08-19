gen:
	openapi-generator generate -g rust \
  -i https://thornode.ninerealms.com/thorchain/doc/openapi.yaml \
  -o thorchain-rust \
  --additional-properties=useSingleRequestParameter=true
