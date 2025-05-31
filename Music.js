/**
 * Simple Web Audio Player - Copyright Michael Sinz
 *
 * This file contains the core functionality for the music player, including:
 * - Audio playback control
 * - UI manipulation and event handling
 * - Playlist management
 * - Directory navigation
 * - Metadata extraction and display
 */

// This is a feature of newer browsers but is ignored on older browsers
// This enforces stricter rules in JavaScript.
// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Strict_mode
"use strict";

//==============================================================================
// GLOBAL VARIABLES AND DOM ELEMENT REFERENCES: Application state and UI elements
//==============================================================================

/**
 * Audio playback state - tracks currently playing and next queued track
 */
let CurrentTrack = null;  // The currently playing audio track
let NextTrack = null;     // Preloaded next track for seamless playback
const NextTrackWave = new Image();  // Container for the waveform image of the next track

/**
 * Shorthand function for document.getElementById
 * @param {string} id - The element ID to find
 * @returns {HTMLElement} The DOM element with the specified ID
 */
const _ = (id) => document.getElementById(id);

/**
 * Page and UI element references - cached for performance
 */
const OriginalTitle = document.title;  // Store original page title
// Progress bar and player control elements
const ProgressSliderBG = _("sliderDiv");
const ProgressDiv = _('sliderProgressDiv');
const KnobDiv = _('sliderKnob');
const ProgressTimeDiv = _("progressTime");
const PlayerDisplay = _('Player');
const PlayButton = _('PlayButton');
const PauseButton = _('PauseButton');
// Playlist and detail display elements
const Playlist = _("Playlist");
const ShowDetailsDiv = _('ShowDetails');
const HideDetailsDiv = _('HideDetails');
const DetailsDiv = _('Details');
const SaveButton = _('SaveButton');
const AddAllButton = _('AddAll');
const PlaylistName = _('PlaylistName');
const SaveCurrent = _('SaveCurrent');
const ContainerDiv = _('Container');
const PlaylistCell = _('PlaylistCell');
const CoverArt = _('CoverArt');
const BackCoverArt = _('BackCoverArt');
const TrackLine = _('TrackLine');
// Additional cached elements to improve consistency and performance
const PathItems = _('PathItems');
const TrackElement = _('Track');
const AlbumElement = _('Album');
const EjectAllButton = _('EjectAll');
const NextTrackButton = _('NextTrackButton');
const PrevTrackButton = _('PrevTrackButton');
const SaveNowPlayingButton = _('SaveNowPlaying');
const CancelButtonElement = _('CancelButton');

/**
 * Special character used to separate items in playlist storage
 * Using ASCII 127 (DEL) as it's unlikely to appear in normal text
 */
const SplitChar = String.fromCharCode(127);

//==============================================================================
// CORE UTILITY FUNCTIONS: General purpose helpers and storage handling
//==============================================================================

/**
 * Gets the base document path without any hash/fragment
 * @returns {string} The document URL without the fragment
 */
function GetDocPath() {
	const t = document.location.href;
	const p = t.indexOf('#');
	if (p > 0) {
		return t.substring(0, p);
	}
	return t;
}

const DocPath = GetDocPath();
const RootPath = DocPath.replace(/\/[^\/]*$/, '');

/**
 * Provides access to localStorage with fallback if not available
 * Must be called after DocPath is initialized!
 * @returns {Object} Either localStorage or a fallback object
 */
function GetLocalStorage() {
	try {
		localStorage.setItem('rawDocPath', DocPath);
		return localStorage;
	}
	catch (e) {
		// If localStorage fails, provide a memory-only fallback object
		console.warn(`localStorage not available "${e}", using memory storage instead`);
		return {
			_data: {},
			getItem: function (key) {
				return this._data[key] || null;
			},
			setItem: function (key, value) {
				this._data[key] = String(value);
			},
			removeItem: function (key) {
				delete this._data[key];
			}
		};
	}
}

/**
 * Storage object for persisting application state
 * Uses either browser's localStorage or memory fallback if unavailable
 */
const MusicStorage = GetLocalStorage();

/**
 * Initial position for audio playback (in seconds)
 * Used when resuming playback from a stored position
 */
let InitialPos = 0;

/**
 * Determines the starting path for the music browser
 * Prioritizes: 1) URL fragment, 2) Stored path, 3) Default path
 * @returns {string} The path to start browsing from
 */
function GetStartPath() {
	const t = document.location.href;
	const p = t.indexOf('#');
	if (p > 0) {
		return decodeURI(t.substring(p + 1));
	}
	if (MusicStorage.path) {
		return MusicStorage.path;
	}
	return '/Music/';
}

/**
 * Attaches an event handler to an element
 * Helper function to simplify event binding and handle null elements gracefully
 *
 * @param {HTMLElement} element - The DOM element
 * @param {string} eventType - The event type (e.g., 'click')
 * @param {Function} handler - The event handler function
 */
//==============================================================================
// INITIALIZATION AND EVENT SETUP: Application startup and event binding
//==============================================================================

/**
 * Attaches an event handler to an element
 * Helper function to simplify event binding and handle null elements gracefully
 *
 * @param {HTMLElement} element - The DOM element
 * @param {string} eventType - The event type (e.g., 'click')
 * @param {Function} handler - The event handler function
 */
function attachEvent(element, eventType, handler) {
	if (element) {
		element.addEventListener(eventType, handler);
	}
}

/**
 * Sets up all player control events
 * Initializes event handlers for playback controls, UI elements, and progress slider
 */
function setupPlayerEvents() {
	// Player control buttons
	attachEvent(EjectAllButton, 'click', ClearPlaylist);
	attachEvent(NextTrackButton, 'click', FastForward);
	attachEvent(PrevTrackButton, 'click', Rewind);
	attachEvent(SaveNowPlayingButton, 'click', SaveNowPlaying);
	attachEvent(CancelButtonElement, 'click', () => {
		hideElement(SaveCurrent);
	});

	// Play/Pause buttons
	attachEvent(PlayButton, 'click', () => {
		if (CurrentTrack) CurrentTrack.play();
	});
	attachEvent(PauseButton, 'click', () => {
		if (CurrentTrack) CurrentTrack.pause();
	});

	// Progress slider and knob events - supports both mouse and touch interfaces
	attachEvent(ProgressSliderBG, 'mousedown', ProgressSliderClick);
	attachEvent(KnobDiv, 'mousedown', KnobGrab);
	attachEvent(KnobDiv, 'touchstart', KnobGrab);
	attachEvent(KnobDiv, 'touchmove', KnobMove);
	attachEvent(KnobDiv, 'touchend', KnobRelease);
	attachEvent(KnobDiv, 'touchcancel', KnobRelease);

	// UI controls for details view and playlist management
	attachEvent(AddAllButton, 'click', AddAllToPlaylist);
	attachEvent(ShowDetailsDiv, 'click', ShowDetails);
	attachEvent(HideDetailsDiv, 'click', HideDetails);
	attachEvent(PlaylistName, 'change', () => { SaveButton.focus(); });
}

/**
 * Main initialization function - sets up event handlers and loads initial content
 * Called from the onload event in the HTML
 *
 * This function:
 * 1. Sets up all UI event handlers
 * 2. Restores previous playback position if available
 * 3. Loads any previously saved playlist
 * 4. Navigates to the initial directory
 */
function Start() {
	// Setup all event handlers for player controls
	setupPlayerEvents();

	// Restore previous playback position if available
	if (MusicStorage.lastPos) {
		InitialPos = MusicStorage.lastPos;
	}

	// Restore previously saved playlist if available
	let priorList = '';
	if (MusicStorage.playing) {
		priorList = MusicStorage.playing;
	}

	// Reset current playlist and add back the saved tracks
	MusicStorage.playing = '';
	AddListToPlaylist(priorList);

	// Start loading the initial directory based on URL fragment or saved path
	TriggerLoadDir(GetStartPath());
}

//==============================================================================
// PLAYER CONTROLS AND TIME DISPLAY: Audio playback and progress tracking
//==============================================================================

/**
 * Updates the time display based on the current position percentage
 * Converts percentage to time and formats it as MM:SS
 *
 * @param {number} value - The percentage of playback progress (0-100)
 */
function UpdateTimeDiv(value) {
	const seconds = Math.round((100 - value) * ProgressTimeDiv.totalSeconds) / 100;

	// Skip update if the time hasn't changed
	if (ProgressTimeDiv.LastTime && ProgressTimeDiv.LastTime == seconds) {
		return;
	}

	ProgressTimeDiv.LastTime = seconds;

	// Format time as MM:SS
	const minutes = Math.floor(seconds / 60);
	const remainingSeconds = Math.floor(seconds % 60);
	ProgressTimeDiv.textContent = `${minutes}:${('00' + remainingSeconds).slice(-2)}`;
}

/**
 * Sets the position of the progress slider knob and updates the time display
 *
 * @param {number} percent - The percentage position (0-100)
 */
function SetKnobPos(percent) {
	ProgressDiv.style.width = percent + '%';
	UpdateTimeDiv(percent);
}

/**
 * Sets both the visual position of the knob and the actual audio position
 *
 * @param {number} offset - The pixel offset from the left edge of the slider
 */
function SetKnobPosAndAudio(offset) {
	if (ProgressSliderBG.clientWidth) {
		const percent = offset * 100.0 / ProgressSliderBG.clientWidth;
		SetKnobPos(percent);
		if (CurrentTrack) {
			if (CurrentTrack.duration && !isNaN(CurrentTrack.duration)) {
				CurrentTrack.currentTime = percent * CurrentTrack.duration / 100.0;
			}
			else {
				EndPlay();
			}
		}
	}
}

/**
 * Handles the user clicking directly on the progress slider
 * Jumps to the clicked position in the audio track
 *
 * @param {MouseEvent} e - The mouse click event
 */
function ProgressSliderClick(e) {
	if (e.button) return;  // Only handle primary button clicks
	e.stopPropagation();
	e.preventDefault();
	SetKnobPosAndAudio(e.offsetX);
}

/**
 * Handles the release of the slider knob
 * Removes document-level event listeners and resumes playback
 *
 * @param {MouseEvent|TouchEvent} e - The mouse/touch event
 */
function KnobRelease(e) {
	if (e.button) return;  // Only handle primary button events
	e.stopPropagation();
	e.preventDefault();

	// Clean up event listeners
	document.removeEventListener('mousemove', KnobMove);
	document.removeEventListener('mouseup', KnobRelease);

	// Resume playback if track is available
	if (CurrentTrack) CurrentTrack.play();
}

/**
 * Stores the starting X position for drag operations
 * Used to calculate relative movement during knob dragging
 */
let startX = 0;

/**
 * Handles the movement of the slider knob during drag operations
 * Updates both the visual position and the playback position
 *
 * @param {MouseEvent|TouchEvent} e - The mouse/touch movement event
 */
function KnobMove(e) {
	if (e.button) return;  // Only handle primary button events
	e.stopPropagation();
	e.preventDefault();

	// Handle touch events by getting the first touch point
	if (e.changedTouches) {
		e = e.changedTouches[0];
	}

	// Only process if there was actual movement
	if (e.screenX != startX) {
		// Calculate new position based on drag distance from starting point
		const maybePos = ProgressDiv.clientWidth + e.screenX - startX;

		// Constrain to slider width boundaries (0 to max width)
		const newPos = Math.min(Math.max(maybePos, 0), ProgressSliderBG.clientWidth - 1);

		// Update starting position for next movement calculation
		startX = e.screenX + newPos - maybePos;

		// Update knob position and audio time
		SetKnobPosAndAudio(newPos);
	}
}

/**
 * Initializes knob dragging when user grabs the slider knob
 * Sets up event listeners for dragging and updates the start position
 *
 * @param {MouseEvent|TouchEvent} e - The mouse/touch event
 */
function KnobGrab(e) {
	if (e.button) return;  // Only handle primary button events
	e.stopPropagation();
	e.preventDefault();

	// Pause the audio
	if (CurrentTrack) CurrentTrack.pause();

	// If there is a touch, switch to it.
	if (e.changedTouches) {
		e = e.changedTouches[0];
	}

	// Save the starting screen location such that moves can be
	// relative to this point
	startX = e.screenX;

	// Set up the document-level event handlers for dragging
	document.addEventListener('mousemove', KnobMove);
	document.addEventListener('mouseup', KnobRelease);
}

/**
 * Updates the UI based on the current track playback state
 * Updates progress slider, play/pause buttons, and track styling based on playback state
 */
function TimeUpdate() {
	if (CurrentTrack) {
		const currentTime = CurrentTrack.currentTime;
		const totalDuration = CurrentTrack.duration;
		let playStyle = 'Audio Playing';

		if (CurrentTrack.paused) {
			// Update UI for paused state
			PauseButton.style.display = 'none';
			PlayButton.style.display = '';
			playStyle = 'Audio Paused';
		}
		else {
			// Update UI for playing state
			PauseButton.style.display = '';
			PlayButton.style.display = 'none';
		}

		// Update the track element's class based on play state
		if (CurrentTrack.trackId) {
			const trackElement = _(CurrentTrack.trackId);
			if (trackElement && trackElement.className != playStyle) trackElement.className = playStyle;
		}

		// Make player visible
		PlayerDisplay.style.display = '';

		// Handle invalid duration
		if (isNaN(totalDuration)) {
			return;
		}

		// Handle zero duration (initial state)
		if (totalDuration == 0) {
			return SetKnobPos(0);
		}

		// Store current position and update progress display
		MusicStorage.lastPos = currentTime;
		ProgressTimeDiv.totalSeconds = totalDuration;
		SetKnobPos(currentTime * 100.0 / totalDuration);
	}
	else {
		// Reset everything when no track is playing
		MusicStorage.lastPos = 0;
		ProgressTimeDiv.totalSeconds = 1;
		SetKnobPos(0);
		PlayerDisplay.style.display = 'none';
	}
}

/**
 * Adds a list of tracks to the playlist
 * Parses a delimited string of track paths and adds each valid track
 *
 * @param {string} list - SplitChar-separated list of track paths
 */
function AddListToPlaylist(list) {
	const items = list.split(SplitChar);
	for (const i in items) {
		const path = items[i];
		// Only process paths with reasonable length (> 5 chars)
		if (path.length > 5) {
			AddToPlaylist(MakeAudio(path));
		}
	}
}

/**
 * Shows the details panel by updating element visibility
 * Controls the display of album art and track information
 */
function ShowDetails() {
	hideElement(ShowDetailsDiv);
	showElementAs(HideDetailsDiv, 'inline');
	showElement(DetailsDiv);
}

/**
 * Hides the details panel by updating element visibility
 * Hides album art and track information
 */
function HideDetails() {
	showElementAs(ShowDetailsDiv, 'inline');
	hideElement(HideDetailsDiv);
	hideElement(DetailsDiv);
}

/**
 * Saves the currently playing playlist
 * Opens a dialog for the user to name and save the playlist
 */
function SaveNowPlaying() {
	const playlist = MusicStorage.playing;
	PlaylistName.value = '';

	/**
	 * Handler for the Save button click
	 * Stores the playlist in localStorage with the user-provided name
	 */
	SaveButton.onclick = () => {
		const playlistKey = SplitChar + PlaylistName.value;
		// Only save if the playlist name is not empty
		if (playlistKey.length > 1) {
			if (playlist.length > 1) {
				// Store the playlist
				MusicStorage[playlistKey] = playlist;
			}
			else {
				// Remove empty playlists
				delete MusicStorage[playlistKey];
			}
		}
		// Hide the save dialog and refresh current directory
		hideElement(SaveCurrent);
		TriggerLoadDir(MusicStorage.path);
	};

	// Show the save dialog
	showElementAs(SaveCurrent, 'table-cell');

	// Focus the playlist name input after a short delay
	// (ensures UI is ready before focusing)
	setTimeout(() => { PlaylistName.focus(); }, 1);
}

/**
 * Counter for pending add operations
 * Used to stagger additions to the playlist for better visual effect
 */
let PendingAdd = 0;

/**
 * Adds an item to the playlist with a visual delay for better UX
 * Creates a staggered effect when adding multiple items at once
 *
 * @param {HTMLElement} div - The element to add to the playlist
 */
function AddToPlaylist(div) {
	// This creates a staggered delay when adding multiple items
	// so they visually "flow" into the display one after another
	if (div.onclick) {
		PendingAdd++;
		setTimeout(() => {
			PendingAdd--;
			div.onclick();
		}, 5 * PendingAdd);  // 5ms multiplier for each pending item
	}
}

//==============================================================================
// HELPER FUNCTIONS - DOM MANIPULATION AND FORMATTING: UI creation and styling
//==============================================================================

/**
 * Creates an HTML element with configurable properties
 *
 * @param {string} tagName - The HTML tag to create
 * @param {Object} [options] - Optional configuration object
 * @param {string} [options.className] - CSS class to apply
 * @param {string} [options.text] - Text content for the element
 * @param {Function} [options.onClick] - Click event handler
 * @param {string} [options.id] - Optional ID for the element
 * @returns {HTMLElement} The created element
 */
function createElement(tagName, options = {}) {
	const element = document.createElement(tagName);

	if (options.className) element.className = options.className;
	if (options.text) element.textContent = options.text;
	if (options.onClick) element.onclick = options.onClick;
	if (options.id) element.id = options.id;

	return element;
}

/**
 * Adds a text div to a parent element
 * @param {HTMLElement} div - The parent element
 * @param {string} text - The text content to add
 * @returns {HTMLElement} The parent element (for chaining)
 */
function AddText(div, text) {
	const t = createElement('div', { text });
	div.appendChild(t);
	return div;
}

/**
 * Safely encodes a path for use in URLs
 * Handles special characters and ensures proper encoding of URL parts
 *
 * @param {string} path - The path to encode
 * @returns {string} The encoded path
 */
function safeEncodePath(path) {
	// Handle null or undefined paths
	if (path == null) return '';

	return path.split('/').map(part => encodeURIComponent(part)).join('/').replace(/'/g, "%27");
}

/**
 * Extracts the file name from a path and formats it
 * Removes the directory path and the file extension and replaces underscores with spaces
 *
 * @param {string} path - The full path including filename
 * @returns {string} The formatted filename
 */
function formatFileName(path) {
	if (!path) return '';

	return path.replace(/^.*\/([^\/]+)\.mp3$/, '$1').replace(/_/g, ' ');
}

//==============================================================================
// UI HELPER FUNCTIONS: Element visibility and content manipulation
//==============================================================================

/**
 * Shows an element by setting its display style to an appropriate value
 * @param {HTMLElement} element - The element to show
 * @param {string} [displayType=''] - Optional display type (block, inline, etc.)
 */
function showElement(element) {
	if (!element) return;

	// For most elements, '' reverts to the default display type
	element.style.display = '';
}

/**
 * Shows an element with a specific display style
 * @param {HTMLElement} element - The element to show
 * @param {string} displayType - The display style to use (block, inline-block, etc.)
 */
function showElementAs(element, displayType) {
	if (!element) return;

	element.style.display = displayType || '';
}

/**
 * Hides an element by setting its display to 'none'
 * @param {HTMLElement} element - The element to hide
 */
function hideElement(element) {
	if (!element) return;

	element.style.display = 'none';
}

/**
 * Toggles an element's visibility based on a condition
 * @param {HTMLElement} element - The element to toggle
 * @param {boolean} shouldShow - Whether to show or hide the element
 * @param {string} [displayType=''] - Optional display type when showing
 */
function toggleElement(element, shouldShow, displayType) {
	if (!element) return;

	element.style.display = shouldShow ? (displayType || '') : 'none';
}

/**
 * Safely updates element's text content
 * @param {HTMLElement} element - The element to update
 * @param {string} text - The text to set
 */
function updateElementText(element, text) {
	if (!element) return;

	element.textContent = text || '';
}

/**
 * Improved version of the SetText function to properly
 * handle visibility based on content
 *
 * @param {string} id - The ID of the element
 * @param {string} text - The text to set
 */
function setElementText(id, text) {
	const element = _(id);
	if (!element) return;

	updateElementText(element, text);
	toggleElement(element, text !== '');
}

//==============================================================================
// PLAYLIST MANAGEMENT: Track list handling and storage
//==============================================================================

/**
 * Joins path segments with proper handling of slashes
 * Removes empty segments and normalizes multiple slashes
 *
 * @param {...string} segments - The path segments to join
 * @returns {string} The joined path
 * @example
 * // returns "path/to/file"
 * joinPath("path", "to", "file")
 * // returns "path/to/file"
 * joinPath("path/", "/to/", "/file")
 */
function joinPath(...segments) {
	// Filter out null/undefined segments
	segments = segments.filter(s => s != null);

	// Join segments and normalize slashes
	return segments.filter(s => s !== '').join('/').replace(/\/+/g, '/');
}

/**
 * Event handler for clicking an audio file in the file list
 * Passes the current element to the PlayFile function
 */
function PlayFileDiv() {
	PlayFile(this);
}

/**
 * Converts a file path to a track ID for DOM element identification
 * Removes the root path prefix, underscores, and file extension
 *
 * @param {string} path - The full file path
 * @returns {string} The formatted track ID
 */
function TrackIdFromPath(path) {
	if (!path) return '';

	if (path.startsWith(RootPath)) {
		path = path.substring(RootPath.length + 1);
	}
	return path.replace(/_/g, ' ').replace('.mp3', '');
}

/**
 * Creates an audio file element for display in the file list
 *
 * @param {string} path - The path to the audio file
 * @returns {HTMLElement} The created audio element div
 */
function MakeAudio(path) {
	try {
		// Format the filename for display
		const name = formatFileName(path);

		// Create the main container element
		const div = createElement('div', {
			className: 'Audio',
			onClick: PlayFileDiv,
			id: TrackIdFromPath(path)
		});
		div.path = path;

		// Apply special styling if this is the current track
		if (CurrentTrack && CurrentTrack.trackId && CurrentTrack.trackId == div.id) {
			div.className = 'Audio Paused';
		}

		// Create the icon for the audio file
		const musicIcon = createElement('img');
		musicIcon.src = 'Blank.png';
		div.appendChild(musicIcon);

		// Add the track name and return the complete element
		return AddText(div, name);
	} catch (error) {
		// Handle errors gracefully with a fallback display
		console.error(`Error creating audio element: ${error.message} for path: ${path}`);
		// Create a fallback element that shows there was an error
		return createElement('div', {
			className: 'Audio',
			text: `Error loading: ${formatFileName(path)}`
		});
	}
}

//==============================================================================
// DIRECTORY NAVIGATION: Folder browsing and file listing
//==============================================================================

/**
 * Creates a folder element for display in the UI
 * Includes proper icon and optional cover art loading
 *
 * @param {string} link - The folder path
 * @param {string} text - The display text for the folder
 * @param {Object} folderInfo - Information about the folder
 * @returns {HTMLElement} The created folder element
 */
function MakeFolder(link, text, folderInfo) {
	// Create the main folder container
	const div = createElement('div', {
		className: 'Folder',
		onClick: LoadDirFunc(link)
	});

	// Create the folder icon
	const cover = createElement('img');
	// Use different icons for folders vs. music albums
	cover.src = folderInfo['Folders'] ? 'Folder.png' : 'Record.png';

	// Handle cover art if available
	if (folderInfo['Cover']) {
		// Create an image element for the real cover
		const realCover = createElement('img');
		realCover.src = `${RootPath}${EncodePath(joinPath(link, 'Cover.jpg'))}`;
		realCover.style.display = 'none';
		// Replace the generic icon with the real cover when it loads
		realCover.onload = function () {
			cover.src = this.src;
			// Once loaded, we really don't need ourselves anymore
			if (this.parentElement) {
				this.parentElement.removeChild(this);
			}
		}
		div.appendChild(realCover);
	}

	div.appendChild(cover);
	return AddText(div, text);
}

/**
 * Creates a playlist element for display in the UI
 * Includes icon, name, and delete button
 *
 * @param {string} name - The playlist name
 * @returns {HTMLElement} The created playlist element
 */
function MakePlaylist(name) {
	// Create the main playlist container with click handler to load the playlist
	const div = createElement('div', {
		className: 'Playlist',
		onClick: function () { AddListToPlaylist(MusicStorage[name]); }
	});

	// Add playlist icon
	const cover = createElement('img');
	cover.src = 'Playlist.png';
	div.appendChild(cover);

	// Add playlist name text
	const divWithText = AddText(div, name.substring(1));

	// Add delete button
	const deleteBtn = createElement('img');
	deleteBtn.src = 'Delete.png';
	deleteBtn.className = 'DeletePlaylist';
	// Set up event handler to delete the playlist and refresh the view
	deleteBtn.onclick = function (e) {
		e.stopPropagation(); // Prevent triggering the parent's click handler
		delete MusicStorage[name];
		TriggerLoadDir(MusicStorage.path);
	};
	divWithText.appendChild(deleteBtn);

	return divWithText;
}

/**
 * Creates the directory contents display with files, folders and playlists
 * Builds the UI components for a directory view based on the provided data
 *
 * @param {string} path - The current directory path
 * @param {Object} treePart - The directory structure data
 * @returns {HTMLElement} The container with directory contents
 */
function LoadedDir(path, treePart) {
	// Create the main container for directory contents
	const div = createElement('div', { id: 'Contains' });
	div.AddAll = 'none';  // Default to hiding "Add All" button

	let hr = false;  // Track if we need to add a separator

	// Handle Files section - display audio files
	if (treePart['Files']) {
		// Create container for audio files
		const fileContainer = createElement('div', { id: 'Files' });
		div.appendChild(fileContainer);

		// Create separator for the next section
		hr = createElement('hr');

		// Show "Add All" button since we have files
		div.AddAll = '';

		// Add each audio file to the container
		for (const i in treePart['Files']) {
			const filePath = `${RootPath}${path}${treePart['Files'][i]}.mp3`;
			fileContainer.appendChild(MakeAudio(filePath));
		}
	}

	// Handle Folders section - display subdirectories
	if (treePart['Folders']) {
		// Add separator if we already displayed files
		if (hr) {
			div.appendChild(hr);
		}
		hr = createElement('hr');

		// Create container for folders
		const dirContainer = createElement('div', { id: 'Folders' });
		div.appendChild(dirContainer);

		// Add each folder to the container
		const folders = treePart['Folders'];
		for (const folderName in folders) {
			const folderInfo = folders[folderName];
			const link = `${folderName}/`;
			const text = folderName.replace(/_/g, ' ');  // Format folder name
			dirContainer.appendChild(MakeFolder(path + link, text, folderInfo));
		}
	}

	// Handle Playlists section - display saved playlists
	const playlists = [];
	// Find all saved playlists in storage
	for (const key in MusicStorage) {
		if (key.indexOf(SplitChar) === 0) {
			playlists.push(key);
		}
	}

	if (playlists.length > 0) {
		// Sort playlists alphabetically
		playlists.sort();
		// Add separator if we've already displayed files or folders
		if (hr) {
			div.appendChild(hr);
		}
		// Create container for playlists
		const playlistContainer = createElement('div', { id: 'Playlists' });
		div.appendChild(playlistContainer);

		// Add each playlist to the container
		for (const i in playlists) {
			playlistContainer.appendChild(MakePlaylist(playlists[i]));
		}
	}

	// Set visibility of "Add All" button based on whether we have files
	AddAllButton.style.display = div.AddAll;
	// Add the entire contents to the main container
	ContainerDiv.appendChild(div);

	// Save the current path for next time
	MusicStorage.path = path;
}

/**
 * Creates a function to load a specific directory
 * Used for event handlers that need to reference a specific path
 *
 * @param {string} path - The path to load
 * @returns {Function} A function that will load the specified directory
 */
function LoadDirFunc(path) {
	return () => { LoadDir(path); };
}

/**
 * Set up a "continuation" that will run LoadDir of the given path
 *
 * @param {string} path
 */
function TriggerLoadDir(path) {
	setTimeout(LoadDirFunc(path), 1)
}

/**
 * Loads and displays the specified directory
 * Clears current display, sets up breadcrumb navigation, and loads directory contents
 *
 * @param {string} path - The directory path to load
 */
function LoadDir(path) {
	// Update the URL fragment to reflect the current path
	_('PathLink').href = '#' + encodeURI(path);
	// Clear the current directory contents
	ContainerDiv.innerHTML = '';
	// Hide "Add All" button until we know if there are files
	AddAllButton.style.display = 'none';

	// Clear the path navigation breadcrumbs
	PathItems.innerHTML = '';

	// We delay loading slightly to allow the prior DOM elements
	// (where the user clicked) to be fully removed before creating new ones.
	// This is important on mobile devices to prevent touch events from
	// being incorrectly registered on the new page elements.
	setTimeout(() => { LoadDirPart2(LoadedDir, path, PathItems); }, 75);
}

/**
 * Second part of directory loading that builds the breadcrumb path
 * and navigates through the data structure to find the requested directory
 *
 * @param {Function} callNext - Function to call with the directory data
 * @param {string} path - The directory path to load
 * @param {HTMLElement} pathItems - DOM element for breadcrumb navigation
 */
function LoadDirPart2(callNext, path, pathItems) {
	let t = path;        // Remaining path to process
	let p = '';          // Accumulated path for breadcrumbs
	let treePart;        // Current position in the directory structure

	// Process the path segments one by one
	while (t.indexOf('/') >= 0) {
		const i = t.indexOf('/');
		let n = t.substring(0, i);  // Get the current path segment
		p += n + '/';               // Accumulate the path
		t = t.substring(i + 1);     // Remove processed segment

		// Handle root directory specially
		if (n == '') {
			n = 'ROOT';   // Display name for root
			treePart = mp3; // Start at the root of the data structure
		}
		else {
			// Navigate to the subdirectory in the data structure
			treePart = treePart['Folders'][n];

			// Add a separator slash to the breadcrumbs
			const spanSlash = document.createElement('span');
			spanSlash.className = 'Slash';
			spanSlash.textContent = ' / ';
			if (pathItems) pathItems.appendChild(spanSlash);
		}

		// Create a clickable breadcrumb for this path segment
		const spanPath = document.createElement('span');
		spanPath.className = 'Path';
		spanPath.onclick = LoadDirFunc(p);
		spanPath.textContent = n.replace(/_/g, ' ');  // Format the display name
		if (pathItems) pathItems.appendChild(spanPath);
	}

	// Call the provided function with the directory data
	callNext(path, treePart);
}

/**
 * Extracts the directory path from a full track path
 *
 * @param {string} trackPath - The full track path
 * @returns {string} The directory path
 */
function TrackPathToPath(trackPath) {
	if (!trackPath) return '';

	return trackPath.replace(RootPath, '').replace(/[^\/]+$/, '');
}

/**
 * Loads the directory containing a specific track
 *
 * @param {string} trackPath - The path of the track
 */
function LoadSongDir(trackPath) {
	TriggerLoadDir(TrackPathToPath(trackPath));
}

/**
 * Sets the text content of an element and controls its visibility
 * Hides the element if text is empty
 *
 * @param {string} id - The ID of the element
 * @param {string} text - The text to set
 */
function SetText(id, text) {
	setElementText(id, text);
}

/**
 * Legacy function for encoding paths - maintained for backward compatibility
 * @param {string} path - The path to encode
 * @returns {string} The encoded path
 */
function EncodePath(path) {
	return safeEncodePath(path);
}

/**
 * Converts a track object to its full file path
 * Handles encoding for special characters in the path
 *
 * @param {Object} track - The track object with path property
 * @returns {string} The full encoded file path
 */
function TrackToPath(track) {
	let trackPath = '';
	if (track && track.path) {
		trackPath = track.path;
		// If the path is already a full path, encode only the relative part
		if (trackPath.startsWith(RootPath)) {
			trackPath = `${RootPath}${EncodePath(trackPath.substring(RootPath.length))}`;
		}
	}
	return trackPath;
}

/**
 * Preloads the next track in the playlist for seamless playback
 * Creates an Audio object and starts loading the file
 */
function PreLoadNext() {
	// Get the path of the first track in the playlist
	const trackPath = TrackToPath(Playlist.firstChild);

	// Skip if the track is already preloaded or there's no track
	if (NextTrack && NextTrack.src == trackPath) return;
	if (trackPath == '') {
		NextTrack = null;
	}
	else {
		try {
			// Create new Audio object for the next track
			NextTrack = new Audio(trackPath);

			// Start loading the audio
			// Note: This is necessary on some browsers/devices (especially iOS)
			// that won't start loading until explicitly told to for power saving
			NextTrack.load();

			// Load the ID3 elements for the track (but don't care about it)
			LoadID3Tags(NextTrack);

			// Handle loading errors
			NextTrack.onerror = () => {
				console.error(`Failed to load next track: ${trackPath}`);
				NextTrack = null; // Clear the failed track
			};

			// Also preload the waveform image if available
			NextTrackWave.src = trackPath + '.png';
		} catch (error) {
			console.error(`Error preloading next track: ${error.message}`);
			NextTrack = null;
		}
	}
}

//==============================================================================
// METADATA AND DISPLAY: Track information and artwork handling
//==============================================================================

/**
 * Displays information about the current track
 * Updates track title, album info, and cover art
 * With enhanced error handling for better resilience
 *
 * @param {Object} track - The track object
 * @param {string} trackPath - The full track path
 */
function DisplayTrackInfo(track, trackPath) {
	// Validate inputs
	if (!track) {
		console.error("DisplayTrackInfo called with null track");
		return;
	}
	if (!trackPath) {
		console.error("DisplayTrackInfo called with null trackPath");
		return;
	}

	try {
		// Display the track name and make it clickable to navigate to its directory
		SetText('Track', track.textContent || formatFileName(track.path));
		TrackLine.style.display = '';

		const goto_album = function () {
			try {
				LoadSongDir(track.path);
			} catch (error) {
				console.error(`Error navigating to track directory: ${error.message}`);
			}
		};

		if (TrackElement) {
			TrackElement.onclick = goto_album;
		}

		if (AlbumElement) {
			AlbumElement.onclick = goto_album;
		}

		// Set up cover art with default image and click handler
		if (CoverArt) {
			CoverArt.onclick = goto_album;
			CoverArt.style.display = 'inline-block';
		}

		if (BackCoverArt) {
			BackCoverArt.style.display = 'none';
		}

		if (CoverArt) {
			CoverArt.src = 'Record.png';  // Default image
		}

		if (BackCoverArt) {
			BackCoverArt.src = 'Record.png';  // Default image
		}

		// Try to find cover art in the folder structure
		try {
			LoadDirPart2(function (path, treePart) {
				// If track changed while loading, abort
				if (!CurrentTrack || CurrentTrack.src != trackPath) return;

				// Check if treePart is valid
				if (!treePart) {
					console.error("Invalid treePart in LoadDirPart2 callback");
					return;
				}

				// If folder has cover art info
				if (treePart['Cover']) {
					// Load front cover
					if (CoverArt) {
						try {
							CoverArt.src = EncodePath(joinPath(path.substring(1), 'Cover.jpg'));
						} catch (error) {
							console.error(`Error setting cover art: ${error.message}`);
							CoverArt.src = 'Record.png';  // Default fallback
						}
					}

					// If folder has back cover art (Cover=2)
					if (treePart['Cover'] == 2 && BackCoverArt) {
						try {
							BackCoverArt.src = EncodePath(joinPath(path.substring(1), 'Back.jpg'));
							BackCoverArt.style.display = 'inline-block';
							BackCoverArt.onclick = CoverArt ? CoverArt.onclick : null;
						} catch (error) {
							console.error(`Error setting back cover art: ${error.message}`);
							BackCoverArt.style.display = 'none';
						}
					}
				}
			}, TrackPathToPath(track.path));
		} catch (error) {
			console.error(`Error in folder cover art lookup: ${error.message}`);
		}

		// Load ID3 tags from the audio file
		LoadID3Tags(CurrentTrack).then(tags => {
			if (!CurrentTrack || CurrentTrack.src != trackPath) return;

			if (tags.album) {
				SetText('Album', tags.album);
			}
			if (tags.title) {
				let title = tags.title;
				// Set page title to include track and artist info
				document.title = `🎼 ${title}${tags.artist ? ` ♮ ${tags.artist}` : ''}`;
				// If track number is available, include it in the display
				if (tags.trackNumber) {
					title = `#${tags.trackNumber}: ${title}`;
				}
				SetText('Track', title);
			}
			if (tags.coverData) {
				// Only if we don't already have cover art
				if (CoverArt && CoverArt.src.endsWith('Record.png')) {
					CoverArt.src = URL.createObjectURL(new Blob([tags.coverData]));;
				}
			}
		}).catch(err => console.error(err));
	} catch (error) {
		console.error(`Unhandled error in DisplayTrackInfo: ${error.message}`);
		// At least attempt to set the track name if everything else fails
		try {
			SetText('Track', formatFileName(track.path));
		} catch (e) {
			// If all else fails, just give up gracefully
			console.error(`Critical error displaying track info: ${e}`);
		}
	}
}

//==============================================================================
// AUDIO PLAYBACK AND CONTROL: Track loading and playback management
//==============================================================================

/**
 * Starts playing the next track in the playlist
 * Handles loading the audio, setting up event handlers, and updating the UI
 * With enhanced error handling for better resilience
 */
function PlayNext() {
	// We are already playing - do nothing
	if (CurrentTrack) return;

	// Get the first track in the playlist
	let track = Playlist ? Playlist.firstChild : null;
	if (!track) {
		// Handle empty playlist case
		resetPlayerUI();
		return;
	}

	// Skip over any non-tracks in the Playlist
	// (could be text nodes or other elements)
	while (track && !track.path) {
		try {
			track.parentElement.removeChild(track);
		} catch (error) {
			console.error(`Error removing non-track element: ${error.message}`);
			// Just move to the next sibling if removal fails
		}
		track = Playlist.firstChild;
	}

	// If we have a track to play
	if (track) {
		const trackPath = TrackToPath(track);

		// Check if we got a valid path
		if (!trackPath) {
			console.error("Invalid track path");
			try {
				Playlist.removeChild(track);
			} catch (error) {
				// Ignore errors when removing invalid tracks
			}
			PlayNext(); // Try the next track
			return;
		}

		try {
			// Use the preloaded track if available, otherwise create a new Audio object
			if (NextTrack && NextTrack.src === trackPath) {
				CurrentTrack = NextTrack;
				NextTrack = null;
			}
			else {
				CurrentTrack = new Audio(trackPath);
			}

			// Store a reference to avoid issues with asynchronous callbacks
			const myTrack = CurrentTrack;

			// Set track ID and update document title
			if (track && track.path) {
				myTrack.trackId = TrackIdFromPath(track.path);
				// Format the document title with music note emoji and track name
				if (track.innerText) {
					document.title = track.innerText.replace(/^\s*[0-9]*[ -]*/, '\uD83C\uDFBC ').replace(/\s*$/, '');
				} else {
					document.title = formatFileName(track.path);
				}
			}

			// Start loading the audio if not already loading
			// Note: Some browsers (especially iOS) won't start loading
			// until explicitly told to for power saving
			if (!myTrack.readyState) {
				myTrack.load();
			}

			// Set up event handlers for audio playback
			myTrack.onended = EndPlay;              // Track finished playing
			myTrack.ontimeupdate = TimeUpdate;      // Progress updates
			myTrack.onpause = TimeUpdate;           // Paused state updates
			myTrack.onplay = TimeUpdate;            // Playing state updates
			myTrack.onerror = (e) => {
				// Get detailed error information if available
				let errorMessage = 'Unknown error';

				if (e && e.target && e.target.error) {
					switch (e.target.error.code) {
						case MediaError.MEDIA_ERR_ABORTED:
							errorMessage = 'Playback aborted by user';
							break;
						case MediaError.MEDIA_ERR_NETWORK:
							errorMessage = 'Network error while loading media';
							break;
						case MediaError.MEDIA_ERR_DECODE:
							errorMessage = 'Media decoding error';
							break;
						case MediaError.MEDIA_ERR_SRC_NOT_SUPPORTED:
							errorMessage = 'Media format not supported';
							break;
						default:
							errorMessage = e.target.error.message || 'Unknown media error';
					}
				}

				console.error(`Error playing track: ${errorMessage}`);

				// Try to remove the track safely
				try {
					if (track && track.parentElement) {
						Playlist.removeChild(track);
					}
				} catch (removeErr) {
					console.error(`Error removing failed track: ${removeErr.message}`);
				}

				EndPlay(); // Clean up and try the next track
			};

			// Store the original path and set initial position
			myTrack.path = track.path;
			myTrack.currentTime = InitialPos;  // Set to stored position if resuming
			InitialPos = 0;  // Reset for next track

			// Try to play the track
			let playPromise;
			try {
				playPromise = myTrack.play();
			} catch (playError) {
				console.error(`Error starting playback: ${playError.message}`);
				// Fall through to the promise handling below
			}

			if (playPromise !== undefined) {
				// Handle autoplay restrictions in modern browsers
				playPromise.catch(error => {
					console.error(`Play failed: ${error.message}`);
					// Some browsers won't auto-play without user interaction
					// We'll keep the track ready but require user to click play
					TimeUpdate(); // Update UI to show pause state
				});
			}

			// Set up a separate execution for the rest of the operations
			// This allows the play command above to start processing
			// while we continue with UI updates and preloading
			setTimeout(function () {
				// If track changed during the timeout, abort
				if (!CurrentTrack || myTrack !== CurrentTrack) return;

				// Try playing again in case it was paused
				if (!myTrack.paused) {
					try {
						myTrack.play();
					} catch (error) {
						// Just log the error, no need to take action here
						// as this is just a retry attempt
						console.error(`Error in retry play: ${error.message}`);
					}
				}

				// Remove the track from the playlist now that it's playing
				try {
					if (track && track.parentElement) {
						Playlist.removeChild(track);
					}
				} catch (error) {
					console.error(`Error removing track from playlist: ${error.message}`);
					// Continue anyway since this is not critical
				}

				// Hide playlist panel if empty
				try {
					if (!Playlist.firstChild) {
						PlaylistCell.style.display = 'none';
					}
				} catch (error) {
					console.error(`Error checking playlist: ${error.message}`);
				}

				// Set waveform background if available
				try {
					ProgressSliderBG.style.backgroundImage = `url('${myTrack.src}.png')`;
				} catch (error) {
					console.error(`Error setting waveform: ${error.message}`);
					// Fallback to no background
					ProgressSliderBG.style.backgroundImage = 'none';
				}

				// Preload the next track and display current track info
				setTimeout(PreLoadNext, 1);
				setTimeout(() => {DisplayTrackInfo(track, myTrack.src);}, 1)
				setTimeout(TimeUpdate, 1);
			}, 1);
		} catch (error) {
			// Handle any errors during playback setup
			console.error(`Error in PlayNext: ${error.message}`);

			// Clear the current track reference
			CurrentTrack = null;

			// Try to remove the problematic track
			try {
				if (track && track.parentElement) {
					Playlist.removeChild(track);
				}
			} catch (removeErr) {
				console.error(`Error removing failed track: ${removeErr.message}`);
			}

			EndPlay(); // Clean up and try next track
		}
	} else {
		// No track to play, reset UI
		resetPlayerUI();
	}
}

/**
 * Resets the player UI when no track is playing
 * Extracted as a separate function for reuse
 */
function resetPlayerUI() {
	ProgressSliderBG.style.backgroundImage = 'none';
	hideElement(PlayerDisplay);
	hideElement(CoverArt);
	hideElement(BackCoverArt);
	hideElement(TrackLine);
	setElementText('Album', '');
	setElementText('Track', '');
	MusicStorage.lastPos = 0;
}

/**
 * Removes a track from the playlist and updates preloading and storage
 *
 * @param {HTMLElement} track - The track element to remove
 */
function RemoveTrack(track) {
	// Get reference to playlist and remove the track
	const playlist = track.parentElement;
	playlist.removeChild(track);

	// Preload the next track now that the order has changed
	PreLoadNext();

	// If playlist is now empty, hide the playlist panel
	track = playlist.firstChild;
	if (!track) {
		PlaylistCell.style.display = 'none';
	}

	// Update the stored playlist with current track plus remaining tracks
	MusicStorage.playing = CurrentTrack.path + SplitChar;
	while (track) {
		MusicStorage.playing += track.path + SplitChar;
		track = track.nextElementSibling;
	}
}

/**
 * Event handler to remove a track when clicked in the playlist
 * Used as onclick handler for tracks in the playlist
 */
function RemoveTrackDiv() {
	RemoveTrack(this);
}

/**
 * Adds a file to the playlist and starts playback if not already playing
 *
 * @param {HTMLElement} item - The audio item to play
 */
function PlayFile(item) {
	// Create a new playlist entry for this track
	const track = document.createElement('div');
	track.onclick = RemoveTrackDiv;
	track.className = 'Audio';
	track.innerHTML = item.innerHTML;  // Copy the HTML content
	track.path = item.path;            // Copy the path to the audio file

	// Add the track to the playlist
	Playlist.appendChild(track);

	// Update the stored playlist
	MusicStorage.playing += track.path + SplitChar;

	// Show the playlist panel and player controls
	PlaylistCell.style.display = '';
	PlayerDisplay.style.display = '';

	// Start playback if not already playing
	if (CurrentTrack) {
		// Preload the next track
		PreLoadNext();
	}
	else {
		PlayNext();
	}
}

/**
 * Ends playback of the current track
 * Called when a track finishes, when the user skips a track,
 * or when an error occurs during playback
 */
function EndPlay() {
	if (CurrentTrack) {
		// Stop playback
		CurrentTrack.pause();

		// Reset the track's visual state in the file list
		if (CurrentTrack.trackId) {
			const trackElement = _(CurrentTrack.trackId);
			if (trackElement) {
				trackElement.className = 'Audio';
			}
		}
	}

	// Clear current track and reset UI
	CurrentTrack = null;
	ProgressDiv.style.width = '0%';
	document.title = OriginalTitle;

	// Remove the first track from the saved playlist
	MusicStorage.playing = MusicStorage.playing.substr(MusicStorage.playing.indexOf(SplitChar) + 1);

	// Start playing the next track
	PlayNext();
}

/**
 * Skips to the next track in the playlist
 * Acts as the handler for the next track button
 */
function FastForward() {
	EndPlay();  // End current track, which will automatically start the next
}

/**
 * Restarts the current track from the beginning
 * Acts as the handler for the previous track button
 */
function Rewind() {
	if (CurrentTrack) {
		CurrentTrack.currentTime = 0;
	}
}

/**
 * Stops playback and clears the entire playlist
 * Used as handler for the "Eject All" button
 */
function ClearPlaylist() {
	// Remove all tracks from the playlist
	Playlist.innerHTML = '';

	// Hide the playlist panel
	PlaylistCell.style.display = 'none';

	// Clear the stored playlist
	MusicStorage.playing = [];

	// Stop playback
	EndPlay();
}

/**
 * Adds all files in the current directory to the playlist
 * Used as handler for the "Add All" button
 */
function AddAllToPlaylist() {
	let div = _('Files').firstChild;
	while (div) {
		AddToPlaylist(div);
		div = div.nextSibling;
	}
}

//==============================================================================
// ID3 tag handling
//==============================================================================

// ID3 formal documents are on id3.org site:  (Sometimes slow to respond)
// ID3v2.2: https://id3.org/id3v2-00
// ID3v2.3: https://id3.org/id3v2.3.0
// ID3v2.4: https://id3.org/id3v2.4.0-changes
//
// However, someone has collected some documents on github:
// https://github.com/id3/ID3v2.4 and https://github.com/id3/ID3v2.3
//
// This is a quick implementation of that and is definitely not complete.
// In fact, it is sure to have some problems but it seems to address what
// I currently need.

/**
 * Compute the size of ID3 frames - they are stored in 7-bit bytes
 * with the upper bit ignored.  Thus this is only 28 bits of size (plenty)
 *
 * @param {number} size1 - Highest byte of ID3 size
 * @param {number} size2 - Second byte of ID3 size
 * @param {number} size3 - Third byte of ID3 size
 * @param {number} size4 - Lowest byte of ID3 size
 * @returns {number} The calculated size value
 */
function ID3Size(size1, size2, size3, size4) {
	// 0x7f = 0b01111111
	const size = size4 & 0x7f
		| ((size3 & 0x7f) << 7)
		| ((size2 & 0x7f) << 14)
		| ((size1 & 0x7f) << 21);

	return size;
}

/**
 * Check for BOM and if there, decode as per the BOM
 * If no BOM, decode as per the fallback.
 *
 * @param {Uint8Array} buffer
 * @param {string} fallback
 * @returns {string} The decoded text string
 */
function DecodeUTF16(buffer, fallback) {
	// UTF-16 with BOM
	// Why do we chcek the BOM?  The browser should really do that but
	// some versions do not...
	if (buffer.length > 3) {
		// Check for UTF-16LE BOM (FF FE)
		if (buffer[1] === 0xFF && buffer[2] === 0xFE) {
			return new TextDecoder('utf-16le').decode(buffer.slice(3));
		}
		// Check for UTF-16BE BOM (FE FF)
		else if (buffer[1] === 0xFE && buffer[2] === 0xFF) {
			return new TextDecoder('utf-16be').decode(buffer.slice(3));
		}
	}

	return new TextDecoder(fallback).decode(buffer.slice(1));
}

/**
 * Convert ID3 T* text frames into a string based on encoding byte
 * May return strings with extra null characters at the end
 *
 * @param {Uint8Array} buffer - ID3 text frame data
 * @returns {string} The decoded text string
 */
function DecodeTextRaw(buffer) {
	const encodingByte = buffer[0];
	switch (encodingByte) {
		case 0x01:
			// UTF-16 with BOM
			return DecodeUTF16(buffer, 'utf-16');
		case 0x02:
			// UTF-16be
			return DecodeUTF16(buffer, 'utf-16be');
		case 0x03:
			// UTF-8
			return new TextDecoder('utf-8').decode(buffer.slice(1));
		default:
			// Latin-1 (default)
			return new TextDecoder('iso-8859-15').decode(buffer.slice(1));
	}
}

/**
 * Convert ID3 T* text frames into a string based on encoding byte
 *
 * @param {Uint8Array} buffer - ID3 text frame data
 * @returns {string} The decoded text string
 */
function DecodeText(buffer) {
	// Clean up the potential extra nulls at the end of the string
	// due to padding
	return DecodeTextRaw(buffer).replace(/\0+$/, '');
}

// These genres are in the ID3 specific ordering such that indexing this array
// by the ID3 genre index will get the right string
const ID3_GENRES = [
	"Blues", "Classic Rock", "Country", "Dance", "Disco", "Funk", "Grunge",
	"Hip-Hop", "Jazz", "Metal", "New Age", "Oldies", "Other", "Pop", "R&B",
	"Rap", "Reggae", "Rock", "Techno", "Industrial", "Alternative", "Ska",
	"Death Metal", "Pranks", "Soundtrack", "Euro-Techno", "Ambient",
	"Trip-Hop", "Vocal", "Jazz+Funk", "Fusion", "Trance", "Classical",
	"Instrumental", "Acid", "House", "Game", "Sound Clip", "Gospel",
	"Noise", "AlternRock", "Bass", "Soul", "Punk", "Space", "Meditative",
	"Instrumental Pop", "Instrumental Rock", "Ethnic", "Gothic",
	"Darkwave", "Techno-Industrial", "Electronic", "Pop-Folk",
	"Eurodance", "Dream", "Southern Rock", "Comedy", "Cult", "Gangsta",
	"Top 40", "Christian Rap", "Pop/Funk", "Jungle", "Native American",
	"Cabaret", "New Wave", "Psychadelic", "Rave", "Showtunes", "Trailer",
	"Lo-Fi", "Tribal", "Acid Punk", "Acid Jazz", "Polka", "Retro",
	"Musical", "Rock & Roll", "Hard Rock", "Folk", "Folk-Rock",
	"National Folk", "Swing", "Fast Fusion", "Bebop", "Latin", "Revival",
	"Celtic", "Bluegrass", "Avantgarde", "Gothic Rock", "Progressive Rock",
	"Psychedelic Rock", "Symphonic Rock", "Slow Rock", "Big Band",
	"Chorus", "Easy Listening", "Acoustic", "Humour", "Speech", "Chanson",
	"Opera", "Chamber Music", "Sonata", "Symphony", "Booty Bass", "Primus",
	"Porn Groove", "Satire", "Slow Jam", "Club", "Tango", "Samba",
	"Folklore", "Ballad", "Power Ballad", "Rhythmic Soul", "Freestyle",
	"Duet", "Punk Rock", "Drum Solo", "Acapella", "Euro-House", "Dance Hall"
];

/**
 * Convert ID3 TCON Genre data into a genre name from the standard list if possible
 *
 * @param {Uint8Array} frameData - The genre frame data including encoding byte
 * @returns {string} The genre name, either from the standard list or as raw text
 */
function DecodeGenre(frameData) {
	const gText = DecodeText(frameData);
	const id = gText.replace(/^\((\d+)\)$/, '$1');
	if (id) {
		const i = parseInt(id);
		if (i < ID3_GENRES.length) {
			return ID3_GENRES[i];
		}
	}
	return gText;
}

/**
 * Parses all ID3 frames from the provided buffer and extracts tag information
 * Note that we assume that the id3Buffer is the buffer that starts from the
 * front of the file and goes at least long enough to cover the whole tag area.
 * If the server supports ranges, it will only have that much of the file.
 *
 * @param {number} version - The ID3 version (2, 3, or 4 for ID3v2.2, v2.3, or v2.4)
 * @param {number} revision - The ID3 revision
 * @param {ArrayBuffer} id3Buffer - The array buffer that holds at least enough of the audio file to cover the ID3 data
 * @returns {Object} Dictionary of extracted tags (title, artist, album, etc.)
 */
function ReadID3Tags(version, revision, id3Buffer) {
	const tags = {};
	const size = id3Buffer.byteLength;
	const id3Bytes = new Uint8Array(id3Buffer, 0, size);
	let offset = 10
	while (offset < size - 10) {
		// Frame ID - ID3v2.2 uses 3-char IDs, ID3v2.3/4 use 4-char IDs
		let frameId;
		if (version === 2) { // ID3v2.2
			frameId = String.fromCharCode(
				id3Bytes[offset], id3Bytes[offset + 1], id3Bytes[offset + 2]
			);
			offset += 3;
		} else { // ID3v2.3 or ID3v2.4
			frameId = String.fromCharCode(
				id3Bytes[offset], id3Bytes[offset + 1], id3Bytes[offset + 2], id3Bytes[offset + 3]
			);
			offset += 4;
		}

		// Verify we have a valid frame ID (non-null bytes)
		if (frameId.trim() === '' || frameId.includes('\0')) {
			// Invalid frame ID, skip to next frame
			break;
		}

		// Map ID3v2.2 frame IDs to their ID3v2.3/4 equivalents
		if (version === 2) {
			switch (frameId) {
				case 'TT2': frameId = 'TIT2'; break; // Title
				case 'TP1': frameId = 'TPE1'; break; // Artist
				case 'TAL': frameId = 'TALB'; break; // Album
				case 'TRK': frameId = 'TRCK'; break; // Track number
				case 'TYE': frameId = 'TYER'; break; // Year
				case 'TCO': frameId = 'TCON'; break; // Genre
				case 'PIC': frameId = 'APIC'; break; // Picture
				case 'COM': frameId = 'COMM'; break; // Comment
				case 'ULT': frameId = 'USLT'; break; // Lyrics
			}
		}

		let frameSize = 0;
		switch (version) {
			case 2: // ID3v2.2
				frameSize = ((id3Bytes[offset] & 0x7F) << 16) | (id3Bytes[offset + 1] << 8) | id3Bytes[offset + 2];
				offset += 3;
				break;
			case 3: // ID3v2.3
				frameSize = (id3Bytes[offset] << 24) | (id3Bytes[offset + 1] << 16) | (id3Bytes[offset + 2] << 8) | id3Bytes[offset + 3];
				offset += 4;
				break;
			case 4: // ID3v2.4
				frameSize = ID3Size(id3Bytes[offset], id3Bytes[offset + 1], id3Bytes[offset + 2], id3Bytes[offset + 3]);
				offset += 4;
				break;
			default:
				throw new Error(`Unknown ID3 version: v2.${version} r${revision}`);
		}

		if (frameSize < 1) {
			break;
		}

		switch (version) {
			case 2:
				// ID3v2.2 has no flags field to skip
				break;
			case 3:
			case 4:
				offset += 2; // Skip flags
				break;
			default:
				throw new Error(`Unknown ID3 version: v2.${version} r${revision}`);
		}

		if (offset + frameSize > size) {
			console.error(`Size ${size} is less than offset ${offset} + frameSize ${frameSize}`)
			break;
		}
		const frameData = new Uint8Array(id3Buffer, offset, frameSize);
		offset += frameSize;

		if (frameId.startsWith('T')) {
			try {
				tags[frameId] = DecodeText(frameData);
			} catch (error) {
				tags[frameId] = '';
				console.error(`id: ${frameId} error: ${error}`);
			}
		}

		switch (frameId) {
			case 'TIT2': tags.title = tags[frameId]; break;
			case 'TPE1': tags.artist = tags[frameId]; break;
			case 'TALB': tags.album = tags[frameId]; break;
			case 'TRCK': tags.trackNumber = tags[frameId]; break;
			case 'TYER': tags.date = tags[frameId]; break;
			case 'TDRC': tags.date = tags[frameId]; break; //v2.4 replacement for TYER
			case 'TCON': tags.genre = tags[frameId]; break;
			case 'APIC':
				const encoding = frameData[0];
				let i = 1;

				if (version === 2) { // ID3v2.2 PIC frame
					// Skip 3-char image format (e.g., "JPG", "PNG")
					i += 3;
				} else { // ID3v2.3/4 APIC frame
					// Skip MIME type
					while (i < frameData.length && frameData[i] !== 0) i++;
					i++; // Skip null
				}

				if (i >= frameData.length) break;

				// Picture type
				const pictureType = frameData[i++];

				// Skip description - handling depends on encoding
				if (encoding === 0 || encoding === 3) { // Latin-1 or UTF-8
					while (i < frameData.length && frameData[i] !== 0) i++;
					i++; // Skip null
				} else if (encoding === 1 || encoding === 2) { // UTF-16 with/without BOM
					// For UTF-16, null terminator is two bytes (0x00 0x00)
					while (i < frameData.length - 1 && !(frameData[i] === 0 && frameData[i + 1] === 0)) i++;
					i += 2; // Skip double null
				}

				if (i >= frameData.length) break;

				// Image data is the rest of the buffer
				const imageData = frameData.slice(i);
				if (imageData.length > 0) {
					tags.coverData = imageData.buffer;
				}
				break;
			default:
				if (!frameId.startsWith('T')) {
					console.warn(`id: ${frameId} : unhandled`);
				}
		}
	}
	return tags;
}

/**
 * Asynchronously loads and parses ID3 tags from an audio element's source file.
 * This function provides a tags dictionary with the results.
 * It supports basic tags (title, artist, album, etc.) as well as all T* tags
 * that aren't directly handled, stored with their frame ID as the key.
 *
 * No errors are returned if any happen. A dictionary is always returned
 * but it may be empty or only contain some tags as a file may have some
 * or even zero tags. The caller must always be able to handle the lack
 * of any given tag or lack of any tag.
 *
 * Why async and await in this code?  While I usually like explicit management
 * of continuations and lambdas, in this case it seemed clearer to make use of
 * the async/await pattern.  These are still continuations implemented with a
 * form of lambdas but they are handled under the covers of the language.  This
 * is only possible in newer browsers/javascript engines but it is, at times,
 * well worth it for easy of expressing the process.
 *
 * @param {HTMLAudioElement} audioElement - The audio element to load tags for
 * @returns {Promise<Object>} Promise resolving to a dictionary of tag values
 */
async function LoadID3Tags(audioElement) {
	if (audioElement && audioElement.src) {
		if (audioElement.id3) return audioElement.id3;

		// It would be nice to have Audio elements know how to do this
		// as a core feature or to be able to at least leverage the already
		// loaded data from them but it seems that is not possible today.
		// So, we will just fetch the data ourselves
		try {
			// First, just get the header to see how big the ID3 tag area is
			const response = await fetch(audioElement.src, { headers: { Range: 'bytes=0-9' } });
			if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
			const headerBuffer = await response.arrayBuffer();
			const header = new Uint8Array(headerBuffer, 0, headerBuffer.byteLength);

			// Looks like ID3 header
			if (header[0] === 73 && header[1] === 68 && header[2] === 51) {
				const version = header[3];
				const revision = header[4];  // unused?

				const size = ID3Size(header[6], header[7], header[8], header[9]);

				// As long as we have a reasonable size, load it
				if (size > 10) {
					// We get from 0 again, just in case the server
					// does not support partial file loads.
					const id3response = await fetch(audioElement.src, { headers: { Range: `bytes=0-${size + 10}` } });
					if (!id3response.ok) throw new Error(`HTTP error! status: ${id3response.status}`);
					const id3Buffer = await id3response.arrayBuffer();

					audioElement.id3 = ReadID3Tags(version, revision, id3Buffer);
					return audioElement.id3;
				}
			}
		} catch (error) {
			console.error('Error reading ID3 tags:', error);
		}
	}
	else {
		console.error('Audio element has no source.');
	}
	return {};
};