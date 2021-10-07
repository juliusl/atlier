Project(atlier)
cmake_minimum_required(VERSION 3.21)
if(WIN32) #to mingw work as all the others
set(CMAKE_SHARED_LIBRARY_PREFIX "")
endif(WIN32)
#general settings

if (EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/../deps/imgui/backends)
	set(BAKENDS_FOLDER "../deps/imgui/backends/")
else()
	set(BAKENDS_FOLDER "../deps/imgui/examples/")
endif()

if (EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/../deps/imgui/imgui_tables.cpp)
	set(TABLES_SOURCE "../deps/imgui/imgui_tables.cpp")
else()
	set(TABLES_SOURCE "")
endif()

include_directories(../deps/cimgui/)
include_directories(../deps/cimnodes/)
include_directories(../deps/imgui/)
include_directories(../deps/imnodes/)
add_definitions("-DIMGUI_DISABLE_OBSOLETE_FUNCTIONS=1")

include_directories(../)
set(IMGUI_SOURCES ../deps/cimgui/cimgui.cpp 
../deps/imgui/imgui.cpp 
../deps/imgui/imgui_draw.cpp 
../deps/imgui/imgui_demo.cpp 
../deps/imgui/imgui_widgets.cpp
../deps/cimnodes/cimnodes.cpp
../deps/imnodes/imnodes.cpp
${TABLES_SOURCE} 
)

set(IMGUI_SOURCES_sdl)
set(IMGUI_LIBRARIES )

if (WIN32)
    add_definitions("-DIMGUI_IMPL_API=extern \"C\" __declspec\(dllexport\)")
else(WIN32)
    add_definitions("-DIMGUI_IMPL_API=extern \"C\" ")
endif(WIN32)

add_compile_definitions("IMGUI_IMPL_OPENGL_LOADER_GL3W")

# optional adding freetype
option(IMGUI_FREETYPE "add Freetype2" OFF)

if(IMGUI_FREETYPE)
	FIND_PACKAGE(freetype REQUIRED PATHS ${FREETYPE_PATH})
	list(APPEND IMGUI_LIBRARIES freetype)
	list(APPEND IMGUI_SOURCES ../deps/imgui/misc/freetype/imgui_freetype.cpp)
	add_definitions("-DCIMGUI_FREETYPE=1")
endif(IMGUI_FREETYPE)

# Setup OpenGL3
list(APPEND IMGUI_SOURCES ${BAKENDS_FOLDER}imgui_impl_opengl3.cpp)
list(APPEND IMGUI_SOURCES ./cimgui_extras.cpp)
include_directories(../deps/imgui/examples/libs/gl3w)
if(WIN32)
       list(APPEND IMGUI_LIBRARIES opengl32)
else(WIN32)#Unix
       list(APPEND IMGUI_LIBRARIES GL)
endif(WIN32)


# Setup SDL2
list(APPEND IMGUI_SOURCES ${BAKENDS_FOLDER}imgui_impl_sdl.cpp)
if(DEFINED SDL_PATH)
	message(STATUS "SDL_PATH defined as " ${SDL_PATH})
	FIND_PACKAGE(SDL2 REQUIRED PATHS ${SDL_PATH})
	get_target_property(SDL_INCLUDE SDL2::SDL2 INTERFACE_INCLUDE_DIRECTORIES)
	message(STATUS "sdlinclude is " ${SDL_INCLUDE})
	if ("${SDL_INCLUDE}" STREQUAL "" OR "${SDL_INCLUDE}" STREQUAL "SDL_INCLUDE-NOTFOUND") #if not found latest SDL2 cmake config use older
		message(STATUS "sdlinclude2 is " ${SDL2_INCLUDE_DIRS})
		include_directories(${SDL2_INCLUDE_DIRS})
		set(IMGUI_SDL_LIBRARY ${SDL2_LIBRARIES})
		message(STATUS IMGUI_SDL_LIBRARY ${SDL2_LIBRARIES})
	else()#use new one SDL2 config
		include_directories(${SDL_INCLUDE})
		set(IMGUI_SDL_LIBRARY SDL2::SDL2)
		set(SDL_MAIN SDL2::SDL2main)
		message(STATUS ${SDL_MAIN} ${IMGUI_SDL_LIBRARY})
	endif()
else(DEFINED SDL_PATH)
	message(STATUS "SDL_PATH not defined")
	set(IMGUI_SDL_LIBRARY SDL2)
endif(DEFINED SDL_PATH)

add_library(cimgui_sdl SHARED ${IMGUI_SOURCES})
target_link_libraries(cimgui_sdl ${IMGUI_LIBRARIES} ${IMGUI_SDL_LIBRARY})

# Create Executable
include_directories(../deps/cimgui/generator/output/)
add_executable(atlier main.c)
if (MINGW)
target_link_options(atlier PRIVATE "-mconsole")
endif()
target_link_libraries(atlier ${IMGUI_SDL_LIBRARY} cimgui_sdl)

