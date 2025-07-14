#!/bin/bash

# Check arguments
if [ $# -ne 2 ]; then
  echo "Usage: $0 dayXX \"Title of the challenge\""
  exit 1
fi

DAY_DIR="$1"
TITLE="$2"

# Extract day number from directory name (e.g., day10 → 10)
DAY_NUM=$(echo "$DAY_DIR" | grep -o '[0-9]\+')

# Create directory
mkdir -p "$DAY_DIR"

# Create Makefile
cat > "$DAY_DIR/Makefile" << 'EOF'
.PHONY: all

all: main.rs
	rustc -C opt-level=3 -C strip=symbols  main.rs && ./main sample.txt

format:
	rustfmt main.rs
EOF

# Create main.rs with dynamic title
cat > "$DAY_DIR/main.rs" << EOF
use std::env;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day$DAY_NUM: $TITLE ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;
    println!("{}", input_file);

    Ok(())
}
EOF

# Create empty input and sample files
touch "$DAY_DIR/input.txt" "$DAY_DIR/sample.txt"

echo "$DAY_DIR initialized with title \"$TITLE\"."

