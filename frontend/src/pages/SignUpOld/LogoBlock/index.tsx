import { Typography } from "@mui/material";
import { Logo } from "./Logo";
import classes from "./styles.module.scss";

export const LogoBlock = () => (
  <div className={classes.logoContainer}>
    <Logo />
    <div className={classes.logoText}>
      <Typography variant="h1" color="info" className={classes.title}>
        Создать аккаунт
      </Typography>
      <Typography variant="p1" color="neutralInfo" className={classes.title}>
        Присоединяйтесь к умному дому
      </Typography>
    </div>
  </div>
);
