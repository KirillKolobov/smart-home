import { Avatar, Box, Container, Typography } from "@mui/material";
import classes from "./styles.module.scss";
import { SignInForm } from "./SignInForm";

export const SignInPage = () => {
  return (
    <Container className={classes.container}>
      <Box className={classes.content}>
        <Avatar className={classes.logo}>🏠</Avatar>
        <Box className={classes.titleContainer}>
          <Typography fontSize={30} lineHeight={1.2} variant="h1">
            Умный дом
          </Typography>
          <Typography fontSize={16} lineHeight={1.5} color="text.secondary">
            Войдите в свой аккаунт
          </Typography>
        </Box>
        <SignInForm />
      </Box>
    </Container>
  );
};
