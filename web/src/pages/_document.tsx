import type { DocumentContext } from "next/document";
import { Html, Head, Main, NextScript } from "next/document";

import type { DocumentHeadTagsProps } from "@mui/material-nextjs/v13-pagesRouter";
import {
  DocumentHeadTags,
  documentGetInitialProps,
} from "@mui/material-nextjs/v13-pagesRouter";

const MyDocument = (props: DocumentHeadTagsProps) => (
  <Html lang="en">
    <Head>
      <DocumentHeadTags {...props} />
    </Head>
    <body>
      <Main />
      <NextScript />
    </body>
  </Html>
);

MyDocument.getInitialProps = async (ctx: DocumentContext) => {
  const finalProps = await documentGetInitialProps(ctx);

  return finalProps;
};

export default MyDocument;
