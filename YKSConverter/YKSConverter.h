/**
 Copyright (c) 2018, Chanwoo Noh
 All rights reserved.
 */

#ifndef YKSCONVERTER_h
#define YKSCONVERTER_h

#include <string>
#include <memory>

#include "ByteBuffer.h"

class YKSConverter : public std::enable_shared_from_this<YKSConverter> {
public:
    YKSConverter(const std::string &mml = "") {
        _mml = mml;
        _timebase = 96;
    }
    
    void mml(const std::string &mml) { _mml = mml; }
    std::string mml() { return _mml; }
    /*
     * MML string을 midi buffer로 변환
     * return std::shared_ptr<YKS::ByteBuffer> 변환된 버퍼. 변환 실패 시 nullptr
     */
    std::shared_ptr<YKS::ByteBuffer> toBuffer();
private:
    
    /**
     * reference :
     *  http://midi.teragonaudio.com/tech/midifile.htm
     *  http://midi.teragonaudio.com/tech/midifile/vari.htm
     */
    std::shared_ptr<YKS::ByteBuffer> _writeVarLen(int value);
    
    std::string _mml;
    uint32_t _timebase;
};

#endif /* YKSCONVERTER_h */
