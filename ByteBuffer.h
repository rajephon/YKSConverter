//
//  ByteBuffer.h
//  YKSConverterSample
//
//  Created by Chanwoo Noh on 2018. 6. 2..
//  Copyright © 2018년 Chanwoo Noh. All rights reserved.
//

#ifndef Util_hpp
#define Util_hpp

#include <vector>
#include <memory>
#include <inttypes.h>
#include <iostream>
#include <algorithm>
#include <stdint.h>

#ifdef GTEST_INCLUDE_GTEST_GTEST_H_
#include <gtest/gtest_prod.h>
class ByteBuffer_init_Test;
#endif

namespace YKS {

class ByteBuffer : public std::enable_shared_from_this<ByteBuffer> {
public:
    ByteBuffer() {
        _readPos = 0;
        _writePos = 0;
    }
    
    template<typename T>
    std::shared_ptr<ByteBuffer> put(T value) {
        _append<T>(value);
        return shared_from_this();
    }
    std::shared_ptr<ByteBuffer> putByte(uint8_t value);
    std::shared_ptr<ByteBuffer> putBytes(std::shared_ptr<ByteBuffer> value);
    std::shared_ptr<ByteBuffer> putShort(uint16_t value);
    std::shared_ptr<ByteBuffer> putString(const std::string &value);
    std::shared_ptr<ByteBuffer> putBytes(uint8_t *value, uint32_t len);
    
    uint8_t get() const;
    uint8_t get(uint32_t idx) const;
    char getChar() const;
    
    uint32_t size();
    void clear();
    
    void printHex();
private:
    template<typename T>
    T read() const {
        T data = read<T>(_readPos);
        auto size = sizeof(T);
        _readPos += size;
        return data;
    }
    
    template<typename T>
    T read(uint32_t index) const {
        if (index + sizeof(T) <= _buf.size()) {
            return *((T*) &_buf[index]);
        }
        return 0;
    }
    
    template<typename T>
    void _append(T value) {
        auto valueSize = sizeof(T);
        if (_buf.size() >= (uint32_t)(_writePos + valueSize)) {
            std::cerr << "error: buffer overflow";
            return;
        }
        _buf.resize(_writePos + valueSize);
        
        uintptr_t start = (uintptr_t)&value;
        
        for (int i = 0; i < valueSize; i++) {
            if (_isBigEndian()) {
                _buf.at(_writePos + i) = *reinterpret_cast<uint8_t *>(start+(uintptr_t)i);
            }else {
                _buf.at(_writePos+(valueSize - i - 1)) = *reinterpret_cast<uint8_t *>(start+(uintptr_t)i);
            }
        }
        _writePos += valueSize;
    }
    
    template<typename T>
    void _insert(T data, uint32_t index) {
        auto dataSize = sizeof(data);
        if ((index + dataSize) > size())
            return;
        std::copy(&_buf[index], &_buf[index] + dataSize, (uint8_t *)data);
        _writePos = index + (uint32_t)dataSize;
    }
    
    bool _isBigEndian() {
        union {
            uint32_t i;
            char c[4];
        } bint = {0x01020304};
        return bint.c[0] == 1;
    }
    
    mutable uint32_t _readPos = 0;
    uint32_t _writePos = 0;
    std::vector<uint8_t> _buf;
    
#ifdef GTEST_INCLUDE_GTEST_GTEST_H_
    friend class ::ByteBuffer_init_Test;
#endif
};
    
}

#endif /* Util_hpp */
