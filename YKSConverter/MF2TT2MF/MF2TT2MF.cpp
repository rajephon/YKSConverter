//
//  MF2TT2MF.cpp
//  Created by Chanwoo Noh on 2018. 6. 2..
//  Copyright © 2018년 Chanwoo Noh. All rights reserved.
//

#include "MF2TT2MF.h"
#include <regex>
#include <iostream>
#include <math.h>

#define REGEX_MML "(MML@)\\s*([\\s0-9a-glnortvA-GLNORTV#<>.&+-]*),\\s*([\\s0-9a-glnortvA-GLNORTV#<>.&+-]*),\\s*([\\s0-9a-glnortvA-GLNORTV#<>.&+-]*);"
#define MIN_VOLUME 1
#define MAX_VOLUME 15
#define MAX_OCTAVE 9

namespace YKS {

MF2TT2MF::MF2TT2MF(unsigned int ch, unsigned int inst, unsigned int pan, unsigned int effect) {
    _ch = ch;
    _inst = inst;
    _pan = pan;
    _effect = effect;
    
    _minNote = 0;
    _maxNote = 96;
    
    _soundMap["c"] = 0;
    _soundMap["d"] = 2;
    _soundMap["e"] = 4;
    _soundMap["f"] = 5;
    _soundMap["g"] = 7;
    _soundMap["a"] = 9;
    _soundMap["b"] = 11;
    _soundMap["C"] = 0;
    _soundMap["D"] = 2;
    _soundMap["E"] = 4;
    _soundMap["F"] = 5;
    _soundMap["G"] = 7;
    _soundMap["A"] = 9;
    _soundMap["B"] = 11;
}

bool MF2TT2MF::fromMML(const std::string &mmls) {
    _trackBuilder.clear();
    std::regex mmlsParseRegex(REGEX_MML);
    std::smatch smatchMML;
    if (!std::regex_search(mmls, smatchMML, mmlsParseRegex)) {
        // MARK: fail regex parsing
        std::cerr << "Regex parse failed" << std::endl;
        return false;
    }
    std::vector<std::string> trackList;
    
    // 각 트랙 별로 분리
    for (size_t i = 0; i < smatchMML.size(); ++i) {
        std::ssub_match sub_match = smatchMML[i];
        std::string track = sub_match.str();
        if (track.substr(0, 4) != "MML@") {
            trackList.push_back(track);
        }
    }
    if (trackList.size() <= 0) {
        std::cerr << "Track is empty" << std::endl;
        return false;
    }
    
    auto ch = _ch;
    auto inst = _inst;
    auto pan = _pan;
    auto effect = _effect;
    for (int i = 0; i < trackList.size(); i++) {
        auto builder = std::make_shared<YKS::TrackBuilder>(ch);
        if (ch == 1 && i == 0) { // 첫 번째 채널, 첫 번째 트랙에만 헤더를 붙여준다.
            std::vector<uint8_t> sysEx = {0xf0, 0x41, 0x10, 0x42, 0x12, 0x40, 0x00, 0x7f, 0x00, 0x41, 0xf7};
            builder->putEvent(std::make_shared<YKS::TE::MetaText>("Yokoso Project(https://yoko.so/)"))
            ->putEvent(std::make_shared<YKS::TE::Tempo>(500000))
            ->putEvent(std::make_shared<YKS::TE::SysEx>(sysEx));
        }
        builder->putEvent(std::make_shared<YKS::TE::PrCh>(ch, inst)->leadTime(192))
        ->putEvent(std::make_shared<YKS::TE::Par>(ch, 10, pan)->leadTime(193))
        ->putEvent(std::make_shared<YKS::TE::Par>(ch, 91, effect)->leadTime(194));
        
        if (!trackList.at(i).empty()) {
            builder->putEvent(_parseTrack(trackList.at(i), 384));
        }else {
            builder->putEvent(std::make_shared<YKS::TE::MetaTrkEnd>()->leadTime(385));
        }
        _trackBuilder.push_back(builder);
    }
    
    return true;
}

std::vector<std::string> MF2TT2MF::buildToString() {
    std::vector<std::string> mf2tt2mf;
    mf2tt2mf.push_back("MFile 1 " + std::to_string(_ch) + " 96");
    for (const auto &builder : _trackBuilder) {
        auto build = builder->build();
        mf2tt2mf.insert(mf2tt2mf.end(), build.begin(), build.end());
    }
    return mf2tt2mf;
}
std::vector<YKS::TrackBuilder::TrackEventList> MF2TT2MF::build() {
    std::vector<std::vector<std::shared_ptr<YKS::TE::TrackEvent>>> trackEventList;
    for (const auto &builder : _trackBuilder) {
        trackEventList.push_back(builder->eventList());
    }
    return trackEventList;
}

std::vector<std::shared_ptr<YKS::TE::TrackEvent>> MF2TT2MF::_parseTrack(std::string track, int deltaTime) {
    std::vector<std::shared_ptr<YKS::TE::TrackEvent>> eventList;
    
    // remove line breaks, white space.
    std::regex rgxFilter("[\\s|\\r\\n|\\r|\\n]+");
    track = std::regex_replace(track, rgxFilter, "");
    std::regex rgxEvent("[OTLVNRA-Gotlvnra-g<>][\\+\\-\\#]?[0-9]*\\.?&?");
    auto rgxIt = std::sregex_iterator(track.begin(), track.end(), rgxEvent);
    auto rgxEnd = std::sregex_iterator();
    
    int noteTime = 96;
    int octave = 4;
    int volume = 8;
    int currNote = 0;
    bool isTied = false;
    int semibreve = noteTime * 4;   // 온음표
    int minim = noteTime * 2;       // 2분음표
    
    while (rgxIt != rgxEnd) {
        // process
        std::string event = (*rgxIt).str();
        int value = 0;
        std::smatch regexExp;
        
        // parse mml event
        if (std::regex_search(event, regexExp, std::regex("([lotvLOTV<>])([1-9][0-9]*|0?)(\\.?)(&?)"))) {
            std::string op = regexExp[1].str();
            value = std::atoi(regexExp[2].str().c_str());
            std::string dot = regexExp[3].str();
            std::string ampersand = regexExp[4].str();
            if (op == "l" || op =="L") {
                if (value > 0 && value <=minim) {
                    noteTime = floor(semibreve/value);
                    if (dot == ".") {
                        noteTime = floor(noteTime * 1.5f);
                    }
                    if (ampersand == "&") {
                        isTied = true;
                        // d+8l16&d+ 과 같이 사이에 'l'이 낀 상황 처리.
                        // 마지막 Off 노트를 꺼내서, currNote에 n을 적고 버린다.
                        for (int i = (int)eventList.size() - 1; i > 0; i--) {
                            auto off = std::dynamic_pointer_cast<YKS::TE::Off>(eventList.at(i));
                            if (off != nullptr) {
                                eventList.erase(eventList.begin()+i);
                                currNote = (int)off->note();
                                break;
                            }
                        }
                    }
                }
            }else if (op == "o" || op == "O") {
                octave = value;
            }else if (op == "t" || op == "T") {
                int tempo = 0;
                tempo = floor(60000000 / value);
                eventList.push_back(std::make_shared<YKS::TE::Tempo>(tempo)->leadTime(deltaTime));
            }else if (op == "v" || op == "V") {
                if (value < MIN_VOLUME) {
                    value = MIN_VOLUME;
                }else if (value > MAX_VOLUME) {
                    value = MAX_VOLUME;
                }
                volume = value;
            }else if (op == "<") {
                if (octave <= 0) {
                    octave = 0;
                }else {
                    octave -= 1;
                }
            }else if (op == ">") {
                if (octave >= MAX_OCTAVE) {
                    octave = MAX_OCTAVE;
                }else {
                    octave += 1;
                }
            }
        }else if (std::regex_search(event, regexExp, std::regex("([a-gnA-GN])([\\+\\#-]?)([0-9]*)(\\.?)(&?)"))) {
            int note = 0;
            int tick = noteTime;
            std::string op = regexExp[1].str();
            std::string pitch = regexExp[2].str();
            value = std::atoi(regexExp[3].str().c_str());
            std::string dot = regexExp[4].str();
            std::string ampersand = regexExp[5].str();
            if (op == "n" || op == "N") {
                if (value >= 0 && value <= _maxNote) {
                    note = value;
                }
            }else {
                if (value >= 1 && value <= minim) {
                    tick = floor(semibreve / value);
                }
                if (dot == ".") {
                    tick = floor(tick*1.5);
                }
                
                if (_soundMap.count(op)) {
                    note = (12 * octave) + _soundMap[op];
                }
                if (pitch == "+" || pitch == "#") {
                    note += 1;
                }else if (pitch == "-") {
                    note -= 1;
                }
            }
            
            while (note < _minNote) {
                note += 12;
            }
            while (note > _maxNote) {
                note -= 12;
            }
            note += 12;
            
            // c&d 와 같은 다른 음 이음줄 처리
            if (isTied && note != currNote) {
                isTied = false;
                eventList.push_back(std::make_shared<YKS::TE::Off>(_ch, currNote)->leadTime(deltaTime));
            }
            if (!isTied) {
                eventList.push_back(std::make_shared<YKS::TE::On>(_ch, note, 8*volume)->leadTime(deltaTime));
            }
            deltaTime += tick;
            if (ampersand == "&") {
                isTied = true;
                currNote = note;
            }else {
                isTied = false;
                eventList.push_back(std::make_shared<YKS::TE::Off>(_ch, note)->leadTime(deltaTime));
            }
        }else if (isTied) {
            isTied = false;
            eventList.push_back(std::make_shared<YKS::TE::Off>(_ch, currNote)->leadTime(deltaTime));
        }
        if (std::regex_search(event, regexExp, std::regex("[rR]([0-9]*)(\\.?)"))) {
            int tick = noteTime;
            int value = std::atoi(regexExp[1].str().c_str());
            std::string dot = regexExp[2].str();
            if (value >= 1 && value <= minim) {
                tick = floor(semibreve/value);
            }
            if (dot == ".") {
                tick = floor(tick*1.5);
            }
            deltaTime += tick;
        }
        rgxIt++;
    }
    deltaTime += noteTime;
    eventList.push_back(std::make_shared<YKS::TE::MetaTrkEnd>()->leadTime(deltaTime));
    return eventList;
}

} // namespace YKS
