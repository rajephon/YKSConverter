/**
 Copyright (c) 2018, Chanwoo Noh
 All rights reserved.
 */

#include "YKSConverter.h"
#include "MF2TT2MF/MF2TT2MF.h"

std::shared_ptr<YKS::ByteBuffer> YKSConverter::toBuffer() {
    auto mf2tt2mf = std::make_shared<YKS::MF2TT2MF>();
    if (!mf2tt2mf->fromMML(_mml)) {
        return nullptr;
    }
    
    auto trackEventList = mf2tt2mf->build();
    if (trackEventList.empty()) {
        return nullptr;
    }

    std::vector<uint8_t> defaultBuffer = { 0x00, 0x00, 0x00, 0x06, 0x00 };
    
    auto byteBuffer = std::make_shared<YKS::ByteBuffer>();
    byteBuffer->putString("MThd");
    byteBuffer->putBytes(&defaultBuffer[0], (uint32_t)defaultBuffer.size());
    // track exist check 1 : 0
    byteBuffer->putByte((trackEventList.size() > 1)? 1 : 0);
    // track size 2 bytes
    byteBuffer->put<uint16_t>(trackEventList.size());
    // timebase 2 bytes
    byteBuffer->put<uint16_t>(_timebase);
    
    for (const auto &eventList : trackEventList) {
        uint32_t time = 0;
        byteBuffer->putString("MTrk");
        auto trackBuffer = std::make_shared<YKS::ByteBuffer>();
        
        uint8_t last = 0x00;
        
        for (const auto &event : eventList) {
            uint32_t deltaTime = event->leadTime() - time;
            time = event->leadTime();
            trackBuffer->putBytes(_writeVarLen(deltaTime));
            
            auto eventBuffer = event->toBuffer();
            if (eventBuffer->size() <= 0) {
                std::cerr << "Event Convert error.";
                std::cerr << event->value();
                continue;
            }
            // repetition, same event, same channel, omit first byte (smaller file size)
            // from http://valentin.dasdeck.com/midi/ midi.class.php
            // thx Valentin Schmidt
            uint8_t start = eventBuffer->get(0);
            if (start < 0x80 || start > 0xef || start != last) {
                trackBuffer->putByte(start);
            }
            for (int i = 1; i < eventBuffer->size(); i++) {
                trackBuffer->putByte(eventBuffer->get(i));
            }
            last = start;
        }
        uint32_t trackLength = trackBuffer->size();
        byteBuffer->put<uint32_t>(trackLength)->putBytes(trackBuffer);
    }
    
    return byteBuffer;
}

std::shared_ptr<YKS::ByteBuffer> YKSConverter::_writeVarLen(int value) {
    int buf = value & 0x7f; // 0x7f : 127
    auto byteBuffer = std::make_shared<YKS::ByteBuffer>();
    while ((value >>= 7)) {
        buf <<= 8;
        buf |= ((value & 0x7f) | 0x80);
    }
    while (true) {
        byteBuffer->putByte(buf%256);
        if (buf & 0x80) { // 0x80 : 128 : 0x10000000(2)
            buf >>= 8;
        }else {
            break;
        }
    }
    return byteBuffer;
}
