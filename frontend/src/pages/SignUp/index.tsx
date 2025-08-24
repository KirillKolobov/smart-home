import { Avatar, Box, Container, Paper, Typography } from "@mui/material";
import classes from "./styles.module.scss";
import { SignUpSteps } from "./Stepper";

import { SignUpForm } from "./SignUpForm";
import { useState } from "react";
import { useMutation, type DefaultError } from "@tanstack/react-query";
import type { IFormData } from "./types";
import type { IUser } from "../../types";
import type { SubmitHandler } from "react-hook-form";

export const SignUpPage = () => {
  const [activeStep, setActiveStep] = useState(0);

  const handleChangeStep = () =>
    activeStep === 0 ? setActiveStep(1) : setActiveStep(0);

  const { mutate } = useMutation<IUser, DefaultError, IFormData>({
    mutationFn: async (arg) => {
      const { email, first_name, last_name, password, phone } = arg;
      const res = await fetch("http://localhost:3000/auth/signup", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          email,
          first_name,
          last_name,
          password,
          phone,
        }),
      });

      if (res.status === 200) {
        const data = await res.json();
        return data;
      }

      if (res.status === 401) {
        throw new Error("Unauthorized");
      }

      if (res.status === 404) {
        throw new Error("User not found");
      }

      if (res.status === 500) {
        throw new Error("Internal Server Error");
      }
    },
  });
  const onSubmit: SubmitHandler<IFormData> = (data) => mutate(data);

  return (
    <Container className={classes.container}>
      <Box className={classes.content}>
        <Avatar className={classes.logo}>🏠</Avatar>
        <Box className={classes.titleContainer}>
          <Typography fontSize={30} lineHeight={1.2} variant="h1">
            Создать аккаунт
          </Typography>
          <Typography fontSize={16} lineHeight={1.5} color="text.secondary">
            Присоединяйтесь к умному дому
          </Typography>
        </Box>
        <SignUpSteps activeStep={activeStep} />
        <SignUpForm
          activeStep={activeStep}
          handleChangeStep={handleChangeStep}
          onSubmit={onSubmit}
        />
        <Paper className={classes.securityInfo} elevation={0}>
          <Box className={classes.securityContainer}>
            <Box className={classes.securityIcon}>🔒</Box>
            <Box className={classes.securityText}>
              <Typography className={classes.securityTitle} fontSize={16}>
                Безопасность данных
              </Typography>
              <Typography
                className={classes.securityDescription}
                fontSize={14}
                color="text.secondary"
              >
                Ваши данные защищены 256-битным SSL-шифрованием и хранятся в
                соответствии с GDPR.
              </Typography>
            </Box>
          </Box>
        </Paper>
      </Box>
    </Container>
  );
};
