# EnhanceKeys - Prompt Enhancement Desktop Application

A native desktop application that provides quick access to predefined prompt enhancement text snippets through a 3x3 grid interface with global hotkeys.

## Overview

EnhanceKeys is designed to seamlessly append standardized helpful prompt additions to any text input field across your system, improving prompt effectiveness without breaking workflow. The application uses a fixed set of 9 prompt enhancement snippets activated via Alt+Shift+1-9 hotkeys.

## Features

### Core Functionality
- **3x3 Grid Interface**: Fixed layout with 9 predefined prompt enhancement buttons
- **Global Hotkeys**: Alt+Shift+1-9 system-wide shortcuts for instant text injection  
- **Text Injection**: Automatically adds line break before injecting enhancement text
- **Always-on-Top Window**: Positioned in top-right corner with semi-transparent overlay
- **Cross-Platform**: Native desktop app for Windows, macOS, and Linux

### Predefined Snippets
1. **Ask Questions**: Prompts for clarifying questions until 95% context certainty
2. **Need Review**: Escalation pattern with "I don't know ===" exit keywords  
3. **More Context**: Requests for additional context and requirements clarification
4. **Step by Step**: Requests numbered, actionable steps with reasoning
5. **Examples**: Requests concrete examples, code snippets, and demonstrations
6. **Validation**: Requests validation steps and success criteria
7. **Error Handling**: Requests error handling strategies and edge case coverage
8. **Documentation**: Requests documentation-formatted responses with clear structure
9. **Testing**: Requests testing suggestions for both positive and edge cases

## Tech Stack

- **Rust Backend**: System integration, hotkey management, text injection
- **React + TypeScript Frontend**: 3x3 grid UI with Tailwind CSS styling  
- **Tauri Framework**: Cross-platform desktop app with web frontend
- **TOML Configuration**: Human-readable config files for snippets
- **Global Hotkey Library**: System-wide Alt+Shift+1-9 shortcuts
- **Enigo**: Cross-platform input simulation for text injection

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

The application uses a TOML configuration file with predefined snippets:

### config/default_snippets.toml
```toml
[hotkeys]
button_1 = "Alt+Shift+1"
button_2 = "Alt+Shift+2"
# ... through button_9

[snippets]
button_1_title = "Ask Questions"
button_1_text = "Ask the user questions in relation to supplementing the context you are given in the prompt..."
button_1_description = "Prompts for clarifying questions"

# ... 8 more predefined enhancement snippets

[ui]
always_on_top = true
transparency = 0.9
position_x = "top-right"
auto_start = false
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Implementation Status

### ✅ Completed (MVP)
- [x] Project structure and Tauri setup with React/Tailwind
- [x] 3x3 grid UI layout with predefined buttons
- [x] Window positioning (top-right) and always-on-top behavior  
- [x] Semi-transparent overlay styling
- [x] Configuration system with TOML file parsing
- [x] Load predefined text snippets from config into buttons
- [x] Global hotkey system (Alt+Shift+1-9)
- [x] Text injection mechanism for focused input fields

### 🔄 In Progress
- [ ] System tray integration
- [ ] Cross-platform testing and refinements

### 📋 Future Enhancements (Post-MVP)
- [ ] Custom snippet creation UI
- [ ] Snippet categories and organization
- [ ] Cloud synchronization with Supabase
- [ ] Usage analytics and optimization
- [ ] Plugin system for custom integrations

## Known Issues
- Currently encountering JavaScriptCore dependency issues during build on Linux
- Requires system dependencies: pkg-config, libssl-dev, libgtk-3-dev, libwebkit2gtk-4.1-dev
- Global hotkey registration may conflict with other applications
