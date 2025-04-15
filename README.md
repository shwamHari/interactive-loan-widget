# Interactive Loan Calculator

## Overview
This project is an **Interactive Loan Calculator** built using **Rust and WebAssembly (WASM)**. It provides an intuitive user interface to calculate and display monthly payments, total interest, and other key financial metrics based on user input.

## Features
- **Graphical Representations**: Visualised payment breakdowns and amortization schedules, using Chart.js
- **Real-time calculations**: Updates calculations dynamically as users input loan details.
- **Interactive UI**: User-friendly interface with easy input methods.
- **Customizable Inputs**: Adjust loan amount, interest rate, and loan term.


## Setup and Installation
### Dependencies
- **Rust**
- **Python**
- **wasm-pack**
- **serde**

### Steps
1. Clone the repository:
   ```sh
   git clone https://github.com/shwamHari/interactive-loan-widget.git
   ```
2. Build the WASM package:
   ```sh
   wasm-pack build --target web
   ```
4. Start the application:
   ```sh
   python -m http.server 8000
   ```
5. Open your browser and go to `http://localhost:8000`
