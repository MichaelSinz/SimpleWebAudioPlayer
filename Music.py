#!/usr/bin/env python3
"""
Simple Web Audio Player - Python Implementation
Copyright Michael Sinz

This script scans a directory structure for MP3, Cover.jpg, and Back.jpg files,
and generates a JavaScript representation of the music library. It creates a
hierarchical tree structure that matches the filesystem organization.

The script performs three main operations:
1. Walking the directory tree to find music files and cover images
2. Pruning folders that don't contain playable content
3. Outputting the tree as browser-compatible JavaScript

Features:
- Uses efficient os.walk() for directory traversal
- Detects MP3 files and cover images
- Prunes folders without playable content (no MP3s directly or in subfolders)
- Builds a clean, hierarchical data structure
- Outputs browser-compatible JavaScript with strategic line breaks
- Produces output with deterministic ordering (via sorted keys)

Output Structure:
The generated JavaScript defines a 'mp3' variable containing a nested object with:
- 'Files': Arrays of MP3 filenames (without the .mp3 extension)
- 'Cover': Value 1 if Cover.jpg exists, 2 if both Cover.jpg and Back.jpg exist
- 'Folders': Nested objects representing subdirectories with music content

Usage: python3 Music.py > Music.py.js
"""

import os
import re
import json


def build_music_tree():
    """
    Build a music tree structure by efficiently walking the filesystem.

    This function uses os.walk() for efficient directory traversal, which makes
    fewer system calls than recursive approaches. It builds a hierarchical
    structure representing the music library, with each node potentially
    containing files, cover image information, and subdirectories.

    Returns:
        dict: A hierarchical dictionary with the following possible keys:
            - 'Files': List of MP3 filenames (without extension)
            - 'Cover': 1 if Cover.jpg exists, 2 if both Cover.jpg and Back.jpg exist
            - 'Folders': Dictionary of subdirectories, each with its own tree structure
    """
    # Initialize empty tree
    tree = {}

    # Get the absolute path of the current directory to use as reference
    # for creating relative paths during directory traversal
    root_dir = os.path.abspath('.')
    root_dir_len = len(root_dir) + 1  # +1 for the path separator

    # Walk the directory tree in a single pass
    # os.walk() is much more efficient than recursive glob methods
    for dirpath, dirnames, filenames in os.walk('.', followlinks=True):
        # Skip hidden directories by modifying dirnames in place
        # This prevents os.walk() from descending into hidden directories
        dirnames[:] = [d for d in dirnames if not d.startswith('.')]

        # Skip empty directories (no files and no subdirectories)
        if not dirnames and not filenames:
            continue

        # Convert the current directory path to a relative path for our tree
        # First get the absolute path, then extract the part relative to root_dir
        abs_path = os.path.abspath(dirpath)
        # Remove the root directory prefix to get a relative path
        rel_path = abs_path[root_dir_len:] if abs_path.startswith(root_dir) else abs_path

        # Skip the root directory itself (empty rel_path)
        if not rel_path:
            continue

        # Split the relative path into components for tree navigation
        path_parts = rel_path.split(os.sep)

        # Start at the root of the tree
        node = tree

        # Navigate to (or create) the node for the current directory
        for part in path_parts:
            # Ensure the Folders dictionary exists
            if 'Folders' not in node:
                node['Folders'] = {}
            # Create this folder node if it doesn't exist
            if part not in node['Folders']:
                node['Folders'][part] = {}
            # Move to this folder's node
            node = node['Folders'][part]

        # Process files in this directory
        mp3_files = []
        has_cover = False
        has_back = False

        # Check each file in the current directory
        for filename in filenames:
            # Skip hidden files
            if filename.startswith('.'):
                continue

            # Identify file types
            if filename == 'Cover.jpg':
                has_cover = True
            elif filename == 'Back.jpg':
                has_back = True
            elif filename.endswith('.mp3'):
                # Store MP3 filename without the extension
                mp3_files.append(filename[:-4])

        # Set Cover property based on which files were found
        if has_cover and has_back:
            node['Cover'] = 2  # Both Cover.jpg and Back.jpg exist
        elif has_cover:
            node['Cover'] = 1  # Only Cover.jpg exists

        # Add MP3 files if any were found (sorted alphabetically)
        if mp3_files:
            node['Files'] = sorted(mp3_files)

    return tree


def prune_empty_folders(node):
    """
    Recursively remove empty folders from the tree.

    This function traverses the tree bottom-up, removing directories that don't
    contain music files or useful subdirectories. This keeps the output focused
    on actual music content without empty folder clutter.

    A folder is kept only if it has:
    - MP3 files (has a 'Files' entry), OR
    - Subdirectories that themselves contain playable content

    Folders with only cover images but no playable content are pruned.

    Args:
        node (dict): The node to process

    Returns:
        bool: True if the node has playable content, False if it's empty
    """
    has_playable_content = False

    # If the node has Files, it has playable content
    if 'Files' in node:
        has_playable_content = True

    # Process subfolders if they exist
    if 'Folders' in node:
        # Check all subfolders and remove empty ones
        # Use list() to create a copy since we'll be modifying the dictionary
        non_empty_folders = {}
        for folder_name, folder_node in list(node['Folders'].items()):
            # Recursively process the subfolder
            if prune_empty_folders(folder_node):
                # Keep this folder if it has playable content
                non_empty_folders[folder_name] = folder_node
                has_playable_content = True

        # Update the Folders entry with only non-empty folders
        if non_empty_folders:
            node['Folders'] = non_empty_folders
        else:
            # If all subfolders were empty, remove the Folders entry
            del node['Folders']

    # If no playable content was found, the entire node will be discarded
    # by the parent call
    return has_playable_content


def format_output(tree):
    """
    Format the tree structure as JavaScript with appropriate formatting.

    This function converts the tree dictionary to JSON with specific formatting
    choices to ensure the output is browser-compatible and readable.

    Args:
        tree (dict): The hierarchical tree structure

    Returns:
        str: Formatted JavaScript string representing the tree
    """
    # Create header for the JavaScript file
    js_header = '// Simple Web Audio Player - Copyright Michael Sinz\n\n'
    js_header += 'const mp3 = '

    # Convert to JSON with specific formatting options:
    # - sort_keys=True: Ensures deterministic output order (important for diffs/comparisons)
    # - ensure_ascii=False: Preserves non-ASCII characters in output
    # - separators=(',', ':'): Minimizes whitespace for compact output
    json_str = json.dumps(tree, sort_keys=True, ensure_ascii=False, separators=(',', ':'))

    # Add line breaks after closing braces with commas for better readability
    # and browser compatibility (prevents excessively long lines)
    json_str = re.sub(r'},', '},\n', json_str)

    # Complete the JavaScript statement with semicolon
    return js_header + json_str + ';'


def main():
    """
    Main function to run the script.

    This function coordinates the three main steps of the process:
    1. Building the initial tree structure
    2. Pruning empty folders from the tree
    3. Formatting and outputting the final JavaScript
    """
    try:
        # Build the tree structure by walking the directory tree
        tree = build_music_tree()

        # Prune empty folders from the tree
        prune_empty_folders(tree)

        # Format the tree as JavaScript and print to stdout
        js_output = format_output(tree)
        print(js_output, end='')

    except Exception as e:
        # Handle any unexpected errors
        import sys
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
