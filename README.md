# KumiDiagrams

KumiDiagrams is a desktop application for creating and organizing Mermaid diagrams. It combines SvelteKit, Tauri, and a local Rust backend, so diagram editing feels like a native app while your diagrams remain plain text files inside project folders.

## Features

- **Projects as folders** - each project is a separate directory with `.mmd` diagrams and optional `.kumi-diagrams.json` configuration.
- **File tree management** - create, rename, and delete folders and diagrams directly from the app.
- **Mermaid editor** - the preview renders automatically while you type, and syntax errors are shown immediately.
- **Editor autocomplete** - the app detects the diagram type and suggests Mermaid identifiers, keywords, and directions. Currently supported for flowcharts, sequence diagrams, state diagrams, and ER diagrams.
- **Appearance configuration** - global, project-level, and per-diagram settings for theme, font, background, Mermaid look, and connection curves.
- **PNG export** - save a diagram as a PNG file or copy the rendered image to the clipboard.
- **Optional local AI** - Ollama integration can generate initial Mermaid source from a description without sending data to external services.

## Requirements

To run the project from source, you need:

- macOS 15 or newer for the target application bundle,
- Node.js,
- pnpm,
- Rust and Cargo,
- system dependencies required by Tauri 2,
- optionally Ollama if you want to generate diagrams with a local model.

## Install From DMG

1. Download `KumiDiagrams.dmg` from the project release.
2. If macOS blocks the downloaded app or the file comes from outside the App Store, clear quarantine attributes:

   ```bash
   xattr -c ~/Downloads/KumiDiagrams.dmg
   ```

   For a specific downloaded file name, use that name instead of `KumiDiagrams.dmg`, for example:

   ```bash
   xattr -c ~/Downloads/file-name
   ```

3. Open the `.dmg` file.
4. Move `KumiDiagrams.app` to the `Applications` folder.
5. Launch the app from Launchpad or from the `Applications` folder.

## Manual Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd kumi-diagrams
   ```

2. Install frontend dependencies:

   ```bash
   pnpm install
   ```

3. Run the app in development mode:

   ```bash
   pnpm tauri dev
   ```

4. Build the production bundle:

   ```bash
   pnpm tauri build
   ```

   Tauri artifacts will be available in:

   ```text
   src-tauri/target/release/bundle/
   ```

## Useful Commands

```bash
pnpm dev          # frontend only, Vite/SvelteKit
pnpm check        # TypeScript and Svelte checks
pnpm test:unit    # unit tests
pnpm build        # frontend build
pnpm tauri dev    # desktop app in development mode
pnpm tauri build  # desktop app and installers
```

## Ollama AI Configuration

AI is optional. To use it:

1. Start a local Ollama server at `localhost:11434`.
2. Pull the model you want to use in Ollama.
3. Open the app settings.
4. Enable Ollama and select a model from the list.
5. When creating a new diagram, enter a description and the app will generate initial Mermaid source.

## Project Data

KumiDiagrams stores diagrams as `.mmd` files. Project appearance configuration is stored in `.kumi-diagrams.json` inside the project directory. This makes diagrams easy to keep in Git, edit manually, and move between machines without being locked into the app.
