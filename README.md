# uomi-makarcorex-agent

Minimal echo-style agent for the UOMI Finney testnet. Sources + release .wasm.

## On-chain
- **Network:** UOMI Finney testnet
- **Publish tx:** [`0xa15e…6d1f`](https://explorer.uomi.ai/tx/0xa15ea446fdbde00b5f7cf0525f13ad1e0f7a3c76972f8cd552e053b15644b6d1f)
- **Agent ID (tokenId):** `77`
- **Deposit / Fee:** `100 UOMI` / `0.044994737 UOMI`

## Params
- **Receiver:** `0x4E3DB810cB71C042d0D1659db8fD37a68cc563Ed`
- **Price / Validators / Blocks:** `0.05 UOMI` / `1` / `5`

## Build
- **Target:** `wasm32-wasip1`
- **Rust edition:** `2021`

## Releases
- Latest: https://github.com/MakarcoreX/uomi-makarcorex-agent/releases/tag/v0.1.0


---

## WASP Parameters (UOMI Finney)

- **Receiver**: `0x4E3DB810cB71C042d0D1659db8fD37a68cc563Ed`
- **Deposit / Fee**: `100 UOMI / 0.044994737 UOMI`
- **Validators / Max Blocks**: `1 / 5`

### Input Schema (JSON)
```json
{
  "type": "object",
  "properties": { "input": { "type": "string" } },
  "required": ["input"]
}
```

### Output Schema (JSON)
```json
{
  "type": "object",
  "properties": { "output": { "type": "string" } },
  "required": ["output"]
}
```

## How it works
Minimal echo agent: reads input bytes and writes the same bytes to output via runtime FFI.

## Build
```bash
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
# artifact: target/wasm32-wasip1/release/agent.wasm
```

## Examples
- `examples/input.json`
```json
{ "input": "hello uomi" }
```
- `examples/output.json`
```json
{ "output": "hello uomi" }
```

## Links
- Explorer tx (safeMint): `0xa15ea446fdbde00b5f7cf052f51ad1e0f7a3c76972f8cd552e053b15644b6d1f`
- Agent NFT tokenId: `77`
