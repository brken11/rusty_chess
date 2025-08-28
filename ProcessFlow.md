
```mermaid
    flowchart TD
    main --> UiManager
    main --> LogManager
    UiManager --> GuiManager
    GuiManager --> X11Backend
    GuiManager --> Windows_Api
```
