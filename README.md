# Todo App

Example app for learning.

## Dependencies

### Fedora

```bash
sudo dnf install gtk4-devel libadwaita-devel meson desktop-file-utils gcc glib-compile-resources gtk4-update-icon-cache update-desktop-database
```

### Debian

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev meson desktop-file-utils gcc gtk-update-icon-cache
```

### Arch

```bash
sudo pacman -S gtk4 libadwaita meson desktop-file-utils gcc
```

## Persistence 

```bash
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp ./resources/org.gtk_rs.Todo1.gschema.xml $HOME/.local/share/glib-2.0/schemas/
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/

```
