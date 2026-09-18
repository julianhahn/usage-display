# Project instructions

## Architecture documentation

Keep architecture sources and human-facing rendered views separate:

- `docs/architecture/source/` contains all LikeC4 source files and machine-readable architecture specifications. The AI reads and edits these files.
- `docs/architecture/view/` contains rendered HTML diagrams and other human-facing outputs. Humans open these files to explore the architecture.

When architecture changes, update the source first. Then regenerate the matching human-facing view.

Do not put generated HTML next to the LikeC4 source files.
