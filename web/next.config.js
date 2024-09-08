const fs = require("fs");
const webpack = require("webpack");

const withBundleAnalyzer = require("@next/bundle-analyzer")({
  enabled: process.env.ANALYZE === "true",
});

const NEXT_PUBLIC_VERSION = fs.readFileSync("../VERSION", "utf8").trim();

const nextConfig = {
  env: {
    NEXT_PUBLIC_VERSION,
  },
  output: "export",
  trailingSlash: true,
  webpack: (config, { isServer }) => {
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
      layers: true,
    };

    if (isServer) {
      config.plugins.push(
        new webpack.NormalModuleReplacementPlugin(/pkg$/, "src/pkg_mock.js"),
      );
    } else {
      config.output.environment = {
        ...config.output.environment,
        asyncFunction: true,
      };
    }

    return config;
  },
};

module.exports = withBundleAnalyzer(nextConfig);
