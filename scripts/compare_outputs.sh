#!/bin/bash

# YKSConverter Output Comparison Script
# Compares C++ and Rust implementation outputs

# Don't exit on errors to see all test results
set +e

echo "=== YKSConverter Output Comparison ==="
echo

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

# Function to compare baseline test outputs
compare_baseline_outputs() {
    echo -e "${BLUE}Comparing baseline test outputs...${NC}"
    
    # Generate C++ output
    cd cpp/build
    if ! ./test_baseline > cpp_baseline_output.txt 2>&1; then
        echo -e "${RED}❌ C++ test_baseline failed${NC}"
        cat cpp_baseline_output.txt
        cd ../..
        return 1
    fi
    cd ../..
    
    # Generate Rust output  
    cd rust
    if ! cargo run --release --bin test_baseline > rust_baseline_output.txt 2>&1; then
        echo -e "${RED}❌ Rust test_baseline failed${NC}"
        cat rust_baseline_output.txt
        cd ..
        return 1
    fi
    cd ..
    
    # Debug: List generated files
    echo -e "${BLUE}Generated C++ files:${NC}"
    ls -la cpp/build/*.midi || echo "No C++ MIDI files found"
    echo -e "${BLUE}Generated Rust files:${NC}"
    ls -la rust/*.midi || echo "No Rust MIDI files found"
    
    # Compare generated MIDI files
    local files_match=true
    
    if [ -f "cpp/build/test1_simple_single.midi" ] && [ -f "rust/test1_simple_single_rust.midi" ]; then
        if ! cmp -s "cpp/build/test1_simple_single.midi" "rust/test1_simple_single_rust.midi"; then
            echo -e "${RED}❌ FAIL - test1_simple_single.midi files differ${NC}"
            files_match=false
            ((FAILED++))
        else
            echo -e "${GREEN}✅ PASS - test1_simple_single.midi files match${NC}"
            ((PASSED++))
        fi
    else
        echo -e "${RED}❌ FAIL - test1_simple_single.midi files not found${NC}"
        files_match=false
        ((FAILED++))
    fi
    
    if [ -f "cpp/build/test2_multi_track.midi" ] && [ -f "rust/test2_multi_track_rust.midi" ]; then
        if ! cmp -s "cpp/build/test2_multi_track.midi" "rust/test2_multi_track_rust.midi"; then
            echo -e "${RED}❌ FAIL - test2_multi_track.midi files differ${NC}"
            files_match=false
            ((FAILED++))
        else
            echo -e "${GREEN}✅ PASS - test2_multi_track.midi files match${NC}"
            ((PASSED++))
        fi
    else
        echo -e "${RED}❌ FAIL - test2_multi_track.midi files not found${NC}"
        files_match=false
        ((FAILED++))
    fi
    
    if [ -f "cpp/build/test3_readme_example.midi" ] && [ -f "rust/test3_readme_example_rust.midi" ]; then
        if ! cmp -s "cpp/build/test3_readme_example.midi" "rust/test3_readme_example_rust.midi"; then
            echo -e "${RED}❌ FAIL - test3_readme_example.midi files differ${NC}"
            files_match=false
            ((FAILED++))
        else
            echo -e "${GREEN}✅ PASS - test3_readme_example.midi files match${NC}"
            ((PASSED++))
        fi
    else
        echo -e "${RED}❌ FAIL - test3_readme_example.midi files not found${NC}"
        files_match=false
        ((FAILED++))
    fi
    
    if [ -f "cpp/build/test4_empty_tracks.midi" ] && [ -f "rust/test4_empty_tracks_rust.midi" ]; then
        if ! cmp -s "cpp/build/test4_empty_tracks.midi" "rust/test4_empty_tracks_rust.midi"; then
            echo -e "${RED}❌ FAIL - test4_empty_tracks.midi files differ${NC}"
            files_match=false
            ((FAILED++))
        else
            echo -e "${GREEN}✅ PASS - test4_empty_tracks.midi files match${NC}"
            ((PASSED++))
        fi
    else
        echo -e "${RED}❌ FAIL - test4_empty_tracks.midi files not found${NC}"
        files_match=false
        ((FAILED++))
    fi
}

# Run baseline comparison
compare_baseline_outputs

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