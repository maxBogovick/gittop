<p align="center">
  <img src="assets/logo.png" alt="GitTop Logo" width="120" height="120">
</p>

<h1 align="center">GitTop</h1>

<p align="center">
  <strong>The Ultimate Developer Trend Aggregator</strong>
</p>

<p align="center">
  One unified search engine for 12+ developer platforms. <br/>
  Track trending repositories, articles, discussions, and products — all in one place.
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#platforms">Platforms</a> •
  <a href="#installation">Installation</a> •
  <a href="#screenshots">Screenshots</a> •
  <a href="#tech-stack">Tech Stack</a> •
  <a href="#contributing">Contributing</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri">
  <img src="https://img.shields.io/badge/Vue.js-4FC08D?style=for-the-badge&logo=vue.js&logoColor=white" alt="Vue.js">
  <img src="https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white" alt="TypeScript">
  <img src="https://img.shields.io/badge/Tailwind-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white" alt="Tailwind">
  <img src="https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL">
</p>

---

## Why GitTop?

**Stop wasting hours jumping between tabs.** As a developer, you probably check GitHub trending, Reddit, Hacker News, Stack Overflow, and various blogs daily. That's 10+ browser tabs and endless context switching.

**GitTop solves this.** It's a blazing-fast native desktop app that aggregates content from all major developer platforms into a single, unified interface.

### The Problem

- **Tab overload** — 10+ browser tabs open just to stay updated
- **Time sink** — Hours spent manually checking different platforms
- **Fragmented search** — No way to search across all platforms at once
- **No unified view** — Missing that one trending repo or viral discussion

### The Solution

- **One app, all platforms** — Everything you need in a single window
- **Instant search** — Search across 12+ platforms simultaneously
- **Smart caching** — PostgreSQL-backed cache for lightning-fast repeat queries
- **Native performance** — Built with Rust & Tauri, not another Electron app

---

## Features

### Unified Global Search
Search across **all platforms at once** with a single query. Press `⌘K` (or `Ctrl+K`) to instantly search GitHub, Reddit, Stack Overflow, Hacker News, and more. No more opening 10 tabs to find what you need.

### Multi-Platform Trend Tracking
Track what's trending across the entire developer ecosystem:
- **Top & New repositories** — Discover trending GitHub repos by language
- **Hot discussions** — Reddit, Hacker News, Lobsters conversations
- **Latest articles** — Dev.to, Medium, Hashnode blog posts
- **New products** — Product Hunt launches, Indie Hackers projects

### Blazing Fast Performance
- Native desktop app built with **Rust** and **Tauri**
- **PostgreSQL caching** for instant repeat queries
- **~10x faster** than web-based alternatives
- **Minimal memory footprint** (not Electron!)

### Export Your Data
Export any data to **CSV** format for further analysis, reporting, or sharing with your team.

### Beautiful Dark/Light Themes
Easy on the eyes with carefully crafted dark and light themes. Automatically respects your system preferences.

### Keyboard-First Experience
- `⌘K` / `Ctrl+K` — Open global search
- Navigate entirely with keyboard shortcuts
- Designed for power users

### Real-Time Refresh
One-click cache refresh to get the latest data from all platforms.

---

## Platforms

GitTop aggregates content from **12+ platforms**:

| Platform | Content Type | Features |
|----------|-------------|----------|
| **GitHub** | Repositories | Top, New, by Language, README preview |
| **Reddit** | Posts | Top, New, Search by subreddit |
| **Stack Overflow** | Questions | Trending questions, multiple tags |
| **Hacker News** | Stories | Top stories, Search |
| **Dev.to** | Articles | Top, New articles |
| **Medium** | Articles | Trending articles |
| **Hashnode** | Posts | Developer blog posts |
| **Product Hunt** | Products | New product launches |
| **Lobsters** | Stories | Tech news, Story details |
| **Crates.io** | Rust Packages | Trending Rust crates |
| **Indie Hackers** | Posts | Maker community discussions |
| **Etsy** | Products | Search marketplace |

---

## Screenshots

<p align="center">
  <img src="assets/screenshot-dashboard.png" alt="Dashboard" width="800">
  <br/>
  <em>Unified Search Dashboard — Search 12+ platforms at once</em>
</p>

<p align="center">
  <img src="assets/screenshot-github.png" alt="GitHub Trending" width="800">
  <br/>
  <em>GitHub Trending — Filter by language, time range, and metrics</em>
</p>

<p align="center">
  <img src="assets/screenshot-search.png" alt="Search Results" width="800">
  <br/>
  <em>Global Search Results — All platforms, one search</em>
</p>

---

## Installation

### Prerequisites

- **Rust** (1.70+) — [Install Rust](https://rustup.rs/)
- **Node.js** (18+) — [Install Node.js](https://nodejs.org/)
- **PostgreSQL** (14+) — For caching

### Environment Setup

Create a `.env` file in the root directory:

```env
# Required for GitHub API (higher rate limits)
GITHUB_TOKEN=your_github_personal_access_token

# PostgreSQL connection
DATABASE_URL=postgres://user:password@localhost:5432/gittop

# Optional: Etsy API access
ETSY_API_KEY=your_etsy_api_key
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/your-username/gittop.git
cd gittop

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Download Pre-built Binaries

> Coming soon! Pre-built binaries for macOS, Windows, and Linux will be available in the Releases section.

---

## Tech Stack

GitTop is built with a modern, performant tech stack:

### Backend (Rust)
- **[Tauri](https://tauri.app/)** — Native desktop framework (10x smaller than Electron)
- **[Tokio](https://tokio.rs/)** — Async runtime for concurrent API calls
- **[Reqwest](https://docs.rs/reqwest/)** — HTTP client with connection pooling
- **[SQLx](https://github.com/launchbadge/sqlx)** — Async PostgreSQL driver
- **[Scraper](https://docs.rs/scraper/)** — HTML parsing for web scraping
- **[RSS](https://docs.rs/rss/)** — RSS feed parsing

### Frontend (Vue.js)
- **[Vue 3](https://vuejs.org/)** — Composition API with `<script setup>`
- **[TypeScript](https://www.typescriptlang.org/)** — Type-safe development
- **[Tailwind CSS 4](https://tailwindcss.com/)** — Utility-first styling
- **[Pinia](https://pinia.vuejs.org/)** — State management
- **[Vue Router](https://router.vuejs.org/)** — SPA navigation

### Why This Stack?

| Choice | Reason |
|--------|--------|
| **Rust + Tauri** | 10x smaller bundle, native performance, secure by default |
| **Vue 3** | Excellent DX, Composition API, great TypeScript support |
| **PostgreSQL** | Reliable caching, complex queries, production-ready |
| **Tailwind CSS** | Rapid UI development, consistent design system |

---

## Architecture

```
gittop/
├── src/                    # Vue.js frontend
│   ├── components/         # Reusable UI components
│   ├── views/              # Page components
│   ├── services/           # Tauri API bindings
│   ├── stores/             # Pinia state management
│   └── router/             # Vue Router config
│
├── src-tauri/              # Rust backend
│   └── src/
│       ├── commands/       # Tauri command handlers
│       ├── cache/          # PostgreSQL caching layer
│       ├── github/         # GitHub API client
│       ├── reddit/         # Reddit API client
│       ├── hackernews/     # HN API client
│       └── ...             # Other platform clients
```

---

## Roadmap

- [x] GitHub trending repositories
- [x] Reddit integration
- [x] Hacker News stories
- [x] Stack Overflow questions
- [x] Dev.to articles
- [x] Medium articles
- [x] Hashnode posts
- [x] Product Hunt products
- [x] Lobsters stories
- [x] Crates.io packages
- [x] Indie Hackers posts
- [x] Global unified search
- [x] CSV export
- [x] Dark/Light themes
- [ ] Pre-built binaries (macOS, Windows, Linux)
- [ ] Notifications for new trending items
- [ ] Customizable dashboard widgets
- [ ] Bookmark favorite items
- [ ] RSS feed export
- [ ] Browser extension companion

---

## Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Setup

```bash
# Install dependencies
npm install

# Start development server (hot reload)
npm run tauri dev

# Run type checking
npm run build
```

### Adding a New Platform

1. Create a new module in `src-tauri/src/{platform}/`
2. Implement the API client in `client.rs`
3. Add Tauri commands in `src-tauri/src/commands/{platform}.rs`
4. Create Vue components in `src/components/{Platform}Table.vue`
5. Add route in `src/router/index.ts`

---

## Support the Project

If GitTop helps you stay productive, consider:

- **Starring** the repository
- **Reporting** bugs and issues
- **Suggesting** new features
- **Contributing** code or documentation

---

## License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

## Author

**[Maxim Bogovic](https://bogovick.com)** — Developer & Creator

---

<p align="center">
  <strong>Built with Rust, Tauri, and Vue.js</strong>
</p>

<p align="center">
  <sub>Stop tab-hopping. Start shipping.</sub>
</p>
