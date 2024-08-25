import { Roboto } from "next/font/google";
import type { PropsWithChildren } from "react";

import { AppCacheProvider } from "@mui/material-nextjs/v13-pagesRouter";
import CssBaseline from "@mui/material/CssBaseline";
import { ThemeProvider, createTheme } from "@mui/material/styles";

const roboto = Roboto({
  display: "swap",
  subsets: ["latin"],
  weight: ["300", "400", "500", "700"],
});

const theme = createTheme({
  palette: {
    mode: "dark",
  },
  typography: {
    fontFamily: roboto.style.fontFamily,
  },
});

export const StylesProvider = ({
  children,
  props,
}: PropsWithChildren<{ props: Record<string, unknown> }>) => (
  <AppCacheProvider {...props}>
    <ThemeProvider theme={theme}>
      <CssBaseline />
      {children}
    </ThemeProvider>
  </AppCacheProvider>
);
