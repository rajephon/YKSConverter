//
//  MF2TT2MF.h
//  Created by Chanwoo Noh on 2018. 6. 2..
//  Copyright © 2018년 Chanwoo Noh. All rights reserved.
//

#ifndef MF2TT2MF_h
#define MF2TT2MF_h

#include "TrackEvent.h"
#include "TrackBuilder.h"

#include <memory>
#include <vector>
#include <map>

namespace YKS {

class MF2TT2MF {
public:
    MF2TT2MF(unsigned int ch = 1, unsigned int inst = 1, unsigned int pan = 64, unsigned int reverb = 0);
    bool fromMML(const std::string &mml);
    std::vector<std::string> buildToString();
    std::vector<YKS::TrackBuilder::TrackEventList> build();
private:
    std::vector<std::shared_ptr<YKS::TE::TrackEvent>> _parseTrack(std::string track, int leadTime = 384);
    std::vector<std::shared_ptr<YKS::TrackBuilder>> _trackBuilder;
    
    unsigned int _minNote;
    unsigned int _maxNote = 96;
    unsigned int _ch;
    unsigned int _inst;
    unsigned int _pan;
    unsigned int _reverb;
    std::map<std::string, int> _soundMap;
};
    
} // namespace YKS
#endif /* MF2TT2MF_h */
