# Data Parsing Application

## Project Summary

This is a data parsing application that cleanses and transforms CSV data, particularly focused on processing review data with locations, dates, ratings, and image links. Written in Rust with Python bindings, it demonstrates real-world data processing while showcasing modern Rust-Python interoperability.

## Key Learning Points

### Rust Core Concepts:
- **Error handling with `thiserror`**
- **Type safety and pattern matching**
- **Module organization**
- **Unit testing**
- **Parallel processing with `rayon`**

### Python-Rust Integration:
- **Using `PyO3` for creating Python bindings**
- **`Maturin` for building Python packages from Rust**
- **Converting between Rust and Python data types**
- **Exposing Rust functionality to Python**

### Software Engineering Practices:
- **Structured error handling**
- **Modular code organization**
- **Test-driven development**
- **Cross-language interoperability**
- **Data processing patterns**

## Why It's Good for Beginners:
- Covers fundamental Rust concepts (ownership, modules, error handling)
- Shows practical data processing use cases
- Demonstrates how to integrate with existing Python ecosystems
- Includes comprehensive testing examples
- Deals with real-world data cleaning challenges

## Project Goals

The project serves as a bridge between theoretical Rust knowledge and practical application development, while also teaching valuable lessons about cross-language integration that's increasingly important in modern software development.

## Getting Started

### Prerequisites
- Rust (latest stable version)
- Python (3.6 or newer)
- `maturin` for building Python packages

### Installation
1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd <repository-directory>
   cargo build
   maturin develop
   ```
1. **Use the Python bindings:**
  ```bash
  import your_rust_module

  # Example usage
  your_rust_module.process_data("path/to/your/csvfile.csv")
  ```
