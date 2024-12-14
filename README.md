# Matra Library (`matra_lib`)

## Overview

The **Matra Library** (`matra_lib`) is a Rust library designed for analyzing Devanagari text, focusing on the syllabic structures called "matra." It provides essential tools to parse, analyze, and interpret Devanagari script for poetic, linguistic, and educational applications. Originally inspired by the classical Indian tradition of *Matrik and Varnik Chhand*, this library is built to assist in understanding and composing structured verse, as well as exploring the intricate patterns of Devanagari text.

## Features

- **Syllable Analysis**: Analyze Devanagari text to break it into its smallest units (*varn*) and determine their associated matra values.
- **Word and Line Breakdown**: Process words and lines of text into their constituent syllables and matra counts for deeper insight.
- **Extensible Functionality**: The library is designed for integration into larger applications, such as web-based tools or standalone analyzers.
- **JSON Integration**: [#TODO] Serialize and deserialize analysis results using `serde`, making it easy to connect with JavaScript frontends or other external systems.

## Use Cases

- **Poetry Analysis**: Extract matra patterns from Devanagari poetry, aiding scholars, enthusiasts, and composers in evaluating metrical accuracy.
- **Educational Applications**: Teach syllabic structures and rhythmic principles of Devanagari texts.
- **Web Integration**: Build client-facing web applications (e.g., WASM apps) to deliver analysis tools directly to users.
- **Text Analysis Tools**: Use as part of NLP (Natural Language Processing) pipelines to analyze Patterns in Sanskrit/Hindi poetic works.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version)
- Optional: WASM-pack for WebAssembly integration

### Installation

1. Add the library to your project:
    
    ```bash
    cargo add matra_lib
    ```
    
2. Or clone the repository for development:
    
    ```bash
    git clone https://github.com/your-username/matra_lib.git
    cd matra_lib
    ```
    
3. Build the project:
    
    ```bash
    cargo build
    ```
    
4. Run tests:
    
    ```bash
    cargo test
    ```
    

### Prerequisites

- Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.

### Installation

1. Clone the repository:
    
    ```bash
    git clone <https://github.com/aknautiyal/matra_lib.git>
    cd devanagari-matra-analyzer
    
    ```
    
2. Build the project:
    
    ```bash
    cargo build
    
    ```
    
3. Run the analyzer:
    
    ```bash
    cargo run
    
    ```
    

## Usage

Here’s a basic example of how to use `matra_lib`:

```rust
use matra_lib::{Charan, Shabd, VarnList};

fn main() {
    let line = "राम जी की जय";
    let charan = Charan::from_str(line);

    println!("Varns: {}", charan.analysis().varns);
    println!("Matra: {}", charan.analysis().matra);
    println!("Total Matra Count: {}", charan.analysis().total);
}
```
