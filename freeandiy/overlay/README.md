<!-- Modified-by: FreeANDIY (Free Andy Agency), 2026. Replaces upstream VectorCraft's README. -->
<p align="center">
  <img src="assets/app-icon/freeandiy-vector-1024.png" width="128" alt="FreeANDIY Vector icon">
</p>

<h1 align="center">FreeANDIY Vector</h1>

<p align="center">
  <b>A free, open-source vector illustration app. Native Rust, runs on macOS, Windows, Linux and in the browser.</b><br>
  Part of <a href="https://freeandiy-web.vercel.app">FreeANDIY</a>, free creative and office apps by Free Andy Agency.
</p>

---

## About

FreeANDIY Vector is a vector illustration app: pen and shape tools, layers, artboards, type, live
effects, symbols, gradients and meshes, with SVG, PDF, EPS and AI-compatible import and export.
It is a native egui app (no webview), and the web version is the same Rust code compiled to
WebAssembly.

Every action is a command, so the app can be driven from a CLI (`freeandiy-vector-cli`), a JSON
control channel or an MCP server.

**FreeANDIY Vector is based on the open-source project
[VectorCraft](https://github.com/storytold/vectorcraft) (© ArtCraft Team and contributors,
MIT/Apache-2.0). FreeANDIY is not affiliated with the ArtCraft team.** The editor itself is their
work; FreeANDIY adds its own branding, packaging and translations, and follows upstream closely.

## Download

Builds for macOS, Windows and Linux are published on the
[Releases](https://github.com/FreeAndy21/freeandiy-vector/releases) page as
`freeandiy-vector-<version>-<os>-<arch>.*`.

## Build from source

Requires Rust (see `rust-version` in `Cargo.toml`).

```sh
cargo run --release -p vectorcraft                         # desktop app (binary: freeandiy-vector)
cargo run --release -p vectorcraft-cli -- --help           # CLI (binary: freeandiy-vector-cli)
cd apps/vectorcraft-web && trunk build --release           # web build (WebAssembly)
```

The crate names keep upstream's `vectorcraft-*`, and documents keep the `.vectorcraft` format, so
upstream changes merge cleanly and files open in both. The developer documentation in `docs/` is
upstream's and still says VectorCraft.

## How this fork is made

Everything FreeANDIY changes lives in [`freeandiy/`](freeandiy/): the brand definition
(`branding.toml`, `app.toml`), assets, a handful of small patches and text rules. All other files
are regenerated from upstream VectorCraft by `freeandiy/scripts/regenerate.sh`, so updates from
upstream never conflict. See [`freeandiy/docs/upstream-sync.md`](freeandiy/docs/upstream-sync.md).

Issues with FreeANDIY builds go to
[our tracker](https://github.com/FreeAndy21/freeandiy-vector/issues), not to upstream.

## License

The code is licensed, like upstream, under either of [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE) at your option. See [`NOTICE`](NOTICE) and [`ASSETS.md`](ASSETS.md)
for third-party material.

The FreeANDIY and Free Andy Agency names, the angel Andy, logos and app icons are © Free Andy
Agency and **not** covered by the code license; see
[`freeandiy/assets/brand/LICENSE-brand.txt`](freeandiy/assets/brand/LICENSE-brand.txt).

<sub>Adobe and Illustrator are trademarks or registered trademarks of Adobe Inc. in the United
States and/or other countries. FreeANDIY Vector is an independent, open-source project and is not
affiliated with, sponsored by or endorsed by Adobe Inc.; these names are used only to describe the
workflows it is compatible with.</sub>
