# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.25

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/elmira/src/Advent2022/2022/011

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/elmira/src/Advent2022/2022/011

# Include any dependencies generated for this target.
include CMakeFiles/day11.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/day11.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/day11.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/day11.dir/flags.make

CMakeFiles/day11.dir/day11.c.o: CMakeFiles/day11.dir/flags.make
CMakeFiles/day11.dir/day11.c.o: day11.c
CMakeFiles/day11.dir/day11.c.o: CMakeFiles/day11.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/elmira/src/Advent2022/2022/011/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/day11.dir/day11.c.o"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/day11.dir/day11.c.o -MF CMakeFiles/day11.dir/day11.c.o.d -o CMakeFiles/day11.dir/day11.c.o -c /home/elmira/src/Advent2022/2022/011/day11.c

CMakeFiles/day11.dir/day11.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/day11.dir/day11.c.i"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/elmira/src/Advent2022/2022/011/day11.c > CMakeFiles/day11.dir/day11.c.i

CMakeFiles/day11.dir/day11.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/day11.dir/day11.c.s"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/elmira/src/Advent2022/2022/011/day11.c -o CMakeFiles/day11.dir/day11.c.s

CMakeFiles/day11.dir/monkeys.c.o: CMakeFiles/day11.dir/flags.make
CMakeFiles/day11.dir/monkeys.c.o: monkeys.c
CMakeFiles/day11.dir/monkeys.c.o: CMakeFiles/day11.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/elmira/src/Advent2022/2022/011/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building C object CMakeFiles/day11.dir/monkeys.c.o"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/day11.dir/monkeys.c.o -MF CMakeFiles/day11.dir/monkeys.c.o.d -o CMakeFiles/day11.dir/monkeys.c.o -c /home/elmira/src/Advent2022/2022/011/monkeys.c

CMakeFiles/day11.dir/monkeys.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/day11.dir/monkeys.c.i"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/elmira/src/Advent2022/2022/011/monkeys.c > CMakeFiles/day11.dir/monkeys.c.i

CMakeFiles/day11.dir/monkeys.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/day11.dir/monkeys.c.s"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/elmira/src/Advent2022/2022/011/monkeys.c -o CMakeFiles/day11.dir/monkeys.c.s

CMakeFiles/day11.dir/team.c.o: CMakeFiles/day11.dir/flags.make
CMakeFiles/day11.dir/team.c.o: team.c
CMakeFiles/day11.dir/team.c.o: CMakeFiles/day11.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/elmira/src/Advent2022/2022/011/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building C object CMakeFiles/day11.dir/team.c.o"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/day11.dir/team.c.o -MF CMakeFiles/day11.dir/team.c.o.d -o CMakeFiles/day11.dir/team.c.o -c /home/elmira/src/Advent2022/2022/011/team.c

CMakeFiles/day11.dir/team.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/day11.dir/team.c.i"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/elmira/src/Advent2022/2022/011/team.c > CMakeFiles/day11.dir/team.c.i

CMakeFiles/day11.dir/team.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/day11.dir/team.c.s"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/elmira/src/Advent2022/2022/011/team.c -o CMakeFiles/day11.dir/team.c.s

CMakeFiles/day11.dir/item.c.o: CMakeFiles/day11.dir/flags.make
CMakeFiles/day11.dir/item.c.o: item.c
CMakeFiles/day11.dir/item.c.o: CMakeFiles/day11.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/elmira/src/Advent2022/2022/011/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building C object CMakeFiles/day11.dir/item.c.o"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/day11.dir/item.c.o -MF CMakeFiles/day11.dir/item.c.o.d -o CMakeFiles/day11.dir/item.c.o -c /home/elmira/src/Advent2022/2022/011/item.c

CMakeFiles/day11.dir/item.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/day11.dir/item.c.i"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/elmira/src/Advent2022/2022/011/item.c > CMakeFiles/day11.dir/item.c.i

CMakeFiles/day11.dir/item.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/day11.dir/item.c.s"
	/usr/bin/gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/elmira/src/Advent2022/2022/011/item.c -o CMakeFiles/day11.dir/item.c.s

# Object files for target day11
day11_OBJECTS = \
"CMakeFiles/day11.dir/day11.c.o" \
"CMakeFiles/day11.dir/monkeys.c.o" \
"CMakeFiles/day11.dir/team.c.o" \
"CMakeFiles/day11.dir/item.c.o"

# External object files for target day11
day11_EXTERNAL_OBJECTS =

day11: CMakeFiles/day11.dir/day11.c.o
day11: CMakeFiles/day11.dir/monkeys.c.o
day11: CMakeFiles/day11.dir/team.c.o
day11: CMakeFiles/day11.dir/item.c.o
day11: CMakeFiles/day11.dir/build.make
day11: CMakeFiles/day11.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/elmira/src/Advent2022/2022/011/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Linking C executable day11"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/day11.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/day11.dir/build: day11
.PHONY : CMakeFiles/day11.dir/build

CMakeFiles/day11.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/day11.dir/cmake_clean.cmake
.PHONY : CMakeFiles/day11.dir/clean

CMakeFiles/day11.dir/depend:
	cd /home/elmira/src/Advent2022/2022/011 && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/elmira/src/Advent2022/2022/011 /home/elmira/src/Advent2022/2022/011 /home/elmira/src/Advent2022/2022/011 /home/elmira/src/Advent2022/2022/011 /home/elmira/src/Advent2022/2022/011/CMakeFiles/day11.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/day11.dir/depend

