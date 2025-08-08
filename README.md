# EnhanceKeys

A Tauri-based desktop application that provides preloaded text snippets that can be injected into AI chat windows via customizable keyboard shortcuts.

## Overview

EnhanceKeys allows users to create, manage, and quickly insert predefined text snippets into AI chat interfaces through configurable hotkeys. This tool enhances productivity when working with AI assistants by providing instant access to commonly used prompts, templates, and responses.

## Features

- **Text Snippet Management**: Create, edit, and organize text snippets
- **Keyboard Shortcuts**: Assign custom hotkeys to your most-used snippets
- **System Integration**: Seamlessly inject text into active chat windows
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Modern UI**: Clean, responsive interface built with React and Tailwind CSS

## Tech Stack

### Backend
- **Rust**: Core application logic and system integration
- **Tauri**: Cross-platform desktop app framework

### Frontend
- **React**: User interface components
- **TypeScript**: Type-safe development
- **Tailwind CSS**: Modern, utility-first styling

### Configuration
- **TOML**: Human-readable configuration files
- **JSON**: Data storage and API responses

### Future Enhancements
- **Supabase**: Cloud database for snippet synchronization (planned for future versions)

## Project Structure

```
prompt-enhancer/
├── src-tauri/                 # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs           # Application entry point
│   │   ├── config.rs         # Configuration management
│   │   ├── hotkeys.rs        # Keyboard shortcut handling
│   │   ├── text_injection.rs # Text injection logic
│   │   └── system_integration.rs # OS integration
│   ├── Cargo.toml            # Rust dependencies
│   └── tauri.conf.json       # Tauri configuration
├── src/                      # React frontend
│   ├── components/
│   │   ├── ButtonGrid.tsx    # Snippet button grid
│   │   ├── SettingsPanel.tsx # Settings interface
│   │   └── Button.tsx        # Reusable button component
│   ├── hooks/
│   │   └── useConfig.ts      # Configuration hooks
│   ├── App.tsx               # Main application component
│   ├── main.tsx              # React entry point
│   └── styles/
│       └── index.css         # Global styles
├── public/                   # Static assets
├── config/
│   └── default_snippets.toml # Default text snippets
├── package.json              # Node.js dependencies
├── tailwind.config.js        # Tailwind configuration
├── vite.config.ts            # Vite build configuration
└── README.md                 # This file
```

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rust-lang.org/) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/setup/)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/enhancekeys.git
   cd enhancekeys
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run the development server:
   ```bash
   npm run tauri dev
   ```

## Development

### Backend Development (Rust)

The Rust backend handles:
- System-level keyboard shortcuts
- Text injection into active windows
- Configuration management
- Cross-platform compatibility

### Frontend Development (React)

The React frontend provides:
- Snippet management interface
- Settings configuration
- Real-time updates
- Responsive design

## Configuration

Snippets are stored in TOML format for easy editing:

```toml
[snippets]
"greeting" = "Hello! How can I help you today?"
"code_review" = "Please review this code and provide feedback on..."
"bug_report" = "I'm experiencing an issue with..."
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] MVP with basic snippet management
- [ ] Custom keyboard shortcuts
- [ ] Text injection into popular AI chat interfaces
- [ ] Snippet categories and tags
- [ ] Cloud synchronization with Supabase
- [ ] Plugin system for custom integrations
- [ ] Mobile companion app
