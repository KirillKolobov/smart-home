import classes from "./styles.module.scss";
import { Box, Button, Link } from "@mui/material";
import { InputText } from "../../../../components/InputText";
import { InputCheckbox } from "../../../../components/InputCheckbox";

type SecondStepProps = {
  handleChangeStep: () => void;
};

export const SecondStep = ({ handleChangeStep }: SecondStepProps) => {
  return (
    <>
      <InputText
        testId="password"
        name="password"
        label="Пароль"
        rules={{
          required: "This field is required",
          maxLength: { message: "Too many symbols", value: 20 },
        }}
        placeholder="Минимум 8 символов"
      />
      <InputText
        testId="repeatPassword"
        name="repeatPassword"
        label="Подтвердите пароль"
        rules={{
          required: "This field is required",
          maxLength: { message: "Too many symbols", value: 20 },
        }}
        placeholder="Повторите пароль"
      />
      <InputCheckbox
        name="acceptPrivacyPolicy"
        label={
          <>
            Я принимаю <Link>условия использования</Link> и{" "}
            <Link> политику конфиденциальности</Link>
          </>
        }
      />
      <Box className={classes.buttonsContainer}>
        <Button
          variant="contained"
          color="secondary"
          className={classes.backButton}
          onClick={handleChangeStep}
        >
          Назад
        </Button>
        <Button variant="contained">Создать аккаунт</Button>
      </Box>
    </>
  );
};
