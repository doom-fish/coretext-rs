// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "CoreTextBridge",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "CoreTextBridge",
            type: .static,
            targets: ["CoreTextBridge"]
        )
    ],
    targets: [
        .target(
            name: "CoreTextABI",
            path: "Sources/CoreTextABI",
            publicHeadersPath: "include"
        ),
        .target(
            name: "CoreTextBridge",
            dependencies: ["CoreTextABI"],
            path: "Sources/CoreTextBridge"
        )
    ]
)
