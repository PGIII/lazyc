cmake_minimum_required(VERSION 3.22)
project(PLACEHOLDER_PROJECT_NAME C CXX ASM)

set(CMAKE_COLOR_DIAGNOSTICS ON)
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

add_executable(PLACEHOLDER_PROJECT_NAME 
  src/main.c
)