
rem START /B /wait cmd /c "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\Blacklist_game.exe"

@REM cargo build -p hooks --target i686-pc-windows-msvc --release
cargo build -p hooks --target i686-pc-windows-msvc
if %errorlevel% neq 0 exit /b %errorlevel%

taskkill /F /IM Blacklist_game.exe
@REM copy .\target\i686-pc-windows-msvc\release\hooks.dll "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\uplay_r1_loader.dll"
copy .\target\i686-pc-windows-msvc\debug\hooks.dll "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\uplay_r1_loader.dll"
copy .\target\i686-pc-windows-msvc\release\hooks.pdb "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\uplay_r1_loader.pdb"