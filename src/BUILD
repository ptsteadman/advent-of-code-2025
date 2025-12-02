load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

# rust_binary(
#     name = "day_1",
#     srcs = ["day_1.rs"],
#     # data = ["day_1_input.txt"],
#     visibility = ["//visibility:public"],
# )

rust_library(
    name = "day_1_lib",
    srcs = ["day_1.rs"],
    visibility = ["//visibility:public"],
)

rust_test(
    name = "day_1_test",
    crate = ":day_1_lib",
    data = ["day_1_input_simple.txt"],
    visibility = ["//visibility:public"],
    deps = [
        ":day_1_lib",
        "@rules_rust//rust/runfiles",
    ],
)

rust_binary(
    name = "hello_world",
    srcs = [
        "day_1.rs",
        "main.rs",
    ],
    visibility = ["//visibility:public"],
    deps = [":day_1_lib"],
)
