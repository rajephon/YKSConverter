/**
 Copyright (c) 2018, Chanwoo Noh
 All rights reserved.
 */

#ifndef TrackEvent_h
#define TrackEvent_h

#include <string>

#include "../ByteBuffer.h"

namespace YKS {
namespace TE {

class TrackEvent : public std::enable_shared_from_this<TrackEvent> {
public:
    unsigned int leadTime();
    
    virtual std::string value() = 0;
    std::shared_ptr<TrackEvent> leadTime(unsigned int time);
    virtual std::shared_ptr<ByteBuffer> toBuffer();
    virtual std::shared_ptr<ByteBuffer> toBuffer(std::shared_ptr<ByteBuffer> buffer) { return nullptr; }
    
protected:
    unsigned int _time = 0;
};
    
class SeqSpec : public TrackEvent {
public:
    /*
     * value : e.g. 00 00 41
     */
    SeqSpec(std::vector<uint8_t> value);
    std::string value() override;
    /**
     * reference: http://midi.teragonaudio.com/tech/midifile.htm
     **/
    std::shared_ptr<ByteBuffer> toBuffer(std::shared_ptr<ByteBuffer> buffer) override;
private:
    std::vector<uint8_t> _value;
};
    
class MetaText : public TrackEvent {
public:
    MetaText(const std::string &value);
    std::string value() override;
    std::shared_ptr<ByteBuffer> toBuffer(std::shared_ptr<ByteBuffer> buffer) override;
private:
    std::string _value;
};
    
class Tempo : public TrackEvent {
public:
    Tempo(unsigned int value);
    std::string value() override;
    /**
     * FF 51 03 tt tt tt
     * reference: http://midi.teragonaudio.com/tech/midifile/tempo.htm
     **/
    std::shared_ptr<ByteBuffer> toBuffer(std::shared_ptr<ByteBuffer> buffer) override;
private:
    unsigned int _value;
};
    
class SysEx : public TrackEvent {
public:
    /*
     * vlaue: e.g. f0 41 10 42 12 40 00 7f 00 41 f7
     */
    SysEx(std::vector<uint8_t> value);
    std::string value() override;
    std::shared_ptr<ByteBuffer> toBuffer(std::shared_ptr<ByteBuffer> buffer) override;
private:
    std::vector<uint8_t> _value;
};
    
class PrCh : public TrackEvent {
public:
    /**
     * ch: channel
     * p: inst
     */
    PrCh(uint8_t ch, uint8_t p);
    std::string value() override;
    /**
     * reference: http://midi.teragonaudio.com/tech/midispec.htm
     */
    std::shared_ptr<ByteBuffer> toBuffer(std::shared_ptr<ByteBuffer> buffer) override;
private:
    uint8_t _ch;
    uint8_t _p;
};
    
class Par : public TrackEvent {
public:
    Par(uint8_t ch, uint8_t c, uint8_t v);
    std::string value() override;
private:
    unsigned int _ch;
    unsigned int _c;
    unsigned int _v;
    std::shared_ptr<ByteBuffer> toBuffer(std::shared_ptr<ByteBuffer> buffer) override;
};
    
class On : public TrackEvent {
public:
    On(uint8_t ch, uint8_t c, uint8_t v);
    std::string value() override;
    std::shared_ptr<ByteBuffer> toBuffer(std::shared_ptr<ByteBuffer> buffer) override;
private:
    unsigned int _ch;
    unsigned int _c;
    unsigned int _v;
};
    
class Off : public TrackEvent {
public:
    Off(uint8_t ch, uint8_t c, uint8_t v = 0);
    std::string value() override;
    std::shared_ptr<YKS::ByteBuffer> toBuffer(std::shared_ptr<YKS::ByteBuffer> buffer) override;
    uint8_t note() { return _c; }
private:
    unsigned int _ch;
    unsigned int _c;
    unsigned int _v;
};
    
class MetaTrkEnd : public TrackEvent {
public:
    MetaTrkEnd() = default;
    std::string value() override;
    std::shared_ptr<YKS::ByteBuffer> toBuffer(std::shared_ptr<YKS::ByteBuffer> buffer) override;
};
    
} // namespace TE;
} // namespace YKS;

#endif /* TrackEvent_h */
