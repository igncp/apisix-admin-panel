import type { AppProps } from "next/app";
import React from "react";
import "src/styles/globals.css";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { StylesProvider } from "../components/ui/StylesProvider";

const queryClient = new QueryClient();

const MyApp = (props: AppProps) => (
  <StylesProvider props={props}>
    <QueryClientProvider client={queryClient}>
      <props.Component {...props.pageProps} />
    </QueryClientProvider>
  </StylesProvider>
);

export default MyApp;
