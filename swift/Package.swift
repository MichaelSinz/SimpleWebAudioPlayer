// swift-tools-version: 6.0

import PackageDescription

let package = Package(
    name: "Waver",
    platforms: [.macOS(.v10_15)],
    dependencies: [
        // other dependencies
        // Could this make 4-color (2-bits per pixel) images (smaller file size)?
        // .package(url: "https://github.com/tayloraswift/swift-png", from: "4.4.8"),
        .package(url: "https://github.com/apple/swift-argument-parser", from: "1.5.0")
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .executableTarget(
            name: "Waver",
            dependencies: [
                // .product(name: "PNG", package: "swift-png"),
                .product(name: "ArgumentParser", package: "swift-argument-parser")
            ]
        )
    ]
)
