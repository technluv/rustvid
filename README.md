# Rust Video Editor

A high-performance, modular video editing application built with Rust, designed as an open-source alternative to Veed.io.

## Project Vision

This project aims to create a professional-grade video editing solution that combines the performance and safety of Rust with modern video processing capabilities. Key goals include:

- **Performance**: Leverage Rust's zero-cost abstractions for real-time video processing
- **Modularity**: Clean separation of concerns with a workspace-based architecture
- **Cross-platform**: Support for Windows, macOS, and Linux
- **Extensibility**: Plugin system for custom effects and workflows
- **Modern UI**: Responsive, intuitive interface for professional video editing

## Architecture

The project is organized as a Cargo workspace with the following crates:

- **`core`**: Video processing engine, codec abstraction, and pipeline management
- **`timeline`**: Timeline management, clips, tracks, and editing operations
- **`effects`**: Video and audio effects system with plugin support
- **`ui`**: User interface layer with support for multiple UI frameworks
- **`export`**: Rendering pipeline and export to various formats

## Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)
- Platform-specific dependencies:
  - **Linux**: GTK3 development libraries (`libgtk-3-dev` on Ubuntu/Debian)
  - **Windows**: Visual Studio Build Tools
  - **macOS**: Xcode Command Line Tools

## Getting Started

1. Clone the repository:
```bash
git clone https://github.com/your-org/rust-video-editor.git
cd rust-video-editor
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Build documentation:
```bash
cargo doc --open
```

## Development

### Project Structure

```
rust-video-editor/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── core/              # Video processing engine
│   ├── timeline/          # Timeline management
│   ├── effects/           # Effects system
│   ├── ui/                # User interface
│   └── export/            # Rendering and export
├── examples/              # Example applications
├── tests/                 # Integration tests
└── docs/                  # Additional documentation
```

### Running Examples

```bash
# Run a specific example (once examples are added)
cargo run --example basic_timeline
```

### Code Style

This project follows standard Rust conventions:
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Write tests for new functionality
- Document public APIs

## Roadmap

### Phase 1: Foundation (Current)
- [x] Project structure and workspace setup
- [x] Basic crate definitions
- [x] CI/CD pipeline
- [ ] Core video processing abstractions
- [ ] Basic timeline implementation

### Phase 2: MVP
- [ ] Video decoding/encoding with chosen framework
- [ ] Timeline with basic editing operations
- [ ] Simple effects (brightness, contrast, etc.)
- [ ] Basic UI with timeline view
- [ ] Export to common formats (MP4, WebM)

### Phase 3: Advanced Features
- [ ] Advanced effects and transitions
- [ ] Audio processing and mixing
- [ ] Plugin system
- [ ] GPU acceleration
- [ ] Collaborative editing features

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is dual-licensed under MIT OR Apache-2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

## Acknowledgments

This project is being developed as part of a collaborative AI-assisted development process using Claude-Flow and SPARC methodology.