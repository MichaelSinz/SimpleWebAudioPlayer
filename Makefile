# Simple Web Audio Player - Copyright Michael Sinz

SRC=Music.html Music.js Music.css Music.color.css Music.py

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

# This is the local directory where you have your music library
# On my machine I have my MP3 library in here.  This is also where I
# publish the Music web site code to.  Set this to your directory.
LOCALDIR=~/Music/MP3/

# The version of Waver I use to actually build the PNG waveform files
WAVER=Waver-rust

# The "open" command is how I start the browser on the given URL - you
# may have another mechanism
OPEN_BROWSER=open

# Unfortunately, not all of the browsers let me use SVG natively in the page
# in all conditions where I can use PNG images.  So, I have this rule set up
# to convert SVG images into PNG images.  Note that I am using a tool known as
# rsvg-convert but you may wish to use resvg (See https://github.com/linebender/resvg)
# That is a Rust implementation that is gaining a lot of support and traction.
# Unfortunately, the PNG files it creates are about twice the size of the ones
# the rsvg-convert tool makes.
.SUFFIXES:
.SUFFIXES: .svg .png
.svg.png:
	# resvg --width 256 --height 256 $< $@
	rsvg-convert -w 256 -h 256 -o $@ $<

default: help

clean:
	(cd c; make clean)
	rm -rf *.png Waver-* swift/.build rust/target perf/reports perf/perf_summary.md
	find . -name .DS_Store -delete

waver: $(WAVER)
	@true

waver-all: Waver-swift Waver-rust Waver-c
	@true

art: $(SRC) $(ART)
	@true

# Simple build target that makes sure all these elements are built
build: art $(WAVER)
	@true

build-all: art waver-all
	@true

###############################################################################

# Copy the Simple Web Audio Player to the LOCALDIR, including the
# artwork.  I use RSYNC such that we can see which files have actually changed
# in the target location.  This way we don't churn the target if it was not
# actually changed.  (Such as just a rebuild of the png files but no change)
localbuild: art
	[ -d $(LOCALDIR) ]
	rsync -cav $(SRC) $(ART) $(LOCALDIR)

# Build the .png wave files for the MP3 files in the local directory
# Once this is done, this runs very quickly and notices that there is nothing
# to do.
localwaves: waver
	[ -d $(LOCALDIR) ]
	./$(WAVER) $(LOCALDIR)

# There is a nice little Rust web server for static web pages that is great
# for testing.  It compiles anywhere Rust does.  static-web-server is
# what it is.  If you don't have it, we will fail this.
# See https://github.com/static-web-server/static-web-server
# Unfortunately, this is likely only working for Linux/MacOS unless you
# have killall available for Windows.
localtest: localbuild localwaves
	which static-web-server
	(killall static-web-server; static-web-server --host 127.0.0.1 --port 8088 --root $(LOCALDIR) & true)
	$(OPEN_BROWSER) http://localhost:8088/Music.html

###############################################################################

# Build the Swift version of the Waver tool
swift/.build/release/Waver: $(wildcard swift/Sources/*.swift) $(wildcard swift/*.swift)
	(cd swift ; swift build -j $$(sysctl -n hw.ncpu) -c release)

Waver-swift: swift/.build/release/Waver
	cp $< $@
	ls -l $@
	strip $@
	ls -l $@

###############################################################################

# Build the Rust version of the Waver tool  This first part is actually
# running the unit tests
rust/target/release/test.log: $(wildcard rust/src/*.rs) $(wildcard rust/src/*/*.rs) rust/Cargo.toml
	mkdir -p rust/target/release
	(cd rust; cargo test --all-features --release -- --nocapture) >$@.err || (cat $@.err; exit 9)
	mv -f $@.err $@
	cat $@

# This is the actual build
rust/target/release/waver: $(wildcard rust/src/*.rs) $(wildcard rust/src/*/*.rs) rust/Cargo.toml rust/target/release/test.log
	(cd rust; cargo build --all-features --release)

Waver-rust: rust/target/release/waver
	cp $< $@
	ls -l $@
	strip $@
	ls -l $@

###############################################################################

# Build the C version of the Waver tool
c/waver: $(wildcard c/src/*) $(wildcard c/include/*) c/Makefile
	(cd c; make)

Waver-c: c/waver
	cp $< $@
	ls -l $@
	strip $@
	ls -l $@

###############################################################################

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
