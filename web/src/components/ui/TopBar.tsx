import MenuIcon from "@mui/icons-material/Menu";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import IconButton from "@mui/material/IconButton";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";

import Button from "./Button";

export default function Dashboard() {
  const version = `v${process.env.NEXT_PUBLIC_VERSION}`;

  return (
    <AppBar component="nav">
      <Toolbar>
        <IconButton
          aria-label="Open drawer"
          color="inherit"
          edge="start"
          sx={{ display: { sm: "none" }, mr: 2 }}
        >
          <MenuIcon />
        </IconButton>
        <Typography className="flex-1" variant="h6">
          APISIX Dashboard
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
        <Box sx={{ display: { sm: "block", xs: "none" } }}>
          <span>{version}</span>
        </Box>
      </Toolbar>
    </AppBar>
  );
}
