/**
 Copyright (c) 2018, Chanwoo Noh
 All rights reserved.
 */

#include "TrackBuilder.hpp"
namespace YKS {
    
std::shared_ptr<TrackBuilder> TrackBuilder::putEvent(std::shared_ptr<TE::TrackEvent> event)  {
    _eventList.push_back(event);
    return shared_from_this();
}

std::shared_ptr<TrackBuilder> TrackBuilder::putEvent(std::vector<std::shared_ptr<TE::TrackEvent>> eventList) {
    _eventList.insert(_eventList.end(), eventList.begin(), eventList.end());
    return shared_from_this();
}

std::vector<std::string> TrackBuilder::build() {
    std::vector<std::string> track;
    track.push_back("MTrk");
    auto deltaTime = 0;
    for (const auto &event : _eventList) {
        deltaTime = event->leadTime();
        auto line = std::to_string(deltaTime) + " " + event->value();
        track.push_back(line);
    }
    track.push_back("TrkEnd");
    return track;
}
    
} // namespace YKS;
