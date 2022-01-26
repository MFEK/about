# MFEKabout

![](/doc/Screenshot.png)

Note: This program generates the about screen. For general information about the project, see the [docs](https://github.com/MFEK/docs/) repository.

MFEK about screen. Uses Skulpin like MFEKglif.

The animation is pre-rendered by Blender. The Blender files then just have their paths taken out by `src/gen_resources.py`, which prepares them to be called with `skia::Path::from_svg`.
