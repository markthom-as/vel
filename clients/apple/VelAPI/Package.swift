// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "VelAPI",
    platforms: [
        .iOS(.v16),
        .watchOS(.v9),
        .macOS(.v13),
    ],
    products: [
        .library(name: "VelAPI", targets: ["VelAPI"]),
    ],
    targets: [
        .target(
            name: "VelAPI",
            path: "Sources/VelAPI"
        ),
    ]
)
