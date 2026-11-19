# SFX Replacement Tool

[![Boosty](https://img.shields.io/badge/Boosty-donate-orange.svg)](https://boosty.to/bright-shards/donate)

Tool for managing VO and SFX replacement, formerly for Deadlock, but it is suitable for other purposes.
Made it to solve problem with replacing 1000+ VO files with hands.

<img width="1637" height="861" alt="James Raynor over Vindicta" src="https://github.com/user-attachments/assets/d9c11bd4-6dfa-4dd2-aa61-79d719a227ec" />

## Why not other solutions?

All i saw as alternatives is tools mostly for single file replacement, and batch option there one is not really comfy, since it is WEBAPPS that is not working with local files.

I prefer old fashion way - export VO files your self and match them with your ripped donor files, then you'll have \[project name\].vo.json and built up results folder.

## WIP
Be careful, i am planning on adding versions for .vo.json, so it possibly can be broken after updates, but i'll try to make sure to make migrations for this (maybe even backwards).

## TODO
- [x] Mouse move selection (LMB on track then move over tracks you want to select)
- [x] Range selections (RMB to extend selection from last clicked track in same folder to clicked one)
- [x] UI optimizations (Replacing simple array with Set made mouse move selection MUCH smoother)
- [ ] Tracks prefabs and reencoding (ogg, opus, wav, flac support using FFMPEG for filters like volume change and cutting tracks). Will require a new schema of project.vo.json and migration too.
