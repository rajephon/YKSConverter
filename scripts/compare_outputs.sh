#!/bin/bash

# YKSConverter Output Comparison Script
# Compares C++ and Rust implementation outputs

set -e

echo "=== YKSConverter Output Comparison ==="
echo

# Test cases
declare -a TEST_CASES=(
    "MML@c,,;"
    "MML@t120l4cdefgab>c4.,,;"
    "MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;"
    "MML@t180l8ccccccc4,l8eeeeeee4,l8ggggggg4;"
)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0

# Build both versions
echo "Building C++ version..."
cd cpp
if [ ! -f "build/Makefile" ]; then
    mkdir -p build
    cd build
    cmake ..
    cd ..
fi
make -C build > /dev/null 2>&1 || {
    echo -e "${RED}❌ Failed to build C++ version${NC}"
    exit 1
}

cd ..

echo "Building Rust version..."
cd rust
cargo build --release > /dev/null 2>&1 || {
    echo -e "${RED}❌ Failed to build Rust version${NC}"
    exit 1
}
cd ..

echo -e "${GREEN}✅ Both versions built successfully${NC}"
echo

# Function to compare outputs
compare_output() {
    local mml="$1"
    local test_name="$2"
    
    echo -e "${BLUE}Testing: $test_name${NC}"
    echo "MML: $mml"
    
    # Generate C++ output
    cd cpp/build
    if ! ./test_baseline "$mml" > cpp_output.txt 2>&1; then
        echo -e "${RED}❌ C++ version failed${NC}"
        cd ../..
        return 1
    fi
    
    # Extract hex from C++ output (assuming it's in the output)
    CPP_HEX=$(grep -E '^[0-9a-f]+$' cpp_output.txt | head -1 | tr -d ' \n')
    cd ../..
    
    # Generate Rust output  
    cd rust
    RUST_HEX=$(cargo run --release -- "$mml" 2>/dev/null | grep -E '^[0-9a-f]+$' | head -1 | tr -d ' \n')
    cd ..
    
    # Compare outputs
    if [ "$CPP_HEX" = "$RUST_HEX" ]; then
        echo -e "${GREEN}✅ PASS - Outputs match exactly${NC}"
        echo "Length: ${#CPP_HEX} characters"
        ((PASSED++))
    else
        echo -e "${RED}❌ FAIL - Outputs differ${NC}"
        echo "C++  length: ${#CPP_HEX} characters"
        echo "Rust length: ${#RUST_HEX} characters"
        
        if [ ${#CPP_HEX} -gt 0 ] && [ ${#RUST_HEX} -gt 0 ]; then
            echo "First 100 chars:"
            echo "C++:  ${CPP_HEX:0:100}..."
            echo "Rust: ${RUST_HEX:0:100}..."
        fi
        ((FAILED++))
    fi
    
    echo
}

# Run test cases
for i in "${!TEST_CASES[@]}"; do
    compare_output "${TEST_CASES[$i]}" "Test $((i+1))"
done

# Run Rust test suite
echo -e "${BLUE}Running Rust test suite...${NC}"
cd rust
if cargo test > /dev/null 2>&1; then
    echo -e "${GREEN}✅ All Rust tests pass${NC}"
else
    echo -e "${RED}❌ Some Rust tests failed${NC}"
    ((FAILED++))
fi
cd ..

# Summary
echo "=== Summary ==="
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed! Binary compatibility confirmed.${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Check output above.${NC}"
    exit 1
fi