<!-- Modified-by: FreeANDIY (Free Andy Agency), 2026. New file, not part of upstream VectorCraft. -->
# FreeANDIY Vector: changes from upstream VectorCraft

This file lists what the FreeANDIY fork changes in VectorCraft (Apache-2.0 section 4(b)). The
exact changes are generated from `freeandiy/` (see `NOTICE`); `git diff upstream-main main`
shows every changed file.

## Unreleased

### Branding
- Product name **FreeANDIY Vector** everywhere users see it: window title, menus, About, Home,
  messages, all bundled translations, installers, desktop entries, the web page.
- Application id `agency.freeandy.vector` (was `ai.storyteller.vectorcraft`): macOS bundle id,
  Linux desktop/metainfo/Flatpak ids, so FreeANDIY builds install next to VectorCraft.
- Settings directory `FreeANDIY Vector` (macOS, Windows) / `freeandiy-vector` (Linux).
- Executables `freeandiy-vector` and `freeandiy-vector-cli`; the cargo packages and the
  `.vectorcraft` document format keep their names.
- New app icon set (the angel Andy, Free Andy Agency) and DMG background.
- The four UI brightness levels get FreeANDIY's flat print look (square corners, no shadows, the
  stamp-rose accent); Dark is ink and paper, Light paper and ink.
- UI fonts IBM Plex Sans and IBM Plex Mono (OFL 1.1) on the desktop.

### Removed
- The ArtCraft name, wordmark and logos (`docs/brand/`), as their terms require of forks.
- The ArtCraft Discord button and links (app bar, Help menu, About, Home) and the ArtCraft
  website links; Help now points to the FreeANDIY website and this repository.
- Upstream's release, nightly and platform workflows.

### Added
- About: attribution to upstream VectorCraft and the Free Andy Agency seal.
- Slovak user interface (menus; the rest falls back to English for now).
- `NOTICE` section and `ASSETS.md` rows for FreeANDIY's assets and fonts; packages now ship
  `NOTICE`, `ASSETS.md`, the font licences and the brand terms.
