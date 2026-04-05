# Weather Landscape — Premium ✨ 🦀

A high-performance, real-time weather visualization engine built with **Rust** and **WebAssembly**, targeting the HTML5 Canvas for a smooth, static-web experience.

![Premium Web Experience](https://img.shields.io/badge/Tech-Rust%20%2B%20WASM-orange?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)

## 🌌 Overview
This project is a premium rewrite of the classic `weather_landscape` Python application. It uses **Open-Meteo API** to fetch real-time atmospheric data and renders a dynamic, responsive landscape with:
- **Atmospheric Gradients**: Real-time backgrounds that adapt to sunny, rainy, cloudy, or night conditions.
- **Weather Metrics Dashboard**: A sleek glassmorphism overlay for temperature, pressure, wind velocity, and cloud density.
- **WASM Rendering Engine**: High-performance pixel manipulations and complex Bezier-curved physics.

## 🛠️ Tech Stack
- **Core Engine**: Rust 2024 Edition
- **Bindings**: `wasm-bindgen`, `js-sys`, `web-sys`
- **Frontend**: Modern ES6 JavaScript & Vanilla CSS (Glassmorphism)
- **Data Source**: Open-Meteo API (No API key required)

## 🚀 Building locally
### Prerequisites
- [Rust](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Instructions
1. Install dependencies:
   ```bash
   wasm-pack build --target web
   ```
2. Serve the directory:
   ```bash
   python -m http.server 8080
   ```
3. Visit `http://localhost:8080` in your browser.

## 🚢 Automated Deployment
This repository is configured with a **GitHub Action** that automatically builds and deploys to **GitHub Pages** on every push to the `main` branch.

## 📄 License
This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for more details.

---

*Powered by Rust & WebAssembly*
