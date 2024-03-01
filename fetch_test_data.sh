# Fetches test roms and success states
# Only needs to be ran once

wget https://github.com/c-sp/gameboy-test-roms/releases/download/v7.0/game-boy-test-roms-v7.0.zip -O test_data.zip;
unzip test_data.zip -d test_data;
rm test_data.zip