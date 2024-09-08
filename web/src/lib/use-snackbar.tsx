import { useRef, useState } from "react";

export const useSnackbar = () => {
  const snackbarTimeout = useRef<null | number>(null);
  const [snackbarMessage, setSnackbarMessage] = useState<null | string>(null);
  const [snackbarOpen, setSnackbarOpen] = useState(false);

  const setSnackbar = (originalMessage: string) => {
    const message =
      typeof originalMessage === "string"
        ? originalMessage
        : String(originalMessage);

    setSnackbarMessage(message);
    setSnackbarOpen(true);

    if (snackbarTimeout.current) {
      clearTimeout(snackbarTimeout.current);
    }

    snackbarTimeout.current = window.setTimeout(() => {
      setSnackbarOpen(false);
      snackbarTimeout.current = null;
    }, 5000);
  };

  return {
    setSnackbar,
    setSnackbarOpen,
    snackbarMessage,
    snackbarOpen,
  };
};
