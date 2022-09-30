// swift-tools-version:5.5.0

import PackageDescription

let package = Package(
    name: "platformMac",
    platforms: [
        .macOS(.v11),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "platformMac",
            type: .static,
            targets: ["platformMac"]),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        .package(name: "SwiftRs", path: "../../../swift-rs/")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "platformMac",
            dependencies: [.product(name: "SwiftRs", package: "SwiftRs")],
            path: "src")
    ]
)