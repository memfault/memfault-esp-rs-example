cmake_minimum_required(VERSION 3.13.1)

project(memfault-firmware-sdk)

set(MEMFAULT_SDK_ROOT ${CMAKE_CURRENT_LIST_DIR}/external/memfault-firmware-sdk)
set(MEMFAULT_PORT_ROOT ${CMAKE_CURRENT_LIST_DIR}/memfault)

message(SDK_ROOT="${MEMFAULT_SDK_ROOT}")
list(APPEND MEMFAULT_COMPONENTS "core" "panic")
include(${MEMFAULT_SDK_ROOT}/cmake/Memfault.cmake)
memfault_library(${MEMFAULT_SDK_ROOT} MEMFAULT_COMPONENTS
 MEMFAULT_COMPONENTS_SRCS MEMFAULT_COMPONENTS_INC_FOLDERS ARCH_XTENSA)

# ${MEMFAULT_COMPONENTS_SRCS} contains the sources

include_directories(include ${MEMFAULT_COMPONENTS_INC_FOLDERS} ${MEMFAULT_PORT_ROOT})
message(SRCS="${MEMFAULT_COMPONENTS_SRCS}")
add_library(memfault-firmware-sdk STATIC ${MEMFAULT_COMPONENTS_SRCS})
install(TARGETS memfault-firmware-sdk LIBRARY DESTINATION lib)
