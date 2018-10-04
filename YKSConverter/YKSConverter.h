/**
 Copyright (c) 2018, Chanwoo Noh
 All rights reserved.
 */

#ifndef YKSCONVERTER_h
#define YKSCONVERTER_h

#include <string>
#include <memory>
#include <vector>
#include <inttypes.h>

#include "ByteBuffer.h"

class YKSConverter : public std::enable_shared_from_this<YKSConverter> {
public:
    YKSConverter(const std::string &mml = "") {
        _mml = {mml};
        _inst = { 1 };
        _timebase = START_TIMEBASE;
    }
    YKSConverter(const std::vector<std::string> &mml, const std::vector<uint8_t> &inst) {
        _mml = mml;
        _inst = inst;
        _timebase = START_TIMEBASE;
    }
    
    void mml(const std::string &mml) { _mml = {mml}; }
    void mml(const std::vector<std::string> &mml) { _mml = mml; }
    void inst(const uint8_t inst) { _inst = {inst}; }
    void inst(const std::vector<uint8_t> inst) { _inst = inst; }
    
    std::vector<std::string> mml() { return _mml; }
    std::vector<uint8_t> inst() { return _inst; }
    
    /*
     * MML string을 midi buffer로 변환
     * return std::shared_ptr<YKS::ByteBuffer> 변환된 버퍼. 변환 실패 시 nullptr
     */
    std::shared_ptr<YKS::ByteBuffer> toBuffer();
private:
    // 1 17 25
    /**
     * reference :
     *  http://midi.teragonaudio.com/tech/midifile.htm
     *  http://midi.teragonaudio.com/tech/midifile/vari.htm
     */
    std::shared_ptr<YKS::ByteBuffer> _writeVarLen(int value);
    
    std::vector<std::string> _mml;
    std::vector<uint8_t> _inst;
    uint32_t _timebase;
    static const uint32_t START_TIMEBASE = 96;
};

#endif /* YKSCONVERTER_h */
