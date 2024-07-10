import React from "react";
import "./Header.scss";
import { useTheme } from "@mui/material";
import Actions from "../Actions/Actions";

const Header = () => {
  const theme = useTheme();
  return (
    <div id={"header"} style={{ ...theme.text.header, ...theme.pane.primary }}>
      NED
      <Actions />
    </div>
  );
};

export default Header;
