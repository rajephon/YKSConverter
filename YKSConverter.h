/**
 Copyright (c) 2018, Chanwoo Noh
 All rights reserved.
 */

#ifndef __YKSCONVERTER__H__
#define __YKSCONVERTER__H__

#include <string>
#include <memory>

#include "ByteBuffer.h"

class YKSConverter : public std::enable_shared_from_this<YKSConverter> {
public:
    
    YKSConverter(const std::string &mml = "") {
        _mml = mml;
        _timebase = 480;
        _timebase = 96;
    }
    
    void mml(const std::string &mml) { _mml = mml; }
    std::string mml() { return _mml; }
    
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

#endif /* defined(__YKSCONVERTER__H__) */
