# Script kiddy patch

# PLEASE DO NOT USE THIS FOR CHEATING!

# DO NOT LINK THIS REPO OUTSIDE OF DM's

This is a patch for a library used by [An anime game launcher](https://github.com/an-anime-team/an-anime-game-launcher) and [The Honkers railway launcher](https://github.com/an-anime-team/the-honkers-railway-launcher) that allows running a prelaunch script.

[Upstream](https://github.com/an-anime-team/anime-launcher-sdk)

A patched version of the anime-launcher-sdk for running a pre-launch-script.

This is intended to be used with [GIMI](https://github.com/SilentNightSound/GI-Model-Importer) or [SRMI](https://github.com/SilentNightSound/SR-Model-Importer) however no special feature is available for these mods.

## Installation Instructions
1.  Clone the repository for the launcher that you want to patch
    Fantasy anime game:   [https://github.com/an-anime-team/an-anime-game-launcher.git](https://github.com/an-anime-team/an-anime-game-launcher)
    Honkers railway game: [https://github.com/an-anime-team/the-honkers-railway-launcher.git](https://github.com/an-anime-team/the-honkers-railway-launcher/)

2.  Open the directory for the launcher and then open the files named ``Cargo.toml``
3.  Find the fine that says ``[dependencies.anime-launcher-sdk]``
4.  Below that line there should be one that says ``git = "https://github.com/an-anime-team/anime-launcher-sdk"``
5.  Replace that line with: ``git = "https://github.com/Slimemaster0/anime-launcher-sdk-script-kiddy-patch"`` then save and exit the file
6.  Run ``cargo build --release``
Now there should be a patched binary in ``target/release``

The script must be placed in ``(PATH/TO/GAME/FILES/(Name of game)/run.bat``
The path is typically ``$HOME/.local/share/name-of-launcher``

## Note
1. If you are use 3DMigoto then you will need to add a file called ``d3dcompiler_47.dll`` to the 3DMigoto directory
2. If you use FPS unlocker for the fantasy anime game the script will be called: ``fps_unlocker.bat``
3. This patch is not required for use 3DMigoto with AAGL, but is required for The Honkers Railway Launcher
4. The file named "hsr.bat" is a refrence launch script use by my self for the honkers railway game
