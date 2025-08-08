/**
 Copyright (c) 2018, Chanwoo Noh
 All rights reserved.
 */

#include <sstream>
#include <iomanip>

#include "TrackEvent.h"

namespace YKS {
namespace TE {
    
// MARK: - TrackEvent
unsigned int TrackEvent::leadTime() {
    return _time;
}

std::shared_ptr<TrackEvent> TrackEvent::leadTime(unsigned int time) {
    _time = time;
    return shared_from_this();
}
    
std::shared_ptr<ByteBuffer> TrackEvent::toBuffer() {
    return toBuffer(std::make_shared<ByteBuffer>());
}
    
// MARK: - SeqSpec
SeqSpec::SeqSpec(std::vector<uint8_t> value) {
    _time = 0;
    _value = value;
}
std::string SeqSpec::value() {
    std::stringstream ss;
    ss << "SeqSpec ";
    ss << std::hex;
    for (int i = 0; i < _value.size(); i++) {
        if (_value.at(i) < 10) {
            ss << "0";
        }
        ss << (int)_value.at(i);
        if (i < _value.size()-1) {
            ss << " ";
        }
    }
    return ss.str();
}
std::shared_ptr<ByteBuffer> SeqSpec::toBuffer(std::shared_ptr<ByteBuffer> buffer) {
    buffer->putByte(0xff)
    ->putByte(0x7f)
    ->putByte(_value.size());
    for (const auto &v : _value) {
        buffer->putByte(v);
    }
    return buffer;
};

// MARK: - MetaText
MetaText::MetaText(const std::string &value) {
    _time = 0;
    _value = value;
}
std::string MetaText::value() {
    return "Meta Text \"" + _value + "\"";
}
std::shared_ptr<ByteBuffer> MetaText::toBuffer(std::shared_ptr<ByteBuffer> buffer) {
    buffer->putByte(0xff);
    buffer->putByte(0x01); // meta type
    buffer->putByte(_value.length());
    buffer->putString(_value);
    return buffer;
}
// MARK: - Tempo
Tempo::Tempo(unsigned int value) {
    _time = 0;
    _value = value;
}
std::string Tempo::value() {
    return "Tempo " + std::to_string(_value);
}
std::shared_ptr<ByteBuffer> Tempo::toBuffer(std::shared_ptr<ByteBuffer> buffer) {
    buffer->putByte(0xff)->putByte(0x51)->putByte(0x03);
    buffer->putByte((_value >> (2 * 8)) & 0x00ff);
    buffer->putByte((_value >> (1 * 8)) & 0x00ff);
    buffer->putByte(_value & 0x00ff);
    return buffer;
}
// MARK: - SysEx
SysEx::SysEx(std::vector<uint8_t> value) {
    _time = 0;
    _value = value;
}
std::string SysEx::value() {
    std::stringstream ss;
    ss << "SysEx ";
    ss << std::hex;
    for (int i = 0; i < _value.size(); i++) {
        if (_value.at(i) < 10) {
            ss << "0";
        }
        ss << (int)_value.at(i);
        if (i < _value.size()-1) {
            ss << " ";
        }
    }
    return ss.str();
}
std::shared_ptr<ByteBuffer> SysEx::toBuffer(std::shared_ptr<ByteBuffer> buffer) {
    if (_value.at(0) != 0xf0) {
        std::cerr << "SysEx's first value msut equal 0xf0";
        return buffer;
    }
    buffer->putByte(0xf0);
    buffer->putByte(_value.size()-1);
    for (int i = 1; i < _value.size(); i++) {
        buffer->putByte(_value.at(i));
    }
    return buffer;
}
    
// MARK: - PrCh
PrCh::PrCh(uint8_t ch, uint8_t p) {
    _ch = ch;
    _p = p;
}
std::string PrCh::value() {
    return "PrCh ch=" + std::to_string(_ch) + " p=" + std::to_string(_p);
}
std::shared_ptr<ByteBuffer> PrCh::toBuffer(std::shared_ptr<ByteBuffer> buffer) {
    buffer->putByte(0xc0 + _ch - 1);
    buffer->putByte(_p);
    return buffer;
}

// MARK: - Par
Par::Par(uint8_t ch, uint8_t c, uint8_t v) {
    _ch = ch;
    _c = c;
    _v = v;
}
std::string Par::value() {
    return "Par ch="+std::to_string(_ch)+" c="+std::to_string(_c)+" v="+std::to_string(_v);
}
std::shared_ptr<ByteBuffer> Par::toBuffer(std::shared_ptr<ByteBuffer> buffer) {
    buffer->putByte(0xb0 + _ch - 1);
    buffer->putByte(_c);
    buffer->putByte(_v);
    return buffer;
}
// MARK: - On
On::On(uint8_t ch, uint8_t c, uint8_t v) {
    _ch = ch;
    _c = c;
    _v = v;
}
std::string On::value() {
    return "On ch="+std::to_string(_ch)+" n="+std::to_string(_c)+" v="+std::to_string(_v);
}
std::shared_ptr<ByteBuffer> On::toBuffer(std::shared_ptr<ByteBuffer> buffer) {
    buffer->putByte(0x90 + _ch - 1);
    buffer->putByte(_c);
    buffer->putByte(_v);
    return buffer;
}
// MARK: - Off
Off::Off(uint8_t ch, uint8_t c, uint8_t v) {
    _ch = ch;
    _c = c;
    _v = v;
}
std::string Off::value() {
    return "Off ch="+std::to_string(_ch)+" n="+std::to_string(_c)+" v="+std::to_string(_v);
}
std::shared_ptr<YKS::ByteBuffer> Off::toBuffer(std::shared_ptr<YKS::ByteBuffer> buffer) {
    buffer->putByte(0x80+_ch-1);
    buffer->putByte(_c);
    buffer->putByte(_v);
    return buffer;
}
    
// MARK: - Meta TrkEnd
std::string MetaTrkEnd::value() {
    return "Meta TrkEnd";
}
std::shared_ptr<YKS::ByteBuffer> MetaTrkEnd::toBuffer(std::shared_ptr<YKS::ByteBuffer> buffer) {
    buffer->putByte(0xff);
    buffer->putByte(0x2f);
    buffer->putByte(0x00);
    return buffer;
}

} // namespace TE;
} // namespace YKS;




