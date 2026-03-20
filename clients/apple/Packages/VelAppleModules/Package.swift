// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "VelAppleModules",
    platforms: [
        .iOS(.v16),
        .watchOS(.v9),
        .macOS(.v13)
    ],
    products: [
        .library(name: "VelDomain", targets: ["VelDomain"]),
        .library(name: "VelApplication", targets: ["VelApplication"]),
        .library(name: "VelInfrastructure", targets: ["VelInfrastructure"]),
        .library(name: "VelUIShared", targets: ["VelUIShared"]),
        .library(name: "VelApplePlatform", targets: ["VelApplePlatform"]),
        .library(name: "VelFeatureFlags", targets: ["VelFeatureFlags"]),
        .library(name: "VelEmbeddedBridge", targets: ["VelEmbeddedBridge"])
    ],
    targets: [
        .target(name: "VelDomain"),
        .target(name: "VelFeatureFlags"),
        .target(
            name: "VelEmbeddedBridge",
            dependencies: [
                "VelDomain",
                "VelFeatureFlags"
            ]
        ),
        .target(
            name: "VelApplication",
            dependencies: [
                "VelDomain",
                "VelFeatureFlags",
                "VelInfrastructure",
                "VelApplePlatform",
                "VelEmbeddedBridge"
            ]
        ),
        .target(
            name: "VelInfrastructure",
            dependencies: ["VelDomain"]
        ),
        .target(
            name: "VelUIShared",
            dependencies: ["VelFeatureFlags"]
        ),
        .target(
            name: "VelApplePlatform",
            dependencies: ["VelFeatureFlags"]
        )
    ]
)
