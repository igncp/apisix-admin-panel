import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";

import Button from "./Button";

export default function Dashboard() {
  const version = `v${process.env.NEXT_PUBLIC_VERSION}`;

  return (
    <AppBar component="nav">
      <Toolbar>
        <Typography className="flex-1" variant="h6">
          APISIX Admin Panel
        </Typography>
        <Box sx={{ display: { sm: "block", xs: "none" } }}>
          <Button
            onClick={() => {
              window.open(
                "https://github.com/igncp/apisix-admin-panel",
                "_blank",
              );
            }}
          >
            Docs
          </Button>
        </Box>
        <Box>
          <span>{version}</span>
        </Box>
      </Toolbar>
    </AppBar>
  );
}
