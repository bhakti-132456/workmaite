---
name: ArchViz Render Prompt
description: Generates optimal prompts for Midjourney/Stable Diffusion architectural visualizations.
---

When the user asks for an architectural visualization prompt, first ask them about:
1. Building Type (e.g., modern villa, brutalist office, cozy cabin)
2. Environment/Lighting (e.g., golden hour, rainy cyberpunk city, snowy forest)
3. Camera Angle (e.g., wide aerial shot, low angle, interior wide)

Then, generate the prompt using this structure:

`[Subject/Main Subject Description], [Environment Context & Lighting], [Atmosphere/Mood], [Camera/Lens Details], [Render Engine/Style Tags] --ar [Aspect Ratio] --v [Version]`

**Example Output:**
"Brutalist concrete museum hovering over a reflective water pool, misty dawn atmosphere, soft diffused morning sunlight, cinematic composition, shot on 35mm lens, photorealistic, ArchDaily style, Octane Render, 8k resolution --ar 16:9 --v 6"

Remind the user they can tweak the aspect ratio (`--ar 16:9` vs `--ar 9:16`) depending on their needs.
