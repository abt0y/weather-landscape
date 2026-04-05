# Rust Weather Landscape — Premium ✨ 🦀

[![Build and Deploy](https://github.com/abt0y/weather-landscape/actions/workflows/deploy.yml/badge.svg)](https://github.com/abt0y/weather-landscape/actions/workflows/deploy.yml)
![Tech-Rust_WASM-orange?style=for-the-badge](https://img.shields.io/badge/Tech-Rust%20%2B%20WASM-orange?style=for-the-badge)
![License-MIT-blue?style=for-the-badge](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)

A complete, high-performance rewrite of the classic **`weather_landscape`** Python application. This project leverages **Rust** and **WebAssembly** to deliver a blazingly fast, real-time atmospheric visualization engine directly in the browser.

---

## 🌟 Modern Features & Rust Enhancements

The transition to Rust has enabled several premium features not possible in the original version:

- **🚀 WASM Rendering Pipeline**: All pixel manipulations, Bezier-curved terrain, and physics simulations are executed in high-performance WebAssembly.
- **🌅 Dynamic Atmosphere System**: A CSS/JS-driven background that adapts its color gradients and "body theme" (Sunny, Rainy, Cloudy, or Night) based on real-time API data.
- **📊 Live Weather Dashboard**: A sleek glassmorphism metrics panel showing:
  - **Temperature** (ported smoothed curve logic)
  - **Atmospheric Pressure** (influencing smoke physics)
  - **Wind Velocity & Direction** (affecting tree swaying and plant placement)
  - **Cloud Density** (dynamic sprite selection)
- **🌦️ Real-Time Sync**: Integrated with **Open-Meteo API** for global weather data without requiring an API key.

## 🛠️ Tech Stack & Architecture

- **Core Engine**: Rust (2024 Edition)
- **Frontend Layer**: Modern ES6+ JavaScript, Vanilla CSS
- **Interop**: `wasm-bindgen` for seamless JS-to-Rust communication
- **Graphics**: HTML5 Canvas rendering context managed by Rust `Renderer`
- **CI/CD**: Automated deployment to **GitHub Pages** via GitHub Actions

## 🏗️ Building Locally

To build and run the project on your local machine:

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (stable channel)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Development Workflow
1. **Compile to WASM**:
   ```bash
   wasm-pack build --target web
   ```
2. **Launch Local Server**:
   ```bash
   python -m http.server 8080
   ```
3. **Access App**: Open your browser to `http://localhost:8080`.

## 🚢 Automated Deployment

This project is fully automated. Every time you push to the `main` branch, a GitHub Action:
1. Installs the Rust toolchain and dependencies.
2. Compiles the project with full optimizations.
3. Bundles all assets (sprites, HTML, JS).
4. Deploys the result to **GitHub Pages**.

## ⚖️ License

Distributed under the **MIT License**. See `LICENSE` for more information.

---

*Engineered with precision using Rust & WebAssembly*
