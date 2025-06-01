# =============================================================================
# Simple Web Audio Player - Makefile
# Copyright Michael Sinz
#
# This Makefile automates various tasks for building, deploying, and testing
# the Simple Web Audio Player components, including:
#  - Converting SVG artwork to PNG files for browser compatibility
#  - Building waveform generation tools in various languages
#  - Deploying to a local music directory and generating waveforms
#  - Setting up a local test server
# =============================================================================

# =============================================================================
# SOURCE AND ARTWORK FILES
# =============================================================================

# Core source files for the web player
SRC=Music.html Music.js Music.css Music.color.css Music.py

# Artwork files that need to be converted from SVG to PNG
ART=Music.ico \
	Blank.png \
	Delete.png \
	EjectButton.png \
	Folder.png \
	Knob.png \
	Music.png \
	NextTrackButton.png \
	PauseButton.png \
	PlayButton.png \
	Playlist.png \
	PrevTrackButton.png \
	Record.png \
	SaveButton.png \
	Speaker.png

# =============================================================================
# CONFIGURATION VARIABLES - CUSTOMIZE THESE FOR YOUR ENVIRONMENT
# =============================================================================

# This is the local directory where you have your music library
# Set this to your own music directory before using localbuild/localwaves
LOCALDIR=~/Music/MP3/

# The version of Waver to use for waveform generation
# Options: Waver-rust (recommended), Waver-swift (macOS only), Waver-c
WAVER=Waver-rust

# Command to open a browser to the test URL
# This may need to be changed depending on your OS:
#   - macOS: use 'open'
#   - Linux: use 'xdg-open'
#   - Windows: use 'start'
OPEN_BROWSER=open

# =============================================================================
# SVG TO PNG CONVERSION
# =============================================================================

# This rule converts SVG files to PNG format for better browser compatibility
# Two options are provided:
#   1. rsvg-convert - Produces smaller files (default)
#   2. resvg - A Rust implementation with growing support
# Uncomment the tool you prefer to use
.SUFFIXES:
.SUFFIXES: .svg .png
.svg.png:
	# resvg --width 256 --height 256 $< $@
	rsvg-convert -w 256 -h 256 -o $@ $<

# =============================================================================
# MAIN BUILD TARGETS
# =============================================================================

# Default target shows help
default: help

# Remove all generated files and build artifacts
clean:
	(cd c; make clean)
	rm -rf *.png Waver-* swift/.build rust/target
	find . -name .DS_Store -delete

# Build just the default waveform generator tool
waver: $(WAVER)
	@true

# Build all waveform generator implementations
waver-all: Waver-swift Waver-rust Waver-c
	@true

# Generate all artwork (SVG to PNG conversion)
art: $(SRC) $(ART)
	@true

# Main build target - builds artwork and default waveform tool
build: art $(WAVER)
	@true

# Build everything - all artwork and all waveform implementations
build-all: art waver-all
	@true

# =============================================================================
# LOCAL DEPLOYMENT TARGETS
# =============================================================================

# Copy the Simple Web Audio Player to the LOCALDIR, including artwork
# Uses rsync to only update files that have changed
localbuild: art
	[ -d $(LOCALDIR) ]
	rsync -cav $(SRC) $(ART) $(LOCALDIR)

# Build waveform PNG files for all MP3 files in the local music directory
# This is fast and only processes files that don't already have waveform PNGs
localwaves: waver
	[ -d $(LOCALDIR) ]
	./$(WAVER) $(LOCALDIR)

# Start a local web server for testing the player with your music library
# Requires the static-web-server tool (install with: cargo install static-web-server)
# Note: This works on Linux/macOS, may need adjustment for Windows
localtest: localbuild localwaves
	which static-web-server
	(killall static-web-server; static-web-server --host 127.0.0.1 --port 8088 --root $(LOCALDIR) & true)
	$(OPEN_BROWSER) http://localhost:8088/Music.html

# =============================================================================
# WAVEFORM GENERATOR IMPLEMENTATIONS
# =============================================================================

# =============================================================================
# Swift implementation - macOS only
# =============================================================================

# Build the Swift version
swift/.build/release/Waver: $(wildcard swift/Sources/*.swift) $(wildcard swift/*.swift)
	(cd swift ; swift build -j $$(sysctl -n hw.ncpu) -c release)

Waver-swift: swift/.build/release/Waver
	cp $< $@
	ls -l $@
	strip $@
	ls -l $@

# =============================================================================
# Rust implementation - cross-platform (recommended)
# =============================================================================

# Run the Rust tests first
rust/target/release/test.log: $(wildcard rust/src/*.rs) $(wildcard rust/src/*/*.rs) rust/Cargo.toml
	mkdir -p rust/target/release
	(cd rust; cargo test --all-features --release -- --nocapture) >$@.err || (cat $@.err; exit 9)
	mv -f $@.err $@
	cat $@

# Build the Rust implementation
rust/target/release/waver: $(wildcard rust/src/*.rs) $(wildcard rust/src/*/*.rs) rust/Cargo.toml rust/target/release/test.log
	(cd rust; cargo build --all-features --release)

Waver-rust: rust/target/release/waver
	cp $< $@
	ls -l $@
	strip $@
	ls -l $@

# =============================================================================
# C implementation - cross-platform (experimental)
# =============================================================================

# Build the C version
c/waver: $(wildcard c/src/*) $(wildcard c/include/*) c/Makefile
	(cd c; make)

Waver-c: c/waver
	cp $< $@
	ls -l $@
	strip $@
	ls -l $@

# =============================================================================
# HELP INFORMATION
# =============================================================================

help:
	@echo "make art        - Build all PNG images from SVG"
	@echo "make waver      - Build $(WAVER)"
	@echo "make build      - Build PNG images and $(WAVER)"
	@echo
	@echo "make waver-all  - Build all Waver variants"
	@echo "make build-all  - Build PNG images and all Waver variants"
	@echo
	@echo "make localbuild - Build web site to $(LOCALDIR)"
	@echo "make localwaves - Build wave PNG files in $(LOCALDIR)"
	@echo "make localtest  - Start simple test server on $(LOCALDIR)"
	@echo