cmake_minimum_required(VERSION 3.25.0)
Project(day11 LANGUAGES C)
set(CMAKE_C_COMPILER  /usr/bin/gcc)

add_executable(day11 day11.c monkeys.c)
target_include_directories(day11 PUBLIC ${DAY11_SOURCE_DIR})