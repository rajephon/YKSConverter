/**
 Copyright (c) 2018, Chanwoo Noh
 All rights reserved.
 */

#ifndef TrackBuilder_h
#define TrackBuilder_h

#include <vector>
#include <memory>
#include <string>

#include "TrackEvent.h"

namespace YKS {
    class TrackBuilder : public std::enable_shared_from_this<TrackBuilder> {
    public:
        TrackBuilder(int ch) {
            _ch = ch;
        }
        typedef std::vector<std::shared_ptr<YKS::TE::TrackEvent>> TrackEventList;
        
        std::shared_ptr<TrackBuilder> putEvent(std::shared_ptr<YKS::TE::TrackEvent> event);
        std::shared_ptr<TrackBuilder> putEvent(TrackEventList eventList);
        std::vector<std::string> build();
        TrackEventList eventList() { return _eventList; }
    private:
        template<typename Base, typename T>
        inline bool _instanceof(std::shared_ptr<T> ptr) {
            return std::dynamic_pointer_cast<Base>(ptr) != nullptr;
        }
        TrackEventList _eventList;
        unsigned int _ch;
    };
}


#endif /* TrackBuilder_h */
