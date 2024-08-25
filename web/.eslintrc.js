const webPackageJson = require("./package.json");

const paddingRule = [
  "error",
  { blankLine: "always", next: "return", prev: "*" },
]
  .concat(
    [
      "const",
      "if",
      "interface",
      "multiline-block-like",
      "multiline-const",
      "multiline-expression",
      "type",
    ]
      .map((item) => [
        { blankLine: "always", next: "*", prev: item },
        { blankLine: "always", next: item, prev: "*" },
      ])
      .flat(),
  )
  .concat([
    {
      blankLine: "any",
      next: ["singleline-const"],
      prev: ["singleline-const"],
    },
  ]);

const disabledRules = {
  "@typescript-eslint/ban-ts-comment": 0,
  "@typescript-eslint/explicit-module-boundary-types": 0,
  "@typescript-eslint/member-ordering": 0,
  "@typescript-eslint/no-empty-function": 0,
  "@typescript-eslint/no-floating-promises": 0,
  "@typescript-eslint/no-non-null-assertion": 0,
  "@typescript-eslint/no-unsafe-argument": 0,
  "@typescript-eslint/no-unsafe-assignment": 0,
  "@typescript-eslint/no-unsafe-call": 0,
  "@typescript-eslint/no-unsafe-member-access": 0,
  "@typescript-eslint/no-unsafe-return": 0,
  "@typescript-eslint/prefer-nullish-coalescing": 0,
  "@typescript-eslint/restrict-plus-operands": 0,
  "@typescript-eslint/restrict-template-expressions": 0,
  "@typescript-eslint/unbound-method": 0,

  "no-restricted-globals": 0,

  "react/no-unescaped-entities": 0,
  "react/prop-types": 0,
  "react/react-in-jsx-scope": 0,
};

const tsRules = {
  "@typescript-eslint/array-type": [2, { default: "array-simple" }],
  "@typescript-eslint/consistent-type-imports": 2,
  "@typescript-eslint/explicit-member-accessibility": 2,
  "@typescript-eslint/lines-between-class-members": [
    2,
    "always",
    { exceptAfterSingleLine: true },
  ],
  "@typescript-eslint/method-signature-style": 2,
  "@typescript-eslint/no-redeclare": 2,
  "@typescript-eslint/no-shadow": 2,
  "@typescript-eslint/no-unnecessary-boolean-literal-compare": 2,
  "@typescript-eslint/no-unnecessary-condition": [
    2,
    { allowConstantLoopConditions: true },
  ],
  "@typescript-eslint/no-unnecessary-qualifier": 2,
  "@typescript-eslint/no-unnecessary-type-arguments": 2,
  "@typescript-eslint/no-unnecessary-type-assertion": 2,
  "@typescript-eslint/no-unnecessary-type-constraint": 2,
  "@typescript-eslint/no-unused-expressions": 2,
  "@typescript-eslint/no-unused-vars": 2,
  "@typescript-eslint/no-use-before-define": [
    2,
    {
      enums: true,
      ignoreTypeReferences: false,
      typedefs: true,
    },
  ],
  "@typescript-eslint/prefer-optional-chain": 2,
  "@typescript-eslint/prefer-readonly": 2,
};

const commonExtends = [
  "eslint:recommended",
  "next",
  "plugin:react/recommended",
];

const tsExtends = commonExtends.concat([
  "plugin:@typescript-eslint/recommended",
  "plugin:@typescript-eslint/recommended-requiring-type-checking",
]);

module.exports = {
  env: {
    browser: true,
    es6: true,
    jest: true,
    node: true,
  },
  extends: commonExtends,
  globals: {
    $: false,

    global: true,
  },
  overrides: [
    {
      files: ["**/scripts/**/*.{js,ts}", "./docs/strategies/**/*.js"],
      rules: {
        "no-console": 0,
      },
    },
    {
      files: ["*.js"],
      rules: {
        "@typescript-eslint/explicit-member-accessibility": 0,
        "@typescript-eslint/no-unused-vars": 0,
        "no-shadow": 2,
      },
    },
    {
      files: ["*.d.ts"],
      rules: {
        "@typescript-eslint/init-declarations": 2,
        "init-declarations": 0,
        "no-var": 0,
      },
    },
    {
      extends: tsExtends,
      files: ["*.ts", "*.tsx"],
      parserOptions: {
        project: ["./tsconfig.json"],
        tsconfigRootDir: __dirname,
      },
      rules: {
        ...disabledRules,
        ...tsRules,
      },
    },
  ],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaVersion: 9,
    sourceType: "module",
  },
  plugins: [
    "prettier",
    "import",
    "@stylistic",
    "@typescript-eslint",
    "perfectionist",
    "jest",
  ],
  rules: {
    ...disabledRules,

    "@stylistic/padding-line-between-statements": paddingRule,

    "arrow-body-style": [2, "as-needed"],
    "block-scoped-var": 2,
    "camelcase": [2, { properties: "never" }],
    "class-methods-use-this": 2,
    "consistent-return": 2,
    "eqeqeq": 2,

    "import/no-duplicates": "error",
    "import/no-namespace": [2, { ignore: ["*.module.css"] }],

    "init-declarations": [2, "always"],

    "jest/no-focused-tests": 2,
    "jest/no-identical-title": 2,

    "newline-before-return": 2,
    "no-console": 2,
    "no-constant-condition": [2, { checkLoops: false }],
    "no-else-return": [2, { allowElseIf: false }],
    "no-func-assign": 2,
    "no-multi-assign": 2,
    "no-nested-ternary": 2,
    "no-new-func": 2,
    "no-param-reassign": 2,
    "no-plusplus": 2,
    "no-restricted-syntax": [
      "error",
      "IfStatement[consequent.type!='BlockStatement']",
    ],
    "no-return-assign": [2, "always"],
    "no-shadow": 0, // using ts plugin one
    "no-unneeded-ternary": 2,
    "no-unreachable": 2,
    "no-unreachable-loop": 2,
    "no-useless-call": 2,
    "no-useless-computed-key": 2,
    "no-useless-concat": 2,
    "no-useless-rename": 2,
    "no-useless-return": 2,
    "no-var": 2,
    "object-shorthand": [2, "always"],
    "one-var": [2, "never"],
    "padding-line-between-statements": [
      2,
      {
        blankLine: "always",
        next: ["const", "let"],
        prev: "*",
      },
      {
        blankLine: "always",
        next: "*",
        prev: ["const", "let"],
      },
      {
        blankLine: "any",
        next: ["const", "let"],
        prev: ["const", "let"],
      },
      {
        blankLine: "always",
        next: "function",
        prev: "multiline-block-like",
      },
      {
        blankLine: "always",
        next: "multiline-block-like",
        prev: "multiline-block-like",
      },
      {
        blankLine: "always",
        next: "block-like",
        prev: "block-like",
      },
      {
        blankLine: "always",
        next: "class",
        prev: "*",
      },
      {
        blankLine: "always",
        next: "*",
        prev: "class",
      },
      {
        blankLine: "always",
        next: "*",
        prev: "multiline-block-like",
      },
      {
        blankLine: "always",
        next: "multiline-block-like",
        prev: "*",
      },
      {
        blankLine: "always",
        next: "default",
        prev: "*",
      },
      {
        blankLine: "always",
        next: "function",
        prev: "*",
      },
    ],

    "perfectionist/sort-array-includes": "error",
    "perfectionist/sort-astro-attributes": "error",
    "perfectionist/sort-classes": "error",
    "perfectionist/sort-enums": "error",
    "perfectionist/sort-exports": "error",
    "perfectionist/sort-interfaces": "error",
    "perfectionist/sort-jsx-props": "error",
    "perfectionist/sort-maps": "error",
    "perfectionist/sort-named-exports": "error",
    "perfectionist/sort-object-types": "error",
    "perfectionist/sort-objects": "error",
    "perfectionist/sort-svelte-attributes": "error",
    "perfectionist/sort-union-types": "error",
    "perfectionist/sort-vue-attributes": "error",

    "prefer-arrow-callback": 2,
    "prefer-const": 2,
    "prefer-destructuring": [
      2,
      {
        AssignmentExpression: {
          array: false,
          object: false,
        },
        VariableDeclarator: {
          array: false,
          object: true,
        },
      },
      { enforceForRenamedProperties: true },
    ],
    "prefer-rest-params": 2,
    "prefer-template": 2,

    "prettier/prettier": "error",

    "quote-props": [2, "consistent-as-needed"],

    "react/jsx-handler-names": 2,
    "react/jsx-sort-props": 2,
    "react/self-closing-comp": 2,
  },
  settings: {
    react: {
      version:
        webPackageJson.dependencies.react ||
        webPackageJson.devDependencies.react ||
        "detect",
    },
  },
};
