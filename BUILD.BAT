@ECHO OFF
REM rustc picks link.exe from the active VS toolset, so pin the XP-capable v141 (14.16) one.
REM The XP-compatible runtime libs themselves are selected in .cargo\config.toml.
SET "PATH=C:\Program Files (x86)\Microsoft Visual Studio\Installer;%PATH%"
CALL "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" x64 -vcvars_ver=14.16 >NUL || EXIT /B 1

cargo +rust9x build --release --target x86_64-rust9x-windows-msvc || EXIT /B 1
COPY /Y target\x86_64-rust9x-windows-msvc\release\seshanxyz_rust9x.exe cgi-bin\ >NUL
