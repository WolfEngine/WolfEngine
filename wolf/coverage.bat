OpenCppCoverage.exe --continue_after_cpp_exception --export_type=html:%CD%\coverage\ --sources %CD% --excluded_sources %CD%\build\_deps -- %CD%\build\Debug\wolf_tests.exe