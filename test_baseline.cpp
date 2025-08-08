#include <iostream>
#include <memory>
#include <fstream>
#include <inttypes.h>
#include <vector>
#include <iomanip>
#include "YKSConverter/YKSConverter.h"

void writeBufferToFile(std::shared_ptr<YKS::ByteBuffer> buffer, const std::string& filename) {
    std::ofstream out(filename, std::ios::binary);
    for (int i = 0; i < buffer->size(); i++) {
        out << buffer->get(i);
    }
    out.close();
    std::cout << "Generated: " << filename << " (size: " << buffer->size() << " bytes)" << std::endl;
}

void printBufferHex(std::shared_ptr<YKS::ByteBuffer> buffer, const std::string& name) {
    std::cout << name << " hex dump:" << std::endl;
    for (int i = 0; i < buffer->size(); i++) {
        std::cout << std::hex << std::setfill('0') << std::setw(2) << (int)buffer->get(i) << " ";
        if ((i + 1) % 16 == 0) std::cout << std::endl;
    }
    std::cout << std::dec << std::endl << std::endl;
}

int main() {
    std::cout << "=== YKSConverter C++ Baseline Tests ===" << std::endl;
    
    // Test Case 1: Simple single track
    std::cout << "Test 1: Simple single track" << std::endl;
    std::string mml1 = "MML@t120l4cdefgab>c4.,,,;";
    auto converter1 = std::make_shared<YKSConverter>(mml1, 1);
    auto buffer1 = converter1->toBuffer();
    if (buffer1) {
        writeBufferToFile(buffer1, "test1_simple_single.midi");
        printBufferHex(buffer1, "Test1");
    }
    
    // Test Case 2: Multi-track ensemble
    std::cout << "Test 2: Multi-track ensemble" << std::endl;
    std::vector<std::string> mml2 = {
        "MML@t180l8ccccccc4,l8eeeeeee4,l8ggggggg4;",
        "MML@t180l8>ccccccc4,l8>eeeeeee4,l8>ggggggg4;"
    };
    std::vector<uint8_t> inst2 = {26, 74};
    auto converter2 = std::make_shared<YKSConverter>(mml2, inst2);
    auto buffer2 = converter2->toBuffer();
    if (buffer2) {
        writeBufferToFile(buffer2, "test2_multi_track.midi");
        printBufferHex(buffer2, "Test2");
    }
    
    // Test Case 3: Complex MML from README
    std::cout << "Test 3: README example" << std::endl;
    std::string mml3 = "MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;";
    auto converter3 = std::make_shared<YKSConverter>(mml3, 1);
    auto buffer3 = converter3->toBuffer();
    if (buffer3) {
        writeBufferToFile(buffer3, "test3_readme_example.midi");
        printBufferHex(buffer3, "Test3");
    }
    
    // Test Case 4: Edge case - empty tracks
    std::cout << "Test 4: Edge case with empty tracks" << std::endl;
    std::string mml4 = "MML@t120l4cde,,;";
    auto converter4 = std::make_shared<YKSConverter>(mml4, 1);
    auto buffer4 = converter4->toBuffer();
    if (buffer4) {
        writeBufferToFile(buffer4, "test4_empty_tracks.midi");
        printBufferHex(buffer4, "Test4");
    }
    
    std::cout << "=== Baseline tests completed ===" << std::endl;
    return 0;
}