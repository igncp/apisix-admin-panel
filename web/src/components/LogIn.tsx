import Head from "next/head";
import { useRouter } from "next/router";
import { useState } from "react";
import { login } from "src/lib/client";
import { useSnackbar } from "src/lib/use-snackbar";

import Snackbar from "@mui/material/Snackbar";

import Layout from "./Layout";
import Button from "./ui/Button";
import { Input } from "./ui/Input";

const labelClasses = "flex flex-col gap-[8px]";

const LogIn = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const router = useRouter();

  const { setSnackbar, setSnackbarOpen, snackbarMessage, snackbarOpen } =
    useSnackbar();

  return (
    <>
      <Head>
        <title>APISIX Admin Panel - Login</title>
      </Head>
      <Layout>
        <div className="flex flex-col items-center justify-center">
          <h2>Log in to the panel</h2>
          <form
            className="flex max-w-[300px] flex-col gap-[24px]"
            onSubmit={(e) => {
              e.preventDefault();

              login(username, password).then((isLoggedIn) => {
                if (isLoggedIn) {
                  router.replace("/");
                } else {
                  setSnackbar("Invalid username or password");
                }
              });
            }}
          >
            <label className={labelClasses}>
              <span>Username:</span>
              <Input
                name="username"
                onChange={(e) => setUsername(e.target.value)}
                value={username}
              />
            </label>
            <label className={labelClasses}>
              <span>Password:</span>
              <Input
                name="password"
                onChange={(e) => setPassword(e.target.value)}
                type="password"
                value={password}
              />
            </label>
            <Button type="submit">Log In</Button>
          </form>
        </div>
        <Snackbar
          anchorOrigin={{ horizontal: "center", vertical: "top" }}
          key="top"
          message={snackbarMessage}
          onClose={() => setSnackbarOpen(false)}
          open={snackbarOpen}
        />
      </Layout>
    </>
  );
};

export default LogIn;
