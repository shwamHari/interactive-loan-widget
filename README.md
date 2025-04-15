
# 🌟 Interactive Loan Widget

## 💡 Overview
Welcome to the **Interactive Loan Calculator**! This project is crafted with **Rust** and **WebAssembly (WASM)** to provide an interactive, user-friendly experience for calculating monthly payments, total interest, and other key financial metrics based on your loan details.

## 🎨 Features
- **📊 Graphical Representations**: Visualize your payment breakdown and amortization schedule using **Chart.js**.
- **⏱ Real-time Calculations**: See dynamic updates of calculations as you modify loan details.
- **🖥 Interactive UI**: A sleek and user-friendly interface for easy input.
- **🔧 Customizable Inputs**: Adjust loan amount, interest rate, and loan term to fit your needs.

## Demo 🎥
Check out this video showcasing the app in action:


https://github.com/user-attachments/assets/90661649-a1d7-4ffb-b9b0-6759754b4b3d


## ⚙️ Setup and Installation

### 🛠️ Dependencies
Ensure you have the following dependencies installed:
- **Rust** 🦀
- **Python** 🐍
- **wasm-pack**
- **serde**

### 📝 Installation Steps
1. **Clone the repository**:
   ```sh
   git clone https://github.com/shwamHari/interactive-loan-widget.git
   ```
2. **Build the WASM package**:
   ```sh
   wasm-pack build --target web
   ```
3. **Start the application**:
   ```sh
   python -m http.server 8000
   ```
4. Open your browser and go to:
   [http://localhost:8000](http://localhost:8000)
