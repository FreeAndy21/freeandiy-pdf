<!-- Modified-by: FreeANDIY (Free Andy Agency), 2026. New file, not part of upstream PdfCraft. -->
# FreeANDIY PDF: changes from upstream PdfCraft

This file lists what the FreeANDIY fork changes in PdfCraft (Apache-2.0 section 4(b)). The exact
changes are generated from `freeandiy/` (see `NOTICE`); `git diff upstream-main main` shows every
changed file.

## Unreleased

### Branding
- Product name **FreeANDIY PDF** everywhere users see it: window title, menus, About, Home,
  messages, all bundled translations, installers, desktop entries, the web page; the Producer of
  PDFs it creates.
- Application id `agency.freeandy.pdf` (was `ai.storyteller.pdfcraft`): macOS bundle id, Linux
  desktop/metainfo/Flatpak ids, so FreeANDIY builds install next to PdfCraft.
- Executables `freeandiy-pdf` and `freeandiy-pdf-cli`; the cargo packages keep their names.
- Portable mode keeps its data in `FreeANDIY PDF Data` beside the executable.
- New app icon set (the angel Andy, Free Andy Agency) and DMG background; the PDF document icon
  stays upstream's (Lucide-based, open licence).
- Light and Dark themes in FreeANDIY's paper-and-ink print look (square corners, no shadows,
  the stamp-rose accent).
- UI fonts IBM Plex Sans and IBM Plex Mono (OFL 1.1) on the desktop.

### Changed
- Help ▸ Check for updates asks this repository's releases (FreeAndy21/freeandiy-pdf) instead of
  upstream's; Help links point to the FreeANDIY website and this repository.

### Removed
- The ArtCraft name, wordmark and logos (`docs/brand/`, About, Home), as their terms require of
  forks.
- The ArtCraft Discord button and links (tab bar, Help menu, About, Home, CLI).
- Upstream's release and platform workflows (upstream's CI still runs the tests).

### Added
- About: attribution to upstream PdfCraft and the Free Andy Agency seal.
- Slovak user interface (menus and commands; the rest falls back to English for now).
- `NOTICE` section for FreeANDIY's assets and fonts; packages now ship `NOTICE`,
  `ATTRIBUTION.md`, the font licences and the brand terms.
