# cloudflare_worker_hello_rust
Hello Rust, but an HTTP endpoint on Cloudflare Workers to get a feel for the process of serverless functions with Rust. Cloudflare Workers runs Rust using WASM.

[Docs](https://developers.cloudflare.com/workers/runtime-apis/webassembly/rust/)
[Demo](https://cloudflare_worker_hello_rust.mackenly.workers.dev/)

## Development
In the project's directory run:
- `wrangler dev` - to run the project locally
- `wrangler publish` - to publish the project to Cloudflare Workers