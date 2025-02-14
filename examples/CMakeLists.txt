# Copyright © SixtyFPS GmbH <info@sixtyfps.io>
# SPDX-License-Identifier: (GPL-3.0-only OR LicenseRef-SixtyFPS-commercial)
cmake_minimum_required(VERSION 3.19)
project(SixtyFPSExamples LANGUAGES CXX)

if (NOT TARGET SixtyFPS::SixtyFPS)
    find_package(SixtyFPS REQUIRED)
    include(FetchContent)
endif()

add_subdirectory(printerdemo/cpp/)
if (SIXTYFPS_FEATURE_INTERPRETER)
    add_subdirectory(printerdemo/cpp_interpreted/)
endif()
add_subdirectory(printerdemo_old/cpp/)
add_subdirectory(todo/cpp/)
add_subdirectory(gallery/)
add_subdirectory(memory/)
if (SIXTYFPS_FEATURE_INTERPRETER AND SIXTYFPS_FEATURE_BACKEND_QT)
    add_subdirectory(qt_viewer/)
endif()
if (SIXTYFPS_FEATURE_INTERPRETER)
    add_subdirectory(iot-dashboard/)
endif()
