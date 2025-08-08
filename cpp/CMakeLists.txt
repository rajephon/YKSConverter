cmake_minimum_required (VERSION 3.2)
project (YKSConverter LANGUAGES CXX VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_BUILD_TYPE Release)

set(PUBLIC_HEADER_FILES 
            YKSConverter/ByteBuffer.h
            YKSConverter/YKSConverter.h)

set(PRIVATE_HEADER_FILES
            YKSConverter/MF2TT2MF/TrackEvent.h
            YKSConverter/MF2TT2MF/TrackBuilder.h
            YKSConverter/MF2TT2MF/MF2TT2MF.h)

add_library (YKSConverter STATIC
            YKSConverter/ByteBuffer.cpp
            YKSConverter/MF2TT2MF/TrackEvent.cpp
            YKSConverter/MF2TT2MF/TrackBuilder.cpp
            YKSConverter/MF2TT2MF/MF2TT2MF.cpp
            YKSConverter/YKSConverter.cpp
            ${PUBLIC_HEADER_FILES}
            ${PRIVATE_HEADER_FILES})

set_target_properties(YKSConverter PROPERTIES VERSION ${PROJECT_VERSION})
set_target_properties(YKSConverter PROPERTIES SOVERSION 1)

set_target_properties(YKSConverter PROPERTIES PUBLIC_HEADER
                        "${PUBLIC_HEADER_FILES}")

include_directories(${CMAKE_SOURCE_DIR})
target_include_directories(YKSConverter PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

set(CMAKE_INSTALL_PREFIX "${CMAKE_CURRENT_SOURCE_DIR}/Release")

install(TARGETS YKSConverter
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    PUBLIC_HEADER DESTINATION include)