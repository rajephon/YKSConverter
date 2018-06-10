//
//  ByteBuffer.cpp
//  YKSConverterSample
//
//  Created by Chanwoo Noh on 2018. 6. 2..
//  Copyright © 2018년 Chanwoo Noh. All rights reserved.
//

#include <stdio.h>
#include "ByteBuffer.h"

namespace YKS {
    
    std::shared_ptr<ByteBuffer> ByteBuffer::putByte(uint8_t value) {
        _append<uint8_t>(value);
        return shared_from_this();
    }
    std::shared_ptr<ByteBuffer> ByteBuffer::putBytes(std::shared_ptr<ByteBuffer> value) {
        auto size = value->size();
        for (uint32_t i = 0; i < size; i++) {
            _append<uint8_t>(value->get(i));
        }
        return shared_from_this();
    }
    std::shared_ptr<ByteBuffer> ByteBuffer::putShort(uint16_t value) {
        _append<uint16_t>(value);
        return shared_from_this();
    }
    std::shared_ptr<ByteBuffer> ByteBuffer::putString(const std::string &value) {
        for (int i = 0; i < value.size(); i++) {
            _append<char>(value.at(i));
        }
        return shared_from_this();
    }
    std::shared_ptr<ByteBuffer> ByteBuffer::putBytes(uint8_t *value, uint32_t len) {
        for (uint32_t i = 0; i < len; i++) {
            _append<uint8_t>(value[i]);
        }
        return shared_from_this();
    }
    
    uint8_t ByteBuffer::get() const {
        return this->read<uint8_t>();
    }
    
    uint8_t ByteBuffer::get(uint32_t idx) const {
        return this->read<uint8_t>(idx);
    }
    
    char ByteBuffer::getChar() const {
        return this->read<char>();
    }
    
    uint32_t ByteBuffer::size() {
        return (uint32_t)_buf.size();
    }
    
    void ByteBuffer::clear() {
        _buf.clear();
    }
    
    void ByteBuffer::printHex() {
        auto length = (uint32_t)_buf.size();
        std::cout << "ByteBuffer Length: " << length << ", Hex: "; //  Print" << std::endl;
        for (int i = 0; i < length; i++) {
            std::printf("%02x ", _buf[i]);
        }
        std::cout << "\n";
    }
}
