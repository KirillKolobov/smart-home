import { Avatar, Box, Container, Paper, Typography } from "@mui/material";
import classes from "./styles.module.scss";
import { SignUpSteps } from "./Stepper";

import { SignUpForm } from "./SignUpForm";
import { useState } from "react";

export const SignUpPage = () => {
  const [activeStep, setActiveStep] = useState(0);

  const handleChangeStep = () =>
    activeStep === 0 ? setActiveStep(1) : setActiveStep(0);

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
