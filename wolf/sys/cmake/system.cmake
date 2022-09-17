# fetch gsl
message("fetching https://github.com/microsoft/GSL.git")
FetchContent_Declare(
  gsl
  GIT_REPOSITORY https://github.com/microsoft/GSL.git
  GIT_TAG        main
  SOURCE_SUBDIR  include
)
FetchContent_Populate(gsl)
list(APPEND INCLUDES ${CMAKE_CURRENT_BINARY_DIR}/_deps/gsl-src/include/)

# fetch mimalloc
message("fetching https://github.com/microsoft/mimalloc.git")
FetchContent_Declare(
    mimalloc-static
    GIT_REPOSITORY https://github.com/microsoft/mimalloc.git
    GIT_TAG        master
)
set(MI_BUILD_SHARED OFF CACHE BOOL "MI_BUILD_SHARED")
set(MI_BUILD_TESTS OFF CACHE BOOL "MI_BUILD_TESTS")

FetchContent_MakeAvailable(mimalloc-static)
list(APPEND INCLUDES ${mimalloc-static_SRC_DIR}/include)
list(APPEND LIBS mimalloc-static)  

# fetch lz4
if (WOLF_SYSTEM_LZ4)
  message("fetching https://github.com/lz4/lz4.git")
  FetchContent_Declare(
    lz4_static
    GIT_REPOSITORY https://github.com/lz4/lz4.git
    GIT_TAG        dev
    SOURCE_SUBDIR  build/cmake
  )
  FetchContent_MakeAvailable(lz4_static)

  file(GLOB_RECURSE LZ4_SRCS
    "${CMAKE_CURRENT_SOURCE_DIR}/system/lz4.h"
    "${CMAKE_CURRENT_SOURCE_DIR}/system/lz4.cpp"
  )
  list(APPEND SRCS ${LZ4_SRCS})
  list(APPEND INCLUDES ${lz4_SRC_DIR}/include)
  list(APPEND LIBS lz4_static)    
endif()