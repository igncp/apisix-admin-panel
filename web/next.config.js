// eslint-disable-next-line @typescript-eslint/no-var-requires
const webpack = require("webpack");

const nextConfig = {
  output: "export",
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

module.exports = nextConfig;
