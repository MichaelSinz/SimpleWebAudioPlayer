// Simple hack program that, on MacOS with Swift, reads audio file (MP3/etc)
// and produces a PNG file that represents its waveform visually.

// This specifically uses streaming such that the whole audio does not
// need to be loaded into memory at once.  This allows the processing of
// some large/long audio files at the speed of decoding them without
// significant memory overhead.  Because of this we can then safely
// parallelize the operations such that multiple audio files can be
// processed at once.

// We use the swift argument parser library to have declarative arguments
// and argument validation handled in both a standard and simple way

import AVFoundation
import AppKit
import ArgumentParser
import Foundation


/// Waver Command Line Tool
/// The main command line tool struct for converting audio files to waveform
/// PNG images.
///
/// This tool accepts various command line arguments to customize the output of
/// the waveforms, such as image dimensions, colors for different channels and
/// background, buffer size for streaming, and more.
@main
struct Waver: ParsableCommand {
    @Option(help: "Image Width")
    var width: Int = 2048

    @Option(help: "Image Height (must be even)")
    var height: Int = 128

    @Option(help: "Left channel (and mono) color in RGB/RRGGBB/RRGGBBAA hex")
    var leftColor = "00ff99"

    @Option(help: "Right channel color in RGB/RRGGBB/RRGGBBAA hex")
    var rightColor = "99ff00"

    @Option(help: "Background color in RGB/RRGGBB/RRGGBBAA hex")
    var backgroundColor = "ffffff00"

    @Option(name: .shortAndLong, help: "Optional output file name (defaults to input+.png)")
    var outputFilename = ""

    @Option(help: "File extensions to process when given a directory")
    var fileExtensions = ["mp3", "m4a"]

    // Constrain our audio reading buffer size such that we stream
    // the audio but not in too small of chunk sizes and not too large.
    //
    // If you set this too large, the memory load and cache performance
    // can be impacted.  If you set it too small, the number stream read
    // requests go up and can impact performance.
    //
    // Perf Tests - when processing a directory tree with 579 MP3 files that
    // total around 3.4GB in size, all in highest quality variable bit rate
    // encodings, we get the following performance on my MacBookPro:
    // (Measured after multiple repeated runs with the same buffer size,
    // taking the best run which usually was the last run)
    //
    //       4194303        131071         65535         32767          4095  --buffer-size
    // ------------- ------------- ------------- ------------- -------------  -------------------------
    //          9.35          8.79          8.64          8.77          9.07  real time
    //        139.18        137.43        135.36        136.81        136.76  user (cpu user time)
    //          4.12          0.77          0.75          1.02          5.11  sys (cpu OS time)
    //    2806333440     206422016     162676736     141836288     123486208  maximum resident set size
    // 1956819286348 1944140432707 1944272337536 1946504453175 1979054857635  instructions retired
    //  511558401161  493050273427  485401415345  482658543430  506685982189  cycles elapsed
    //    2802748008     201704432     157926360     137069504     118653792  peak memory footprint
    //
    // Note how increased buffer sizes slow things down due to the impact on
    // CPU cache and smaller sizes slow things down due to number of additional
    // read stream calls.
    //
    // The peak performance is somewhere between 32K and 128K.  In order to test
    // all of this, I had it as a parameter but have the default value at the
    // peak performance point.
    //
    // I have this as an option so that it can be tuned for performance
    // and experimented with.  I have also found some cases where the
    // AVAudioFile.length is not accurate and reading until that length
    // can fail.  This is actually more common than I expected.  See comments
    // in the code where I handle those conditions.
    @Option(help: "Buffer size to use while streaming audio (in frames)")
    var bufferSize = AVAudioFrameCount(65535)

    @Flag(help: "Do not write anything")
    var dryRun = false

    @Flag(help: "Overwrite existing output file")
    var overwrite = false

    @Flag(help: "Quieter output")
    var quiet = false

    @Flag(help: "Verbose - show overwrite warnings")
    var verbose = false

    // The audio file(s) or directory to process.
    @Argument(help: "The audio file to process (or files/directory if not using --output-filename)")
    var audioFilenames: [String]

    /// An error type for handling argument validation issues.
    struct ArgumentError: LocalizedError {
        let description: String

        init(_ description: String) {
            self.description = description
        }

        var errorDescription: String? {
            description
        }
    }

    /// An error type for handling waveform generation issues.
    struct GenerationError: LocalizedError {
        let description: String

        init(_ description: String) {
            self.description = description
        }

        var errorDescription: String? {
            description
        }
    }

    /// Converts a hex color string to a `CGColor` object.
    /// Supports these RGB formats:
    ///  RGB      - transforms to RRGGBB
    ///  RRGGBB   - assumes alpha is 1.0
    ///  RRGGBBAA - alpha value is 00 to FF (0.0 to 1.0)
    ///
    /// - Parameter rgbString: hex string representing the color (RGB, RRGGBB, or RRGGBBAA).
    /// - Returns: A `CGColor` object.
    /// - Throws: An `ArgumentError` if the color string is invalid.
    func cgColorFrom(rgbString: String) throws -> CGColor {
        guard
            let rgb = UInt(rgbString, radix: 16)
        else {
            throw ArgumentError(
                "Invalid color format: '\(rgbString)' - invalid hex number")
        }
        // Simple 3 digit RGB color
        if rgbString.count == 3 {
            return CGColor(
                red: (CGFloat)(((rgb & 0x0F00) >> 8) * 17) / 255.0,
                green: (CGFloat)(((rgb & 0x00F0) >> 4) * 17) / 255.0,
                blue: (CGFloat)((rgb & 0x000F) * 17) / 255.0,
                alpha: 1.0)
        }
        // Common 6 digit RGB color
        if rgbString.count == 6 {
            return CGColor(
                red: (CGFloat)((rgb & 0x00FF_0000) >> 16) / 255.0,
                green: (CGFloat)((rgb & 0x0000_FF00) >> 8) / 255.0,
                blue: (CGFloat)(rgb & 0x0000_00FF) / 255.0,
                alpha: 1.0)
        }
        // Common 8 digit RGBA color
        if rgbString.count == 8 {
            return CGColor(
                red: (CGFloat)((rgb & 0x00_FF00_0000) >> 24) / 255.0,
                green: (CGFloat)((rgb & 0x00_00FF_0000) >> 16) / 255.0,
                blue: (CGFloat)((rgb & 0x00_0000_FF00) >> 8) / 255.0,
                alpha: (CGFloat)(rgb & 0x00_0000_00FF) / 255.0)
        }
        // Unknown format
        throw ArgumentError(
            "Invalid color format: '\(rgbString)' - We support RGB, RRGGBB, and RRGGBBAA hex formats"
        )
    }

    /// Validates the command-line arguments provided.  It can not mutate
    /// anything, just success or throws an error.  Automatically called
    /// by the argument parser after completing the parsing of the command
    /// line arguments and before calling the run() entry point.
    ///
    /// - Throws: An `ArgumentError` if any validation fails.
    func validate() throws {
        // Since validation can't actually save the results, just checks, we
        // just try to convert each color argument to validate that they do
        // correctly convert during validation such that we don't have to do it
        // later.
        _ = try cgColorFrom(rgbString: leftColor)
        _ = try cgColorFrom(rgbString: rightColor)
        _ = try cgColorFrom(rgbString: backgroundColor)

        if width < 16 {
            throw ArgumentError("--width must be at least 16 pixels")
        }

        // We need the height to be even (left/right channels) and at least 6 pixels
        if height < 6 || height & 1 == 1 {
            throw ArgumentError("--height must be at least 6 pixels and even")
        }

        if bufferSize < 1024 {
            throw ArgumentError("--buffer-size must be at least 1024 frames")
        }

        if audioFilenames.count < 1 {
            throw ArgumentError("No audio file specified")
        }

        // Check that output filename is not specified with multiple audio files
        if audioFilenames.count > 1 && outputFilename.count > 0 {
            throw ArgumentError("Cannot specify --output-filename with multiple audio files")
        }

        // Check that audio files/directories exist
        for filename in audioFilenames {
            var isDirectory: ObjCBool = false
            if !FileManager.default.fileExists(atPath: filename, isDirectory: &isDirectory) {
                throw ArgumentError("Could not find audio-filename: \(filename)")
            }
            // Check that output filename is not specified with a directory
            if outputFilename.count > 0 && isDirectory.boolValue {
                throw ArgumentError("Cannot specify --output-filename with a directory")
            }
        }
    }

    /// Prints messages to standard error.
    ///
    /// - Parameter items: The items to print.
    /// - Parameter separator: A string to separate the printed items (default is space).
    /// - Parameter terminator: The character(s) to print after the last item (default is newline).
    func printToStdErr(_ items: Any..., separator: String = " ", terminator: String = "\n") {
        if !quiet {
            let output =
                items
                .map { String(describing: $0) }
                .joined(separator: separator) + terminator

            FileHandle.standardError.write(output.data(using: .utf8)!)
        }
    }

    /// Generates a waveform PNG image from an audio file.
    ///
    /// This method reads audio data in chunks (streaming) to minimize memory usage,
    /// processes each chunk to compute the waveform, and draws it onto a Core Graphics context.
    ///
    /// - Parameters:
    ///   - inputFile: The path to the input audio file.
    ///   - outputFile: The path where the output PNG image should be saved.
    ///   - colors: An array of `CGColor` objects representing the left channel, right channel,
    ///     and background colors in that order.
    /// - Throws: A `GenerationError` if any issues occur during processing, such as file I/O errors
    ///           or invalid audio formats.
    func generateWavePng(
        inputFile: String,
        outputFile: String,
        colors: [CGColor]
    ) throws {
        // We were going to make the image and PNG as a 2-bit (4 color) image
        // but MacOS libraries no longer seem to support that.  This results in
        // PNG files that are a bit larger (about twice as large) as they would
        // be if they were 2bpp.  But they are rather small anyway.

        // Check for existing output file if overwriting is not allowed
        if !overwrite && FileManager.default.fileExists(atPath: outputFile) {
            if !verbose {
                return
            }
            throw GenerationError(
                "Output file '\(outputFile)' already exists - use --overwrite to overwrite")
        }
        do {
            // Open the MP3 file without loading it fully into memory
            guard
                let audioFile = try? AVAudioFile(forReading: URL(fileURLWithPath: inputFile))
            else {
                throw GenerationError("Could not open for audio processing")
            }

            guard
                let buffer = AVAudioPCMBuffer(
                    pcmFormat: audioFile.processingFormat,
                    frameCapacity: bufferSize)
            else {
                throw GenerationError("Could not create buffer for audio file")
            }

            let imageWidth = CGFloat(width)
            let imageHeight = CGFloat(height)
            let imageCenter = imageHeight / 2
            let pixelsPerSample = imageWidth / Double(audioFile.length)

            // Get our rendering context for an image of our size
            guard
                let context = CGContext(
                    data: nil,
                    width: width,
                    height: height,
                    bitsPerComponent: 8,  // 8 bits per component
                    bytesPerRow: 0,  // Let CGContext define this
                    space: CGColorSpace(name: CGColorSpace.sRGB)!,
                    bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)
            else {
                throw GenerationError("Could not create rendering context")
            }

            // Set the background color and fill the entire image with it
            context.setFillColor(colors[colors.count - 1])
            context.fill(CGRect(origin: .zero, size: CGSize(width: imageWidth, height: imageHeight)))

            // Tracks the maximum values for each channel
            var left: Float = 0.0
            var right: Float = 0.0

            // Track our progress along the graph.  We use fractional
            // pixel progress as the samples are unlikely to be a perfect
            // multiple of the image width.
            var pixelProgress = CGFloat(0.0)
            var pixelPos = CGFloat(0.0)
            var nextPixel = pixelPos + 1.0

            // Stream the audio data in chunks and render the waveform
            while audioFile.framePosition < audioFile.length {
                let beforePos = audioFile.framePosition
                do {
                    try audioFile.read(into: buffer)
                } catch {
                    // Some audio terminate a bit earlier than expected and
                    // if the buffer size happens to match a multiple of the
                    // early termination point then we did not detect it at
                    // the bottom of the loop due to partial buffer fill and
                    // have to notice it here.
                    // If we are under a buffer length left over, it was likely
                    // this same problem and we just happened to have a buffer
                    // size that was a whole fraction of the early termination
                    // point.  What this really means is that the length of
                    // an AVAudioFile looks to be inaccurate at times.  (Likely
                    // due to VBR MP3 files, which is what I have)
                    // So, in this case, we accept it as "it is what it is"
                    // with optional verbose "warning" being logged
                    if audioFile.length - beforePos < bufferSize {
                        if verbose {
                            printToStdErr("Warning: Premature end reading '\(inputFile)' @ \(beforePos)/\(audioFile.length) \(Double(beforePos*10000/audioFile.length)/100.0)% : \(error.localizedDescription)")
                        }
                        break
                    } else {
                        throw GenerationError(
                            "Error reading @ \(beforePos)/\(audioFile.length): \(error.localizedDescription)"
                        )
                    }
                }

                let channelCount = min(2, Int(buffer.format.channelCount))
                let frameCount = buffer.frameLength

                guard
                    let channelData = buffer.floatChannelData
                else {
                    break
                }

                // Loop over each frame (sample) in the buffer keeping track of
                // the maximum value for each channel.  We do this until we have
                // combined enough samples to fill a single pixel width of the
                // image.  Then we render the pixel and reset the maximum values
                // back to 0.
                for frame in 0..<Int(frameCount) {
                    left = max(left, min(1.0, abs(channelData[0][frame])))
                    if channelCount > 1 {
                        right = max(right, min(1.0, abs(channelData[1][frame])))
                    }
                    pixelProgress += pixelsPerSample
                    if pixelProgress > nextPixel {
                        // Render this pixel of the image...
                        context.setFillColor(colors[0])
                        context.fill(
                            CGRect(
                                x: pixelPos, y: imageCenter, width: 1,
                                height: round(imageCenter * CGFloat(left))))
                        if channelCount < 2 {
                            right = left
                        }
                        else {
                            context.setFillColor(colors[1])
                        }
                        context.fill(
                            CGRect(
                                x: pixelPos, y: imageCenter, width: 1,
                                height: -round(imageCenter * CGFloat(right))))

                        // Get ready for the next pixel
                        pixelPos = nextPixel
                        nextPixel += 1.0
                        left = 0.0
                        right = 0.0
                    }
                }

                if buffer.frameLength < bufferSize {
                    // Render the last pixel in the wave for
                    // those cases where we did not get to a
                    // perfect multiple of the image size.
                    if pixelPos < imageWidth {
                        context.setFillColor(colors[0])
                        context.fill(
                            CGRect(
                                x: pixelPos, y: imageCenter, width: 1,
                                height: round(imageCenter * CGFloat(left))))
                        if channelCount < 2 {
                            right = left
                        }
                        else {
                            context.setFillColor(colors[1])
                        }
                        context.fill(
                            CGRect(
                                x: pixelPos, y: imageCenter, width: 1,
                                height: -round(imageCenter * CGFloat(right))))
                    }
                    // We are done with the file - we should not need to break
                    // out but it turns out AVAudioFile.length is not always
                    // accurate enough and we may think there are a few more
                    // samples to get but, really, we are done since we got
                    // a partial read.  It is faster for us to just break out
                    // here rather than do the next read which will throw and
                    // then break out.
                    break
                }
            }

            // Generate the final image
            let image = context.makeImage()!

            // If we are not dry-run, write the image to the file.
            if !dryRun {
                guard
                    let pngData = NSBitmapImageRep(cgImage: image).representation(
                        using: NSBitmapImageRep.FileType.png, properties: [:])
                else {
                    throw GenerationError("Could not create PNG representation of image")
                }

                try pngData.write(to: URL(fileURLWithPath: outputFile))
                if !quiet {
                    print("Created \(outputFile)")
                }
            }
            else if verbose {
                    print("DryRun \(outputFile)")
            }
        } catch {
            throw GenerationError("Error processing '\(inputFile)' : \(error.localizedDescription)")
        }
    }

    /// Recursively processes files in a directory tree.
    ///
    /// This method traverses the directory structure starting at the given URL and applies
    /// the provided handler closure to each matching file.
    ///
    /// - Parameters:
    ///   - path: The URL pointing to a file or directory to process.
    ///   - handler: A closure that processes each matching file.  The closure receives a `URL`
    ///             pointing to the file to be processed.
    /// - Throws: A `GenerationError` if there are issues accessing files or directories.
    func handleFiles(path: URL, handler: (URL) -> Void) throws {
        let fileManager = FileManager.default

        // Check if the directory exists or is a file
        var isDirectory: ObjCBool = false
        guard
            fileManager.fileExists(atPath: path.path, isDirectory: &isDirectory)
        else {
            throw GenerationError("Directory does not exist: \(path.path)")
        }
        if !isDirectory.boolValue {
            return handler(path)  // If it is not a directory, just call the handler
        }

        // It is a directory.  Get the contents of the directory
        // and process them.
        let items = try fileManager.contentsOfDirectory(at: path, includingPropertiesForKeys: nil)

        for item in items {
            // If the item is a directory, recursively call the function
            if item.hasDirectoryPath {
                try handleFiles(path: item, handler: handler)
            } else {
                // Check if the item matches the pattern
                for fileExtension in fileExtensions {
                    if item.pathExtension == fileExtension {
                        handler(item)  // Call the handler with the matching file
                        break
                    }
                }
            }
        }
    }

    /// The main execution method that runs the Waver tool.
    ///
    /// - Throws: A `GenerationError` if any issues occur during processing.
    func run() throws {
        // This is the main code - after all of the options have been
        // parsed and somewhat validated.

        // Note that since validation already checked these, the "try" will
        // always work here.  (The validation would have failed if they were
        // invalid and thus run() would never have been called.)
        let colors = [
            try cgColorFrom(rgbString: leftColor),  // User left channel color (and mono)
            try cgColorFrom(rgbString: rightColor),  // User right channel color
            try cgColorFrom(rgbString: backgroundColor),  // User background color (last element)
        ]

        // We need a queue group to wait for work to finish...
        let workGroup = DispatchGroup()
        // We need a dispatch group that lets work run concurrently...
        let workQueue = DispatchQueue(label: "waver.workQueue", attributes: .concurrent)

        // Safely accumulate errors from concurrent file processing
        // We need a safe way to accumulate errors from the generators
        // Unfortunately, Swift does not have such a safe thing built in
        // and building it safely still requires users to mark it as unsafe.
        class ErrorList {
            private var strings: [String] = ["while processing files:"]
            private let lock = NSLock()

            // Method to append a string in a thread-safe manner
            func safeAppend(_ string: String) {
                lock.lock()
                defer { lock.unlock() }
                strings.append(string)
            }

            func hasErrors() -> Bool {
                lock.lock()
                defer { lock.unlock() }
                return strings.count > 1
            }

            func getText() -> String {
                lock.lock()
                defer { lock.unlock() }
                return strings.joined(separator: "\n* ")
            }
        }

        // This is safe when used to append to the list
        nonisolated(unsafe) let errors = ErrorList()

        for path in audioFilenames {
            do {
                // We run handleFiles which recursively processes a path
                // if it is a directory.  In all cases, it then calls
                // our closure (lambda) for each of the files.
                try handleFiles(path: URL(fileURLWithPath: path)) { fileURL in
                    // At this point we have a single file name
                    // as passed to us from the handleFiles function.
                    // We use the dispatch queue to dispatch the processing
                    // of that file here.  This way we don't actually do the
                    // work directly but just queue the work and let each
                    // file be processed in parallel with other files.
                    let filename = fileURL.path
                    workQueue.async(group: workGroup) {
                        do {
                            try generateWavePng(
                                inputFile: filename,
                                // If output filename given, use it, otherwise generate one
                                outputFile: (outputFilename.count > 0) ? outputFilename : filename + ".png",
                                colors: colors)
                        } catch {
                            let errorText = "\(error.localizedDescription)"
                            printToStdErr(errorText)
                            errors.safeAppend(errorText)
                        }
                    }
                }
            } catch {
                let errorText = "Error handling '\(path)' : \(error.localizedDescription)"
                printToStdErr(errorText)
                errors.safeAppend(errorText)
            }
        }

        // Wait for all file processing to complete
        workGroup.wait()

        // Throw an error if any issues occurred during processing
        if errors.hasErrors() {
            // If there are errors, print a blank line to split the output
            // and then throw a detailed error text that will be rendered
            // by the argument parser entry point.
            printToStdErr("")
            throw GenerationError(errors.getText())
        }
    }
}
