# HotPotAI

[![](https://img.shields.io/github/v/tag/thechampagne/hotpotai-cli?label=version)](https://github.com/thechampagne/hotpotai-cli/releases/latest) [![](https://img.shields.io/github/license/thechampagne/hotpotai-cli)](https://github.com/thechampagne/hotpotai-cli/blob/main/LICENSE)

Hotpot offers AI tools for graphic design image editing, and copywriting. Hotpot services include AI copywriting, background removal, object removal, picture colorization, photo restoration, image upscaling, art personalization, app localization, and more.

### Download

Latest Release: [GitHub Release](https://github.com/thechampagne/hotpotai-cli/releases/latest)

### Usage

```
hotpotai "input" [-options]
        (To spark your writing)
or  hotpotai -BR "path-to-image" "path-to-save" "image-name"
        (Remove the background from images with AI in seconds)
or  hotpotai -PC "path-to-image" [colorization factor] "path-to-save" "image-name"
        (Turning black and white photos to color in seconds)
or  hotpotai -PR "path-to-image" [has scratch] "path-to-save" "image-name"
        (Restore, sharpen, and repair pictures with AI)
where options include:
    --idea
    --summary
    --product-name
    --product-summary
    --song-lyric
    --writing-idea
colorization factor:
    12
    15
    18
    20
    25
has scratch:
    true
    false
```

### License

HotPotAI is released under the [Apache License 2.0](https://github.com/thechampagne/hotpotai-cli/blob/main/LICENSE).

```
 Copyright 2022 XXIV

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
```